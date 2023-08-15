use anyhow::anyhow;

use crate::storage::{
    add_group, add_password, default_save_path, init, show, StorageFormat, StorageGroup,
};

pub fn storage_default_save_path(storage_format: StorageFormat) -> anyhow::Result<String> {
    default_save_path(storage_format)
        .map(|v| v.to_str().unwrap().to_string())
        .map_err(|e| anyhow!(e.to_string()))
        .into()
}

pub fn storage_init(
    storage_format: StorageFormat,
    path: String,
    master_key: String,
    force: bool,
) -> anyhow::Result<()> {
    if path.is_empty() {
        init(storage_format, None, &master_key, force).map_err(|e| anyhow!(e.to_string()))
    } else {
        init(storage_format, Some(&path), &master_key, force).map_err(|e| anyhow!(e.to_string()))
    }
}

pub fn storage_show(
    storage_format: StorageFormat,
    path: String,
    master_key: String,
) -> anyhow::Result<StorageGroup> {
    if path.is_empty() {
        show(storage_format, None, &master_key).map_err(|e| anyhow!(e.to_string()))
    } else {
        show(storage_format, Some(&path), &master_key).map_err(|e| anyhow!(e.to_string()))
    }
}

pub fn storage_add_group(
    storage_format: StorageFormat,
    path: String,
    master_key: String,
    group: String,
) -> anyhow::Result<()> {
    if path.is_empty() {
        add_group(storage_format, None, &master_key, &group).map_err(|e| anyhow!(e.to_string()))
    } else {
        add_group(storage_format, Some(&path), &master_key, &group)
            .map_err(|e| anyhow!(e.to_string()))
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
) -> anyhow::Result<()> {
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
        .map_err(|e| anyhow!(e.to_string()))
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
        .map_err(|e| anyhow!(e.to_string()))
    }
}
