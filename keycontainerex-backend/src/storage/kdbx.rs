use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

use dirs;
use keepass::{Database, DatabaseKey};
use rpassword::read_password;

use crate::util::box_error;

fn get_kdbx_file() -> Result<PathBuf, Box<dyn Error>> {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push("KeyContainerEx");
            path.push("default.kdbx");
            Ok(path)
        }
        _ => box_error!("failed to get kdbx path"),
    }
}

pub fn init_kdbx(path: Option<&String>, force: bool) -> Result<(), Box<dyn Error>> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(&path),
        None => get_kdbx_file()?,
    };
    if kdbx_path.exists() {
        if !force {
            return box_error!("file already exists");
        }
        if fs::metadata(&kdbx_path)?.is_dir() {
            fs::remove_dir(&kdbx_path)?;
        } else {
            fs::remove_file(&kdbx_path)?;
        }
    }

    print!("Password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    if password.is_empty() {
        return box_error!("empty password");
    }
    print!("Confirm password: ");
    io::stdout().flush().unwrap();
    let password_confirm = read_password().unwrap();
    if password != password_confirm {
        return box_error!("password confirm not pass");
    }
    let db = Database::new(Default::default());
    db.save(
        &mut OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&kdbx_path)
            .unwrap(),
        DatabaseKey::with_password(&password),
    )?;

    Ok(())
}

pub fn open_kdbx(path: PathBuf, password: &str) -> Result<Database, Box<dyn Error>> {
    Ok(Database::open(
        &mut File::open(path)?,
        DatabaseKey::with_password(password),
    )?)
}
