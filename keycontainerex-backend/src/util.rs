use std::io;
use std::io::Write;

use rpassword;

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

pub fn read_password(hint: &str) -> io::Result<String> {
    print!("{}", hint);
    io::stdout().flush().unwrap();
    rpassword::read_password()
}
