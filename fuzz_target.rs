#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate rustc_driver;

fn try_main(data: &[u8]) -> Result<(), ::std::io::Error> {
    let basename = format!("fuzz_at_{:?}", ::std::time::SystemTime::now()
                           .duration_since(::std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis());
    let sourcefile = format!("/tmp/{}.rs", &basename);
    let outputfile = format!("/tmp/{}", basename);
    {
        let mut file = ::std::fs::File::create(&sourcefile)?;
        ::std::io::Write::write_all(&mut file, data)?;
    }
    ::rustc_driver::main_fuzz(&sourcefile, &outputfile);
    Ok(())
}


fuzz_target!(|data: &[u8]| {
    match ::std::str::from_utf8(data) {
        Ok(s) => {
            if s.is_ascii() {
                let _ = try_main(data);
            }
        }
        _ => ()
    }
});
