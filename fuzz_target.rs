#![feature(read_buf)]
#![feature(core_io_borrowed_buf)]
#![no_main]
#[macro_use] extern crate libfuzzer_sys;

/// rustc_driver::Callbacks object that stops before codegen.
pub struct FuzzCallbacks {
    input: Vec<u8>,
}

impl FuzzCallbacks {
    pub fn new(input: Vec<u8>) -> FuzzCallbacks {
        Self {
            input
        }
    }
}

impl rustc_driver::Callbacks for FuzzCallbacks {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.file_loader = Some(Box::new(FuzzFileLoader::new(self.input.clone())));
        config.make_codegen_backend =
            Some(Box::new(|_| {Box::new(NullCodegenBackend)}));
    }

    fn after_analysis<'tcx>(&mut self,
                            _compiler: &rustc_interface::interface::Compiler,
                            _tcx: rustc_middle::ty::TyCtxt<'_>) -> rustc_driver::Compilation {
        // Stop before codegen.
        rustc_driver::Compilation::Stop
    }
}

/// FileLoader that holds the contents of a single input file in memory.
/// The idea here is to avoid needing to write to disk.
struct FuzzFileLoader {
    // The contents of the single input file.
    input: Vec<u8>,
}

impl FuzzFileLoader {
    fn new(input: Vec<u8>) -> Self {
        FuzzFileLoader {
            input,
        }
    }
}

// The name of the single input file.
const INPUT_PATH: &str = "fuzz_input.rs";

impl rustc_span::source_map::FileLoader for FuzzFileLoader {
    fn file_exists(&self, path: &std::path::Path) -> bool {
        std::path::Path::new(INPUT_PATH) == path
    }

    fn read_file(&self, path: &std::path::Path) -> std::io::Result<String> {
        if self.file_exists(path) {
            let s = match String::from_utf8(self.input.to_vec()) {
                Ok(s) => s,
                Err(_e) => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                          "not utf8")),
            };
            Ok(s)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "tried to open nonexistent file".to_string()))
        }
    }

    fn read_binary_file(&self, path: &std::path::Path) -> std::io::Result<std::sync::Arc<[u8]>> {
        use std::io::Read;
        if self.file_exists(path) {
            let len = self.input.len();
            let mut bytes = std::sync::Arc::new_uninit_slice(len as usize);
            let mut buf = std::io::BorrowedBuf::from(std::sync::Arc::get_mut(&mut bytes).unwrap());
            let mut file = std::io::Cursor::new(&self.input[..]);
            file.read_buf_exact(buf.unfilled())?;
            Ok(unsafe { bytes.assume_init() })
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "tried to open nonexistent file".to_string()))
        }
    }
}

/// CodegenBackend that panics when being called to do any actual codegen.
/// We use this to avoid needing to compile rustc_codegen_llvm.
pub struct NullCodegenBackend;

impl rustc_codegen_ssa::traits::CodegenBackend for NullCodegenBackend {
    fn codegen_crate<'tcx>(&self,
                           _: rustc_middle::ty::TyCtxt<'tcx>,
                           _: rustc_metadata::EncodedMetadata,
                           _: bool) -> std::boxed::Box<(dyn core::any::Any + 'static)> {
        unimplemented!()
    }

    fn join_codegen(
        &self,
        _ongoing_codegen: Box<dyn core::any::Any>,
        _sess: &rustc_session::Session,
        _outputs: &rustc_session::config::OutputFilenames,
    ) ->(rustc_codegen_ssa::CodegenResults,
         rustc_data_structures::fx::FxIndexMap<rustc_middle::dep_graph::WorkProductId,
                                               rustc_middle::dep_graph::WorkProduct>)
    {
        unimplemented!()
    }

    fn link(
        &self,
        _sess: &rustc_session::Session,
        _codegen_results: rustc_codegen_ssa::CodegenResults,
        _outputs: &rustc_session::config::OutputFilenames,
    ) {
        unimplemented!()
    }

    fn locale_resource(&self) -> &'static str {
        ""
    }
}


pub fn main_fuzz(input: Vec<u8>) {
    let mut callbacks = FuzzCallbacks::new(input);
    let args = &["rustc".to_string(),
                 INPUT_PATH.to_string(),
                 "-o".to_string(),
                 "dummy_output_file".to_string(),
                 "--edition".to_string(),
                 "2018".to_string(),
                 "-L".to_string(),
                 env!("FUZZ_RUSTC_LIBRARY_DIR").to_string()];
    let _ = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(args, &mut callbacks);
    }).and_then(|result| Ok(result));
}

fuzz_target!(|data: &[u8]| {
    if data.contains(&0x0c) || data.contains(&0x0d) || data.contains(&0x0b) /*|| data.contains (&b'&')*/ {
        return;
    }
//    if let Ok(t) = std::str::from_utf8(data) {
//        if let Some(_) = t.find("derive") {
//            return;
//        }
        main_fuzz(data.into());
//    }
});
