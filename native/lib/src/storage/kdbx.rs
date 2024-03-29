use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

use anyhow::{bail, Result};
use dirs;
use keepass::db::{Entry, Group, Node, NodeRefMut, Value};
use keepass::{Database, DatabaseKey};

use crate::storage::{StorageGroup, StoragePassword};

// Parse raw path into vector of string.
// e.g.
//   a/b/c/d   => vec!["a", "b", "c", "d"]
//   a/b\/c/d  => vec!["a", "b/c", "d"]
//   a/b\\/c/d => vec!["a", "b\\c", "d"]
//   /a/b/c/d  => vec!["", "b", "c", "d"]
fn parse_group_path(raw_path: &str) -> Vec<String> {
    if !raw_path.contains('\\') {
        return raw_path
            .split('/')
            .map(String::from)
            .collect::<Vec<String>>();
    }

    let mut group_path_vec: Vec<String> = vec![];
    let mut current_group_path = String::new();
    let mut have_backslash = false;

    for raw_char in raw_path.chars() {
        if have_backslash {
            // Previous char is backslash(\)
            current_group_path.push(raw_char);
            println!("=> {}", current_group_path);
            have_backslash = false;
        } else if raw_char == '\\' {
            have_backslash = true;
        } else if raw_char == '/' {
            group_path_vec.push(current_group_path);
            current_group_path = String::new();
        } else {
            current_group_path.push(raw_char);
            println!("=> {}", current_group_path);
        }
    }
    if !current_group_path.is_empty() {
        group_path_vec.push(current_group_path);
    }
    group_path_vec
}

fn locate_group_node<'a>(node: &'a mut Group, path_vec: &[&str]) -> Result<&'a mut Group> {
    return if let Some(NodeRefMut::Group(g)) = node.get_mut(path_vec) {
        Ok(g)
    } else {
        bail!("failed to found")
    };

    // let mut current_node: &mut Group = node;
    // for path in path_vec {
    //     let mut not_found = true;
    //     for child in current_node.children {
    //         if let Node::Group(child_group) = child {
    //             if &child_group.name == path {
    //                 // Get next step
    //                 not_found = false;
    //                 current_node = &child_group;
    //                 break;
    //             }
    //         }
    //     }
    //     if not_found {
    //         return box_error!("failed to find group path {}", path);
    //     }
    // }
    // Ok(current_node)
}

fn get_kdbx_file() -> Result<PathBuf> {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push("KeyContainerEx");
            path.push("default.kdbx");
            Ok(path)
        }
        _ => bail!("failed to get kdbx path"),
    }
}

pub fn get_default_kdbx_path() -> Result<PathBuf> {
    get_kdbx_file()
}

pub fn init_kdbx(path: Option<&String>, master_key: &str, force: bool) -> Result<()> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(&path),
        None => get_kdbx_file()?,
    };
    if kdbx_path.exists() {
        if !force {
            return bail!("file already exists");
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
        DatabaseKey::with_password(master_key),
    )?;

    Ok(())
}

pub fn open_kdbx(path: Option<&String>, master_key: &str) -> Result<Database> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    Ok(Database::open(
        &mut File::open(kdbx_path)?,
        DatabaseKey::with_password(master_key),
    )?)
}

fn search_kdbx_node(node: &Group, group: &mut StorageGroup) {
    for node in &node.children {
        match node {
            Node::Group(g) => {
                let mut sg = StorageGroup::new(g.name.clone());
                search_kdbx_node(g, &mut sg);
                group.add_sub_group(sg);
            }
            Node::Entry(e) => {
                let title = e.get_title().unwrap_or("(no title)");
                let username = e.get_username().unwrap_or("(no username)");
                let password = e.get_password().unwrap_or("(no password)");
                group.add_sub_password(StoragePassword::new(
                    title.to_string(),
                    username.to_string(),
                    password.to_string(),
                ));
            }
        }
    }
}

pub fn show_kdbx(database: Database) -> Result<StorageGroup> {
    let mut root_group = StorageGroup::new(database.root.name.clone());
    search_kdbx_node(&database.root, &mut root_group);
    Ok(root_group)
}

pub fn add_kdbx_group(path: Option<&String>, master_key: &str, group: &str) -> Result<()> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    let mut database = Database::open(
        &mut File::open(&kdbx_path)?,
        DatabaseKey::with_password(master_key),
    )?;
    let mut group_full_path_vec = parse_group_path(group);
    let group_name = group_full_path_vec.pop().unwrap();
    let group_parent = match locate_group_node(
        &mut database.root,
        &group_full_path_vec
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<&str>>(),
    ) {
        Ok(parent) => parent,
        Err(e) => return Err(e),
    };
    println!("find parent {:?}", group_parent.clone().name);
    let g = Group::new(&group_name);
    group_parent.children.push(Node::Group(g));
    database.save(
        &mut OpenOptions::new().write(true).open(&kdbx_path)?,
        DatabaseKey::with_password(master_key),
    )?;
    Ok(())
}

pub fn remove_kdbx_group(path: Option<&String>, master_key: &str, group: &str) -> Result<()> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    let mut database = Database::open(
        &mut File::open(&kdbx_path)?,
        DatabaseKey::with_password(master_key),
    )?;
    // TODO: Implement group delete.

    Ok(())
}

pub fn add_kdbx_entry(
    path: Option<&String>,
    master_key: &str,
    group: &str,
    title: &str,
    username: &str,
    password: &str,
) -> Result<()> {
    let kdbx_path = match path {
        Some(path) => PathBuf::from(path),
        None => get_kdbx_file()?,
    };
    let mut database = Database::open(
        &mut File::open(&kdbx_path)?,
        DatabaseKey::with_password(master_key),
    )?;
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

    // Allow empty group which means add password under root node.
    if !group.is_empty() {
        match database.root.get_mut(
            &parse_group_path(group)
                .iter()
                .map(|e| e.as_str())
                .collect::<Vec<&str>>(),
        ) {
            Some(NodeRefMut::Group(g)) => {
                g.children.push(Node::Entry(entry));
            }
            Some(NodeRefMut::Entry(_)) => {}
            None => {
                return bail!("failed to add password: group \"{}\" not found", &group);
            }
        }
    } else {
        database.root.children.push(Node::Entry(entry));
    }

    database.save(
        &mut OpenOptions::new().write(true).open(&kdbx_path)?,
        DatabaseKey::with_password(master_key),
    )?;
    Ok(())
}

pub fn remove_kdbx_entry(
    path: Option<&String>,
    master_Key: &str,
    group: &str,
    title: &str,
    username: &str,
    password: &str,
) -> Result<()> {
    Ok(())
}
