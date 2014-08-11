#![macro_escape]

use std::fmt;

macro_rules! raise(
    ($kind:ident) => (
        return Err(::result::Error {
            kind: $kind,
            message: String::new(),
        })
    );
    ($kind:ident, $($arguments:tt)+) => (
        return Err(::result::Error {
            kind: $kind,
            message: format!($($arguments)+),
        })
    );
)

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

pub enum ErrorKind {
    ArgumentError,
    ParseError,
}

impl fmt::Show for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}
