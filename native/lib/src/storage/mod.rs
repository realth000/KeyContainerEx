use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf};

use kdbx::init_kdbx;

use crate::box_error;
use crate::storage::kdbx::{
    add_kdbx_entry, add_kdbx_group, get_default_kdbx_path, open_kdbx, show_kdbx,
};

mod kdbx;

pub enum StorageFormat {
    Kdbx4,
    Json,
}

#[derive(Default)]
pub struct StorageGroup {
    pub title: String,
    pub sub_group: Vec<StorageGroup>,
    pub sub_password: Vec<StoragePassword>,
}

impl Debug for StorageGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageGroup")
            .field("title", &format_args!("{}", self.title))
            .field("sub_group", &format_args!("{:#?}", self.sub_group))
            .field("sub_password", &format_args!("{:#?}", self.sub_password))
            .finish()
    }
}

impl StorageGroup {
    pub fn new(title: String) -> Self {
        StorageGroup {
            title,
            sub_group: vec![],
            sub_password: vec![],
        }
    }

    pub fn add_sub_group(&mut self, group: StorageGroup) {
        self.sub_group.push(group);
    }

    pub fn add_sub_groups(&mut self, group: Vec<StorageGroup>) {
        self.sub_group.extend(group);
    }

    pub fn remove_sub_group(&mut self, title: &str) -> bool {
        match self.sub_group.iter().position(|x| x.title == title) {
            Some(v) => {
                self.sub_group.remove(v);
                true
            }
            None => false,
        }
    }

    pub fn add_sub_password(&mut self, password: StoragePassword) {
        self.sub_password.push(password);
    }

    pub fn add_sub_passwords(&mut self, password: Vec<StoragePassword>) {
        self.sub_password.extend(password);
    }

    pub fn remove_sub_password(&mut self, password: &StoragePassword) -> bool {
        match self
            .sub_password
            .iter()
            .position(|x| x.title == password.title && x.username == password.username)
        {
            Some(v) => {
                self.sub_password.remove(v);
                true
            }
            None => false,
        }
    }

    pub fn remove_sub_password_recursive(&mut self, password: &StoragePassword) -> bool {
        match self
            .sub_password
            .iter()
            .position(|x| x.title == password.title && x.username == password.username)
        {
            Some(v) => {
                self.sub_password.remove(v);
                true
            }
            None => {
                for group in &mut self.sub_group {
                    match group.remove_sub_password_recursive(password) {
                        true => {
                            return true;
                        }
                        false => continue,
                    }
                }
                return false;
            }
        }
    }
}

#[derive(Default)]
pub struct StoragePassword {
    pub title: String,
    pub username: String,
    pub password: String,
}

impl Debug for StoragePassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoragePassword")
            .field("title", &format_args!("{}", self.title))
            .field("username", &format_args!("{}", self.username))
            .field("password", &format_args!("{}", self.password))
            .finish()
    }
}

impl StoragePassword {
    pub fn new(title: String, username: String, password: String) -> Self {
        StoragePassword {
            title,
            username,
            password,
        }
    }
}

pub fn default_save_path(storage_format: StorageFormat) -> Result<PathBuf, Box<dyn Error>> {
    match storage_format {
        StorageFormat::Kdbx4 => get_default_kdbx_path(),
        StorageFormat::Json => {
            box_error!("Json format not implemented yet")
        }
    }
}

pub fn check_init(
    storage_format: StorageFormat,
    path: Option<&String>,
) -> Result<bool, Box<dyn Error>> {
    match path {
        Some(v) => Ok(Path::new(v).exists()),
        None => Ok(default_save_path(storage_format)?.exists()),
    }
}

pub fn init(
    storage_format: StorageFormat,
    path: Option<&String>,
    master_key: &str,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    match storage_format {
        StorageFormat::Kdbx4 => init_kdbx(path, master_key, force),
        StorageFormat::Json => {
            box_error!("Json format not implemented yet")
        }
    }
}

pub fn show(
    storage_format: StorageFormat,
    path: Option<&String>,
    master_key: &str,
) -> Result<StorageGroup, Box<dyn Error>> {
    match storage_format {
        StorageFormat::Kdbx4 => {
            let database = open_kdbx(path, master_key)?;
            show_kdbx(database)
        }
        StorageFormat::Json => {
            box_error!("Json format not implemented yet")
        }
    }
}

pub fn add_group(
    storage_format: StorageFormat,
    path: Option<&String>,
    master_key: &str,
    group: &str,
) -> Result<(), Box<dyn Error>> {
    match storage_format {
        StorageFormat::Kdbx4 => add_kdbx_group(path, master_key, group),
        StorageFormat::Json => {
            box_error!("Json format not implemented yet")
        }
    }
}

pub fn add_password(
    storage_format: StorageFormat,
    path: Option<&String>,
    master_key: &str,
    group: &str,
    title: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    match storage_format {
        StorageFormat::Kdbx4 => add_kdbx_entry(path, master_key, group, title, username, password),
        StorageFormat::Json => {
            box_error!("Json format not implemented yet")
        }
    }
}
