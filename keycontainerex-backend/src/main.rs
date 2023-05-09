use clap::{Arg, ArgAction, Command};

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
            Command::new("show").arg(
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
                return;
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
            let show_all = show_matches.get_flag("all");
            println!("[debug] show: show_all={}", show_all);
        }
        Some(("config", config_matches)) => {}
        _ => {}
    }
}
