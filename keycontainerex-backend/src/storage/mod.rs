use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use dirs;
use rpassword::read_password;

use crate::storage::kdbx::init_kdbx;

mod kdbx;

fn get_config_file() -> Result<PathBuf, Box<dyn Error>> {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push("KeyContainerEx");
            path.push("default.kdbx");
            Ok(path)
        }
        _ => Err("failed to get config directory".into()),
    }
}

pub fn init(path: Option<&String>, force: bool) -> Result<(), Box<dyn Error>> {
    let storage_path = match path {
        Some(path) => PathBuf::from(&path),
        None => get_config_file()?,
    };
    if storage_path.exists() {
        if !force {
            return Err("file already exists".into());
        }
        if fs::metadata(&storage_path)?.is_dir() {
            fs::remove_dir(&storage_path)?;
        } else {
            fs::remove_file(&storage_path)?;
        }
    }
    fs::File::create(&storage_path)?;

    // TODO: Init kdbx here.
    print!("Password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    if password.is_empty() {
        return Err("empty password".into());
    }
    print!("Confirm: ");
    io::stdout().flush().unwrap();
    let password_confirm = read_password().unwrap();
    if password != password_confirm {
        return Err("password confirm not pass".into());
    }

    init_kdbx(storage_path, password.as_str())?;
    Ok(())
}
