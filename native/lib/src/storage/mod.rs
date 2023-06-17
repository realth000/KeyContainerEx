use std::error::Error;

use config::init_config;
use kdbx::init_kdbx;

pub mod config;
pub mod kdbx;

pub fn init(path: Option<&String>, force: bool) -> Result<(), Box<dyn Error>> {
    init_kdbx(path, force)?;
    init_config(None)?;
    Ok(())
}
