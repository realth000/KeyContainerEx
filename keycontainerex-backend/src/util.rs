use std::io;
use std::io::Write;

use clap::Arg;
use rpassword;

pub enum ArgExType {
    Bool(bool),
    String(String),
}

pub struct ArgEx {
    pub arg_name: String,
    pub arg_value: Arg,
    pub arg_type: ArgExType,
}

#[macro_export]
macro_rules! box_error {
    ($($arg:tt)*) => {Err(Box::<dyn Error>::from(format!($($arg)*)))};
}

// #[macro_export]
// macro_rules! unwrap_or_return {
//     ($e:expr, $($msg:literal)*) => {
//         match $e {
//             Ok(v) => v,
//             Err(_) => return box_error!($($msg)*),
//         }
//     };
// }

#[macro_export]
macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(Box::<dyn Error>::from(e.to_string())),
        }
    };
}

pub fn read_line(hint: &str) -> io::Result<String> {
    print!("{}", hint);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    if result.ends_with('\n') {
        result.pop();
    }
    Ok(result)
}

pub fn read_password(hint: &str) -> io::Result<String> {
    print!("{}", hint);
    io::stdout().flush().unwrap();
    rpassword::read_password()
}

#[macro_export]
macro_rules! arg_ex {
    ($name: literal, $arg_type: expr, $help: expr, $action: expr) => {
        ArgEx {
            arg_name: String::from($name),
            arg_value: Arg::new($name).long($name).help($help).action($action),
            arg_type: $arg_type,
        }
    };
}

// macro_rules! arg_vec {
//     ($(arg:expr),*) => {
//         vec![$($arg)*]
//     }
// }
//
// macro_rules! subcommand {
//     ($name: expr => $($arg:ident,),*) => {{
//         let args = arg_vec![$($arg.name, $arg.long, $arg.help, $arg.action),*];
//         SubCommand::new($name).$(.arg(args.get(stringify!($arg)).unwrap()))*
//     }};
// }

pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
