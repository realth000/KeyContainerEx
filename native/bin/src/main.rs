mod config;
mod util;

use std::error::Error;

use clap::{Arg, ArgAction, ArgMatches, Command};
use keepass::db::NodeRef;

use config::{get_config_vec, update_config, ArgEx, ArgExType};
use keycontainerex_backend::storage::init;
use keycontainerex_backend::storage::kdbx::{
    add_kdbx_entry, add_kdbx_group, get_default_kdbx_path, open_kdbx,
};
use keycontainerex_backend::{box_error, unwrap_or_return};

fn get_config_command_args() -> &'static Vec<ArgEx> {
    get_config_vec()
}

fn handle_show_command(show_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let show_all = show_matches.get_flag("all");
    println!("[debug] show: show_all={}", show_all);
    let default_key = String::from("");
    let mut key = String::from("");
    let key = show_matches.get_one::<String>("key").unwrap_or_else(|| {
        key = util::read_password("password: ").unwrap_or(default_key);
        &key
    });
    let database = show_matches.get_one::<String>("database");
    if database.is_some() {
        println!("[debug] show: database={}", database.unwrap());
    }
    let database = unwrap_or_return!(open_kdbx(database, key));
    for node in &database.root {
        match node {
            NodeRef::Group(g) => {
                println!("Saw group {}", g.name);
            }
            NodeRef::Entry(e) => {
                let title = e.get_title().unwrap_or("(no title)");
                let username = e.get_username().unwrap_or("(no username)");
                let password = e.get_password().unwrap_or("(no password)");
                println!("Entry '{}': '{}' '{}'", title, username, password);
            }
        }
    }
    Ok(())
}

