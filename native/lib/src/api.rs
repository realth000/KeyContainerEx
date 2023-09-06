use anyhow::Result;

use crate::storage::{
    add_group, add_password, check_init, default_save_path, init, show, StorageFormat, StorageGroup,
};

pub fn storage_default_save_path(storage_format: StorageFormat) -> Result<String> {
    default_save_path(storage_format).map(|v| v.to_str().unwrap().to_string())
}

pub fn storage_check_init(storage_format: StorageFormat, path: String) -> Result<bool> {
    if path.is_empty() {
        check_init(storage_format, None)
    } else {
        check_init(storage_format, Some(&path))
    }
}

pub fn storage_init(
    storage_format: StorageFormat,
    path: String,
    master_key: String,
    force: bool,
) -> Result<()> {
    if path.is_empty() {
        init(storage_format, None, &master_key, force)
    } else {
        init(storage_format, Some(&path), &master_key, force)
    }
}

pub fn storage_show(
    storage_format: StorageFormat,
    path: String,
    master_key: String,
) -> Result<StorageGroup> {
    if path.is_empty() {
        show(storage_format, None, &master_key)
    } else {
        show(storage_format, Some(&path), &master_key)
    }
}

pub fn storage_add_group(
    storage_format: StorageFormat,
    path: String,
    master_key: String,
    group: String,
) -> Result<()> {
    if path.is_empty() {
        add_group(storage_format, None, &master_key, &group)
    } else {
        add_group(storage_format, Some(&path), &master_key, &group)
    }
}

pub fn storage_add_password(
    storage_format: StorageFormat,
    path: String,
    master_key: String,
    group: String,
    title: String,
    username: String,
    password: String,
) -> Result<()> {
    if path.is_empty() {
        add_password(
            storage_format,
            None,
            &master_key,
            &group,
            &title,
            &username,
            &password,
        )
    } else {
        add_password(
            storage_format,
            Some(&path),
            &master_key,
            &group,
            &title,
            &username,
            &password,
        )
    }
}
