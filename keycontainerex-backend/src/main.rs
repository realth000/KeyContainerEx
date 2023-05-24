use std::error::Error;

use clap::{Arg, ArgAction, Command};

use keycontainerex_backend::storage;
use keycontainerex_backend::util::read_password;
use keycontainerex_backend::{box_error, unwrap_or_return};

fn main() -> Result<(), Box<dyn Error>> {
    let user_arg = Arg::new("user")
        .short('u')
        .long("user")
        .help("User name or account name")
        .action(ArgAction::Set)
        .required(true);

    let password_arg = Arg::new("password")
        .short('p')
        .long("password")
        .help("Password value")
        .action(ArgAction::Set)
        .required(true);

    let key_arg = Arg::new("key")
        .short('k')
        .long("key")
        .action(ArgAction::Set);

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
                .arg(Arg::new("path").index(1).action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("add")
                .arg(user_arg.clone())
                .arg(password_arg.clone()),
        )
        .subcommand(
            Command::new("remove")
                .arg(user_arg.clone())
                .arg(password_arg.clone()),
        )
        .subcommand(
            Command::new("show").arg(key_arg.clone()).arg(
                Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("Show all users and password")
                    .action(ArgAction::SetTrue),
            ),
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
            let username = add_matches.get_one::<String>("user").unwrap();
            let password = add_matches.get_one::<String>("password").unwrap();
            println!("[debug] add: username={}, password={}", username, password)
        }
        Some(("remove", remove_matches)) => {
            let username = remove_matches.get_one::<String>("user").unwrap();
            let password = remove_matches.get_one::<String>("password").unwrap();
            println!(
                "[debug] remove: username={}, password={}",
                username, password
            )
        }
        Some(("show", show_matches)) => {
            let default_key = String::from("");
            let key = show_matches
                .get_one::<String>("key")
                .unwrap_or(&default_key);
            if key.is_empty() {
                let k = unwrap_or_return!(read_password("password"), "failed to read password");
            }
            let show_all = show_matches.get_flag("all");
            println!("[debug] show: show_all={}", show_all);
        }
        Some(("config", config_matches)) => {}
        _ => {}
    }
    Ok(())
}
