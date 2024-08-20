use std::fmt;

#[macro_export]
macro_rules! panic_red {
    ($($arg:tt)*) => {{
        eprintln!("\x1b[31;1m{}\x1b[0m", format!($($arg)*));
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! eprintln_red {
    ($($arg:tt)*) => {{
        eprintln!("\x1b[31;1m{}\x1b[0m", format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println_green {
    ($($arg:tt)*) => {{
        println!("\x1b[32m{}\x1b[0m", format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println_cyan {
    ($($arg:tt)*) => {{
        println!("\x1b[36;1m{}\x1b[0m", format!($($arg)*));
    }};
}


// error types here, might make it an individual file if I need more

// Implement da error type
#[derive(Debug)]
pub(crate) enum Polar<T> {
    Some(T),
    Silly(u16),
}

// Implement a display for printing
impl<T: fmt::Debug> fmt::Display for Polar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Polar::Some(value) => write!(f, "Some({:?})", value),
            Polar::Silly(value) => write!(f, "Silly({})", value),
        }
    }
}

// Implement the functions
impl<T: Default> Polar<T> {
    pub(crate) fn is_some(&self) -> bool {
        matches!(self, Polar::Some(_))
    }

    pub(crate) fn is_silly(&self) -> bool {
        matches!(self, Polar::Silly(_))
    }

    // normal unwrap is low-key silly
    pub(crate) fn unwrap_or(self, default: T) -> T {
        match self {
            Polar::Some(value) => value,
            Polar::Silly(_) => default,
        }
    }

    pub(crate) fn unwrap_or_default(self) -> T {
        match self {
            Polar::Some(value) => value,
            Polar::Silly(_) => T::default(),
        }
    }
}


//pub(crate) trait PolarUnwrap<T> {
//    fn unwrap_or_return(self, error_code: u16) -> Polar<T>;
//}
//
//impl<T> PolarUnwrap<T> for Option<T> {
//    fn unwrap_or_return(self, error_code: u16) -> Polar<T> {
//        match self {
//            Some(value) => Polar::Some(value),
//            None => return Polar::Silly(error_code),
//        }
//    }
//}

//impl<T, E> From<Result<T, E>> for Polar<T>
//{
//    fn from(result: Result<T, E>) -> Self {
//        match result {
//            Ok(value) => Polar::Some(value),
//            Err(e) => Polar::Silly(16), // Convert error to u16
//        }
//    }
//}