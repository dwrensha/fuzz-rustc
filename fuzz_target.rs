#![no_main]
#[macro_use] extern crate libfuzzer_sys;

pub struct FuzzCallbacks;

impl rustc_driver::Callbacks for FuzzCallbacks {
    fn after_analysis<'tcx>(&mut self,
                            _compiler: &rustc_interface::interface::Compiler,
                            _queries: &'tcx rustc_interface::Queries<'tcx>,) -> rustc_driver::Compilation {
        rustc_driver::Compilation::Stop
    }
}



struct FuzzFileLoader {
    input: String,
}

impl FuzzFileLoader {
    fn new(input: String) -> Self {
        FuzzFileLoader {
            input,
        }
    }
}

impl rustc_span::source_map::FileLoader for FuzzFileLoader {
    fn file_exists(&self, _path: &std::path::Path) -> bool { true }
    fn abs_path(&self, _path: &std::path::Path) -> Option<std::path::PathBuf> { None }
    fn read_file(&self, _path: &std::path::Path) -> std::io::Result<String> { Ok(self.input.clone()) }
}


pub fn main_fuzz(input: String, output_filename: &str) {
    let file_loader = Box::new(FuzzFileLoader::new(input));
    //init_rustc_env_logger();
    let mut callbacks = FuzzCallbacks;
    let _result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&["rustc".to_string(), "fuzz_input.rs".to_string(),
                       "-o".to_string(),
                       output_filename.to_string(),
                       "--edition".to_string(),
                       "2018".to_string(),
                       "-L".to_string(),
                       env!("FUZZ_RUSTC_LIBRARY_DIR").to_string()],
                     &mut callbacks,
                     Some(file_loader),
                     None)
    }).and_then(|result| result);
}

fuzz_target!(|data: &[u8]| {
    if data.contains(&0x0c) || data.contains(&0x0d) || data.contains(&0x0b) {
        return;
    }
    if let Ok(t) = String::from_utf8(data.into()) {
        if let Some(_) = t.find("derive") {
            return;
        }
        main_fuzz(t, "/tmp/dummy_output_file");
    }
});
