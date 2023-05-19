use std::error::Error;

use crate::storage::config::init_config;
use crate::storage::kdbx::init_kdbx;

mod config;
mod kdbx;

pub fn init(path: Option<&String>, force: bool) -> Result<(), Box<dyn Error>> {
    init_kdbx(path, force)?;
    init_config(None)?;
    Ok(())
}
