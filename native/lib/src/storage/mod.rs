use std::error::Error;

use kdbx::init_kdbx;

pub mod kdbx;

pub fn init(path: Option<&String>, key: &str, force: bool) -> Result<(), Box<dyn Error>> {
    init_kdbx(path, key, force)?;
    Ok(())
}
