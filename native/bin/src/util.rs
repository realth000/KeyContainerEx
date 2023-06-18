use std::io;
use std::io::Write;

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

pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
