use std::any::Any;
use std::fs;
use std::path::PathBuf;
use std::string::ToString;

use anyhow::Result;
use anyhow::{bail, Context};
use clap::{Arg, ArgAction};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::arg_ex;
use crate::util::type_of;

type ArgExSetFn = fn(&mut Config, &ArgExType) -> Result<()>;

pub enum ArgExType {
    Bool(bool),
    String(String),
}

pub struct ArgEx {
    pub arg_name: String,
    pub arg_value: Arg,
    pub arg_type: ArgExType,
    pub arg_set: ArgExSetFn,
}

#[macro_export]
macro_rules! arg_ex {
    ($name: literal, $arg_type: expr, $help: expr, $action: expr, $set_func: expr) => {
        match $arg_type {
            ArgExType::Bool(_) => ArgEx {
                arg_name: String::from($name),
                arg_value: Arg::new($name)
                    .long($name)
                    .help($help)
                    .action($action)
                    .value_parser(clap::builder::BoolishValueParser::new()),
                arg_type: $arg_type,
                arg_set: $set_func,
            },
            _ => ArgEx {
                arg_name: String::from($name),
                arg_value: Arg::new($name).long($name).help($help).action($action),
                arg_type: $arg_type,
                arg_set: $set_func,
            },
        }
    };
}

// TODO: Optimize these similar macros.
macro_rules! config_set_bool_func {
    ($($config_path: ident).+) => {
        |config, value| -> Result<()> {
            if let ArgExType::Bool(v) = value {
                config.$($config_path).+ = Some(*v);
                Ok(())
            } else {
                bail!(
                    "failed to set bool value config, expected bool, got {}",
                    type_of(&value)
                )
            }
        }
    };
}

macro_rules! config_set_string_func {
    ($($config_path: ident).+) => {
        |config, value| -> Result<()> {
            if let ArgExType::String(v) = value {
                config.$($config_path).+ = Some((&v).to_string());
                Ok(())
            } else {
                bail!(
                    "failed to set string value config, expected bool, got {}",
                    type_of(&value)
                )
            }
        }
    };
}

lazy_static! {
    static ref CONFIG_ARGS: Vec<ArgEx> = vec![
        arg_ex!(
            "allowDuplicateTitle",
            ArgExType::Bool(false),
            "Allow duplicate title in a password group, default=false",
            ArgAction::Set,
            config_set_bool_func!(storage.allow_duplicate_title)
        ),
        arg_ex!(
            "allowDuplicateGroup",
            ArgExType::Bool(false),
            "Allow duplicate group name in database, default=false",
            ArgAction::Set,
            config_set_bool_func!(storage.allow_duplicate_group)
        ),
        arg_ex!(
            "databasePath",
            ArgExType::String("".to_string()),
            "Database file path, default is ~/.config",
            ArgAction::Set,
            config_set_string_func!(kdbx_path)
        ),
    ];
}

pub fn get_config_vec() -> &'static Vec<ArgEx> {
    &CONFIG_ARGS
}

#[derive(Deserialize, Serialize)]
struct Storage {
    allow_duplicate_title: Option<bool>,
    allow_duplicate_group: Option<bool>,
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            allow_duplicate_title: Some(false),
            allow_duplicate_group: Some(false),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    kdbx_path: Option<String>,
    storage: Storage,
}

fn get_config_file() -> Result<PathBuf> {
    let mut path = dirs::config_dir().context("failed to get config path")?;
    path.push("KeyContainerEx");
    path.push("config.toml");
    Ok(path)
}

pub fn init_config(path: Option<&String>) -> Result<()> {
    let config_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_config_file()?,
    };
    if config_path.exists() && fs::metadata(&config_path)?.is_dir() {
        fs::remove_dir(&config_path)?;
    }
    let config = Config {
        kdbx_path: Some(config_path.to_str().unwrap().to_string()),
        storage: Default::default(),
    };
    fs::write(config_path, toml::to_string(&config).unwrap())?;
    Ok(())
}

pub fn update_config(path: Option<&String>, name: &str, value: ArgExType) -> Result<()> {
    let config_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_config_file()?,
    };
    let mut config: Config = toml::from_str(fs::read_to_string(&config_path)?.as_str())?;
    let mut found = false;
    for arg_ex in CONFIG_ARGS.iter() {
        if arg_ex.arg_name == name {
            found = true;
            if arg_ex.arg_type.type_id() != value.type_id() {
                bail!(
                    "invalid config value type for {}: expected {}, got {}",
                    &name,
                    type_of(&arg_ex.arg_type),
                    type_of(&value),
                )
            }
            (arg_ex.arg_set)(&mut config, &value)?;
            break;
        }
    }
    if !found {
        bail!("config not found: {}", name)
    }
    fs::write(config_path, toml::to_string(&config).unwrap())?;
    Ok(())
}
