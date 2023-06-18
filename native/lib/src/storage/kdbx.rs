use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

use dirs;
use keepass::db::{Entry, Group, Node, NodeRefMut, Value};
use keepass::{Database, DatabaseKey};

use crate::box_error;

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

pub fn init_kdbx(path: Option<&String>, key: &str, force: bool) -> Result<(), Box<dyn Error>> {
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

    let db = Database::new(Default::default());
    db.save(
        &mut OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&kdbx_path)?,
        DatabaseKey::with_password(key),
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
    key: &str,
    group_name: &str,
) -> Result<(), Box<dyn Error>> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    let mut database = Database::open(
        &mut File::open(&kdbx_path)?,
        DatabaseKey::with_password(key),
    )?;
    let group = Group::new(group_name);
    for node in &database.root.children {
        if let Node::Group(g) = node {
            if g.name == group_name {
                return box_error!("group already exists");
            }
        }
    }
    database.root.children.push(Node::Group(group));
    database.save(
        &mut OpenOptions::new().write(true).open(&kdbx_path)?,
        DatabaseKey::with_password(key),
    )?;
    Ok(())
}

pub fn remove_kdbx_group(
    path: Option<&String>,
    key: &str,
    group_name: &str,
) -> Result<(), Box<dyn Error>> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    let mut database = Database::open(
        &mut File::open(&kdbx_path)?,
        DatabaseKey::with_password(key),
    )?;
    // TODO: Implement group delete.

    Ok(())
}

pub fn add_kdbx_entry(
    path: Option<&String>,
    key: &str,
    group: &str,
    title: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    let mut database = Database::open(
        &mut File::open(&kdbx_path)?,
        DatabaseKey::with_password(key),
    )?;
    for node in &database.root.children {
        if let Node::Group(g) = node {
            if g.name == group {
                for n in &g.children {
                    if let Node::Entry(e) = n {
                        if e.get_title().unwrap() == title {
                            return box_error!("title already exists in the same group");
                        }
                    }
                }
            }
        }
    }
    let mut entry = Entry::new();
    entry
        .fields
        .insert("Title".to_string(), Value::Unprotected(title.to_string()));
    entry.fields.insert(
        "UserName".to_string(),
        Value::Unprotected(username.to_string()),
    );
    entry.fields.insert(
        "Password".to_string(),
        Value::Protected(password.as_bytes().into()),
    );

    match database.root.get_mut(&[group]) {
        Some(NodeRefMut::Group(g)) => {
            g.children.push(Node::Entry(entry));
        }
        Some(NodeRefMut::Entry(_)) => {
            return box_error!("failed to add password: \"{}\" is an entry", &group);
        }
        None => {
            return box_error!("failed to add password: group \"{}\" not found", &group);
        }
    }

    database.save(
        &mut OpenOptions::new().write(true).open(&kdbx_path)?,
        DatabaseKey::with_password(key),
    )?;
    Ok(())
}

pub fn remove_kdbx_entry(
    path: Option<&String>,
    key: &str,
    group: &str,
    title: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
