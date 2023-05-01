use std::error::Error;
use std::path::PathBuf;

use dirs;

pub mod storage;

pub fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push("KeyContainerEx");
            Ok(path)
        }
        _ => {
            Err("failed to get config directory".into())
        }
    }
}

pub fn init() -> Result<(), Box<dyn Error>> {
    let config = match get_config_dir() {
        Ok(path) => path,
        Err(e) => {
            return Err(e);
        }
    };

    let config_dir = config.as_path().read_dir();

    Ok(())
}
