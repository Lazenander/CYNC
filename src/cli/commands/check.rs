use crate::pipeline::driver::driver::CompilerDriver;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn execute(path_buf: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut driver = CompilerDriver::new(path_buf);
    driver.pipeline();
    driver.log_errors();
    Ok(())
}
