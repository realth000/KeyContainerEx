use std::error::Error;

use config::init_config;
pub use config::{get_config_vec, update_config};
use kdbx::init_kdbx;
pub use kdbx::{add_kdbx_entry, add_kdbx_group, open_kdbx};

mod config;
mod kdbx;

pub fn init(path: Option<&String>, force: bool) -> Result<(), Box<dyn Error>> {
    init_kdbx(path, force)?;
    init_config(None)?;
    Ok(())
}
