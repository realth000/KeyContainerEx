use std::any::Any;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::string::ToString;

use clap::{Arg, ArgAction};
use dirs;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::arg_ex;
use crate::box_error;
use crate::util::{type_of, ArgEx, ArgExType};

lazy_static! {
    static ref CONFIG_ARGS: Vec<ArgEx> = vec![
        arg_ex!(
            "allowDuplicateTitle",
            ArgExType::Bool(false),
            "Allow duplicate title in a password group, default=false",
            ArgAction::Set
        ),
        arg_ex!(
            "databasePath",
            ArgExType::String("".to_string()),
            "Database file path, default is ~/.config",
            ArgAction::Set
        ),
    ];
}

pub fn get_config_vec() -> &'static Vec<ArgEx> {
    &CONFIG_ARGS
}

#[derive(Deserialize, Serialize)]
struct Storage {
    allow_duplicate_title: bool,
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            allow_duplicate_title: false,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Config {
    kdbx_path: String,
    storage: Storage,
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
        storage: Default::default(),
    };
    fs::write(config_path, toml::to_string(&config).unwrap())?;
    Ok(())
}

pub fn update_config(
    path: Option<&String>,
    name: &str,
    value: ArgExType,
) -> Result<(), Box<dyn Error>> {
    let config_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_config_file()?,
    };
    let mut config: Config = toml::from_str(fs::read_to_string(&config_path)?.as_str())?;
    let mut found = false;
    // TODO: Need improve.
    for arg_ex in CONFIG_ARGS.iter() {
        if arg_ex.arg_name == name {
            found = true;
            if arg_ex.arg_type.type_id() != value.type_id() {
                return box_error!(
                    "invalid config value type for {}: expected {}, got {}",
                    "allowDuplicate",
                    type_of(&arg_ex.arg_type),
                    type_of(&value),
                )?;
            }
            match &value {
                ArgExType::Bool(v) => {
                    break;
                }
                ArgExType::String(v) => {
                    break;
                }
                _ => (),
            }
        }
    }
    if !found {
        return box_error!("config not found: {}", name);
    }
    fs::write(config_path, toml::to_string(&config).unwrap())?;
    Ok(())
}