fn handle_add_command(add_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let database = add_matches.get_one::<String>("database");
    if database.is_some() {
        println!("[debug] add: database={}", database.unwrap());
    }
    let default_key = String::from("");
    let mut key = String::from("");
    let key = add_matches.get_one::<String>("key").unwrap_or_else(|| {
        key = util::read_password("password: ").unwrap_or(default_key);
        &key
    });
    match add_matches.subcommand() {
        Some(("group", group_matches)) => {
            let group_name = group_matches.get_one::<String>("groupname").unwrap();
            println!("[debug] add group {}", group_name);
            let ret = add_kdbx_group(database, key, group_name);
            if ret.is_err() {
                return box_error!("failed to add group: {}", ret.unwrap_err().to_string());
            }
            Ok(())
        }
        Some(("password", password_matches)) => {
            let mut group = String::new();
            let mut title = String::new();
            let mut username = String::new();
            let mut password = String::new();
            let group = password_matches
                .get_one::<String>("group")
                .unwrap_or_else(|| {
                    group = util::read_line("group (press enter to skip): ").unwrap();
                    &group
                });
            let title = password_matches
                .get_one::<String>("title")
                .unwrap_or_else(|| {
                    title = util::read_line("title: ").unwrap();
                    &title
                });
            let username = password_matches
                .get_one::<String>("username")
                .unwrap_or_else(|| {
                    username = util::read_line("username: ").unwrap();
                    &username
                });
            let password = password_matches
                .get_one::<String>("password")
                .unwrap_or_else(|| {
                    password = util::read_password("password: ").unwrap();
                    &password
                });
            println!("[debug] add username={}, password={}", username, password);
            let ret = add_kdbx_entry(database, key, group, title, username, password);
            if ret.is_err() {
                return box_error!("failed to add password: {}", ret.unwrap_err().to_string());
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn handle_remove_command(remove_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match remove_matches.subcommand() {
        Some(("group", group_matches)) => {
            let group_name = group_matches.get_one::<String>("group").unwrap();
            println!("[debug] add group {}", group_name);
            Ok(())
        }
        Some(("password", password_matches)) => Ok(()),
        _ => Ok(()),
    }
}

fn handle_config_command(config_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    for arg in get_config_command_args() {
        if config_matches
            .value_source(arg.arg_value.get_id().as_str())
            .is_none()
        {
            continue;
        }

        match arg.arg_type {
            ArgExType::Bool(_) => update_config(
                None,
                arg.arg_name.as_str(),
                ArgExType::Bool(config_matches.get_flag(arg.arg_name.as_str())),
            ),
            ArgExType::String(_) => update_config(
                None,
                arg.arg_name.as_str(),
                ArgExType::String(
                    config_matches
                        .get_one::<String>(arg.arg_name.as_str())
                        .unwrap()
                        .to_owned(),
                ),
            ),
        }?
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let title_arg = Arg::new("title")
        .short('t')
        .long("title")
        .help("Title for this pair of username and password");

    let user_arg = Arg::new("username")
        .short('u')
        .long("username")
        .help("User name or account name");

    let password_arg = Arg::new("password")
        .short('p')
        .long("password")
        .help("Password value");

    let key_arg = Arg::new("key").short('k').long("key");

    let database_arg = Arg::new("database").short('d').long("database");

    let group_arg = Arg::new("group")
        .short('g')
        .long("group")
        .help("Specify a group to save password, by group name");

    let matches = Command::new("keyContainer")
        .about("Password manage tool")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Force create, will remove the old file if already exists.")
                        .action(ArgAction::SetTrue),
                )
                .arg(key_arg.clone())
                .arg(database_arg.clone()),
        )
        .subcommand(
            Command::new("add")
                .about("add [group | password] to database")
                .subcommand_required(true)
                .subcommand(
                    Command::new("group")
                        .alias("g")
                        .about("add kdbx group, contains entries data")
                        .arg(Arg::new("groupname").index(1).required(true)),
                )
                .subcommand(
                    Command::new("password")
                        .alias("p")
                        .arg(group_arg.clone())
                        .arg(title_arg.clone())
                        .arg(user_arg.clone())
                        .arg(password_arg.clone()),
                )
                .arg(key_arg.clone().global(true))
                .arg(database_arg.clone().global(true)),
        )
        .subcommand(
            Command::new("remove")
                .about("remove [group | password] from database")
                .subcommand_required(true)
                .subcommand(
                    Command::new("group")
                        .alias("g")
                        .about("remove kdbx group, will clear all password and group under it.")
                        .arg(Arg::new("groupname").index(1).required(true)),
                )
                .subcommand(Command::new("password").alias("p").arg(group_arg.clone()))
                .arg(title_arg.clone())
                .arg(user_arg),
        )
        .subcommand(
            Command::new("show")
                .arg(key_arg)
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Show all users and password")
                        .action(ArgAction::SetTrue),
                )
                .arg(database_arg),
        )
        .subcommand(
            Command::new("config").about("Setup configs").args(
                get_config_command_args()
                    .iter()
                    .map(|arg_ex| &arg_ex.arg_value)
                    .collect::<Vec<&Arg>>(),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let mut database = init_matches.get_one::<String>("database");
            let mut force = init_matches.get_flag("force");
            let default_database = String::from(get_default_kdbx_path()?.to_str().unwrap());
            if database.is_none() {
                database = Some(&default_database);
            }

            if std::path::PathBuf::from(database.unwrap()).exists() {
                match util::ask_yes_or_no("Database already exists, delete it?") {
                    Ok(v) => {
                        if !v {
                            return Ok(());
                        }
                        force = true;
                    }
                    Err(e) => {
                        return box_error!("{}", e);
                    }
                }
            }
            let key = match init_matches.get_one::<String>("key") {
                Some(v) => v.clone(),
                None => {
                    let key = util::read_password("Key (database password): ").unwrap();
                    if key.is_empty() {
                        return box_error!("empty key");
                    }
                    let key_confirm = util::read_password("Confirm key: ").unwrap();
                    if key != key_confirm {
                        return box_error!("key and its confirm not the same");
                    }
                    key
                }
            };
            let result = init(database, &key, force);
            if result.is_err() {
                println!("failed to init: {}", result.err().unwrap());
            }
            if database.is_some() {
                println!("[debug] init: database={}", database.unwrap());
            }
        }
        Some(("add", add_matches)) => {
            return handle_add_command(add_matches);
        }
        Some(("remove", remove_matches)) => {}
        Some(("show", show_matches)) => {
            return handle_show_command(show_matches);
        }
        Some(("config", config_matches)) => {
            return handle_config_command(config_matches);
        }
        _ => {}
    }
    Ok(())
}
