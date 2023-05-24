use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::string::ToString;

use dirs;
use serde::{Deserialize, Serialize};

use crate::box_error;

#[derive(Deserialize, Serialize)]
struct Config {
    kdbx_path: String,
}

fn get_config_file() -> Result<PathBuf, Box<dyn Error>> {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push("KeyContainerEx");
            path.push("config.toml");
            Ok(path)
        }
        _ => box_error!("failed to get config path"),
    }
}

pub fn init_config(path: Option<&String>) -> Result<(), Box<dyn Error>> {
    let config_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_config_file()?,
    };
    if config_path.exists() && fs::metadata(&config_path)?.is_dir() {
        fs::remove_dir(&config_path)?;
    }
    let config = Config {
        kdbx_path: config_path.to_str().unwrap().to_string(),
    };
    fs::write(config_path, toml::to_string(&config).unwrap())?;
    Ok(())
}
