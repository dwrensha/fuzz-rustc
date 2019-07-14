#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate rustc_driver;

fn try_main(data: &[u8]) -> Result<(), ::std::io::Error> {
    let mut file = ::tempfile::Builder::new().prefix("fuzz").tempfile()?;
    ::std::io::Write::write_all(&mut file, data)?;
    ::std::io::Write::flush(&mut file)?;
    ::rustc_driver::main_fuzz(&file.path().to_str().expect("path string"), "/tmp/dummy_output_file");
    file.close()?; // removes the file
    Ok(())
}


fuzz_target!(|data: &[u8]| {
    let _ = try_main(data);

});
