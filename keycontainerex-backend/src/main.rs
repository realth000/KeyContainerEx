use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {

    Add {

    },

    Remove {

    },

    Show {

    },

    Config {

    },

    Test {
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Some(name) = cli.name.as_deref() {
        println!("name's value = {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("config's value = {}", config_path.display())
    }

    match cli.debug {
        0 => println!("debug level = 0"),
        1 => println!("debug level = 1"),
        _ => println!("too many debugs"),
    }

    match &cli.command {
        Some(Commands::Add {  }) => {

        }

        Some(Commands::Remove {  }) => {

        }
        Some(Commands::Show {  }) => {

        }
        Some(Commands::Config {  }) => {

        }
        Some(Commands::Test { list }) => {
            if *list {
                println!("printing test lists...")
            } else {
                println!("Not printing test lists...")
            }
        }
        None => {}
    }
}
