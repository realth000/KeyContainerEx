use std::path::PathBuf;

use clap::{Arg, arg, ArgAction, ArgMatches, Command};

use keycontainerex_backend::storage;

fn main() {
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

    let matches = Command::new("keyContainer")
        .about("Password manage tool")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .arg(user_arg.clone())
                .arg(password_arg.clone())
        )
        .subcommand(
            Command::new("remove")
                .arg(user_arg.clone())
                .arg(password_arg.clone())
        )
        .subcommand(
            Command::new("show")
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Show all users and password")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("config")
        ).get_matches();

    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let username = add_matches.get_one::<String>("user").unwrap();
            let password = add_matches.get_one::<String>("password").unwrap();
            println!("[debug] add: username={:?}, password={:?}", username, password)
        }
        Some(("remove", remove_matches)) => {}
        Some(("show", show_matches)) => {}
        Some(("config", config_matches)) => {}
        _ => {}
    }
}
