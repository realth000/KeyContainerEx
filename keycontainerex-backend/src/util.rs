macro_rules! box_error {
    ($($arg:tt)*) => {Err(Box::<dyn Error>::from(format!($($arg)*)))};
}

pub(crate) use box_error;
