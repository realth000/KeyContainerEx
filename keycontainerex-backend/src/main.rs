use std::error::Error;

use clap::{Arg, ArgAction, Command};

use keepass::db::NodeRef;
use keycontainerex_backend::storage;
use keycontainerex_backend::util::{read_line, read_password};
use keycontainerex_backend::{box_error, unwrap_or_return};

fn main() -> Result<(), Box<dyn Error>> {
    let user_arg = Arg::new("username")
        .short('u')
        .long("username")
        .help("User name or account name");

    let password_arg = Arg::new("password")
        .short('p')
        .long("password")
        .help("Password value");

    let key_arg = Arg::new("key").short('k').long("key");

    let path_arg = Arg::new("path").short('p').long("path");

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
                .arg(path_arg.clone()),
        )
        .subcommand(
            Command::new("add")
                .about("add [group | entry | password] to database")
                .subcommand_required(true)
                .subcommand(
                    Command::new("group")
                        .alias("g")
                        .about("add kdbx group, contains entries data")
                        .arg(Arg::new("groupname").index(1).required(true)),
                )
                .subcommand(
                    Command::new("entry")
                        .alias("e")
                        .about("add kdbx entry, contains passwords data")
                        .arg(Arg::new("entryname").index(1).required(true)),
                )
                .subcommand(
                    Command::new("password")
                        .alias("p")
                        .arg(user_arg.clone())
                        .arg(password_arg.clone()),
                )
                .arg(path_arg.clone().global(true)),
        )
        .subcommand(
            Command::new("remove")
                .arg(user_arg.clone().required(true))
                .arg(password_arg.clone().required(true)),
        )
        .subcommand(
            Command::new("show")
                .arg(key_arg.clone())
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Show all users and password")
                        .action(ArgAction::SetTrue),
                )
                .arg(path_arg.clone()),
        )
        .subcommand(Command::new("config"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let path = init_matches.get_one::<String>("path");
            let force = init_matches.get_flag("force");
            let result = storage::init(path, force);
            if result.is_err() {
                println!("failed to init: {}", result.err().unwrap());
            }
            if path.is_some() {
                println!("[debug] init: path={}", path.unwrap());
            }
        }
        Some(("add", add_matches)) => {
            let path = add_matches.get_one::<String>("path");
            if path.is_some() {
                println!("[debug] add: path={}", path.unwrap());
            }
            let default_key = String::from("");
            let mut readed_key = String::from("");
            let key = add_matches.get_one::<String>("key").unwrap_or_else(|| {
                readed_key = read_password("password: ").unwrap_or(default_key);
                &readed_key
            });
            match add_matches.subcommand() {
                Some(("group", group_matches)) => {
                    let group_name = group_matches.get_one::<String>("groupname").unwrap();
                    println!("[debug] add group {}", group_name);
                    let database = unwrap_or_return!(storage::add_kdbx_group(path, key));
                }
                Some(("entry", entry_matches)) => {
                    let entry_name = entry_matches.get_one::<String>("entryname").unwrap();
                    println!("[debug] add entry {}", entry_name);
                }
                Some(("password", password_matches)) => {
                    let mut readed_username = String::new();
                    let mut readed_password = String::new();
                    let username = password_matches
                        .get_one::<String>("username")
                        .unwrap_or_else(|| {
                            readed_username = read_line("username: ").unwrap();
                            &readed_username
                        });
                    let password = password_matches
                        .get_one::<String>("password")
                        .unwrap_or_else(|| {
                            readed_password = read_password("password: ").unwrap();
                            &readed_password
                        });
                    println!("[debug] add username={}, password={}", username, password);
                }
                _ => {}
            }
            // let username = add_matches.get_one::<String>("username").unwrap();
            // let password = add_matches.get_one::<String>("password").unwrap();
            // println!("[debug] add: username={}, password={}", username, password)
        }
        Some(("remove", remove_matches)) => {
            let username = remove_matches.get_one::<String>("username").unwrap();
            let password = remove_matches.get_one::<String>("password").unwrap();
            println!(
                "[debug] remove: username={}, password={}",
                username, password
            )
        }
        Some(("show", show_matches)) => {
            let show_all = show_matches.get_flag("all");
            println!("[debug] show: show_all={}", show_all);
            let default_key = String::from("");
            let mut readed_key = String::from("");
            let key = show_matches.get_one::<String>("key").unwrap_or_else(|| {
                readed_key = read_password("password: ").unwrap_or(default_key);
                &readed_key
            });
            let path = show_matches.get_one::<String>("path");
            if path.is_some() {
                println!("[debug] show: path={}", path.unwrap());
            }
            let database = unwrap_or_return!(storage::open_kdbx(path, &key));
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
        }
        Some(("config", config_matches)) => {}
        _ => {}
    }
    Ok(())
}
