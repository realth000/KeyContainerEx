use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use dirs;
use keepass::db::{Group, Node};
use keepass::{Database, DatabaseKey};

use crate::box_error;
use crate::util::read_password;

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
    } else if kdbx_path.parent().is_some() {
        let folder = kdbx_path.parent().unwrap();
        if !folder.exists() {
            fs::create_dir_all(folder)?;
        }
    }

    let password = read_password("Password: ").unwrap();
    if password.is_empty() {
        return box_error!("empty password");
    }
    let password_confirm = read_password("Confirm password: ").unwrap();
    if password != password_confirm {
        return box_error!("password confirm not pass");
    }
    let db = Database::new(Default::default());
    db.save(
        &mut OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&kdbx_path)?,
        DatabaseKey::with_password(&password),
    )?;

    Ok(())
}

pub fn open_kdbx(path: Option<&String>, password: &str) -> Result<Database, Box<dyn Error>> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    Ok(Database::open(
        &mut File::open(kdbx_path)?,
        DatabaseKey::with_password(password),
    )?)
}

pub fn add_kdbx_group(
    path: Option<&String>,
    password: &str,
    group_name: &str,
) -> Result<(), Box<dyn Error>> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    let mut database = Database::open(
        &mut File::open(&kdbx_path)?,
        DatabaseKey::with_password(password),
    )?;
    let group = Group::new(&group_name);
    database.root.children.push(Node::Group(group));
    database.save(
        &mut OpenOptions::new().write(true).open(&kdbx_path)?,
        DatabaseKey::with_password(password),
    )?;
    Ok(())
}
