use clap::{Arg, ArgAction, Command};

pub struct ConfigArg<'a> {
    pub name: &'a str,
    pub long: &'a str,
    pub help: &'a str,
    pub action: &'a ArgAction,
}

pub static CONFIG_ARGS: &[ConfigArg] = &[
    ConfigArg {
        name: "allowDuplicateTitle",
        long: "allowDuplicateTitle",
        help: "Allow duplicate title in a password group, default=false",
        action: &ArgAction::Set,
    },
    ConfigArg {
        name: "databasePath",
        long: "databasePath",
        help: "Database file path, default is ~/.config",
        action: &ArgAction::Set,
    },
];

pub fn apply_args(mut command: Command, args: &'static [ConfigArg]) -> Command {
    for arg in args {
        command = command.arg(
            Arg::new(arg.name)
                .long(arg.long)
                .help(arg.help)
                .action(arg.action.to_owned()),
        );
    }
    command
}

macro_rules! arg {
    ($name: expr, $long:expr, $help: expr, $action: expr) => {
        Arg::new($name).long($long).help($help).action($action)
    };
}

macro_rules! args {
    ($(arg:expr),*) => {
        vec![$($arg)*]
    }
}

macro_rules! subcommand {
    ($name: expr =>)
}
