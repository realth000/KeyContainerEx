use std::error::Error;
use std::fs;
use std::path::PathBuf;

use dirs;

fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
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

pub fn init(path: Option<&String>) -> Result<(), Box<dyn Error>> {
    let storage_path;
    match path {
        Some(path) => storage_path = PathBuf::from(&path),
        None => storage_path = get_config_dir()?,
    }
    if !storage_path.is_dir() {
        fs::remove_file(&storage_path)?;
    }
    if !storage_path.exists() {
        fs::create_dir(&storage_path)?;
    }
    Ok(())
}

