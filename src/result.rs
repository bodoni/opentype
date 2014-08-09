#![macro_escape]

use std::fmt;

macro_rules! error(
    ($kind:expr) => (
        return Err(::result::Error::new($kind, ""))
    );
    ($kind:expr, $message:expr) => (
        return Err(::result::Error::new($kind, $message))
    );
)

macro_rules! try(
    ($suspect:expr, $kind:expr) => (
        match $suspect {
            Ok(result) => result,
            Err(_) => error!($kind)
        }
    );
    ($suspect:expr, $kind:expr, $message:expr) => (
        match $suspect {
            Ok(result) => result,
            Err(_) => error!($kind, $message)
        }
    );
)

macro_rules! assert(
    ($condition:expr, $kind:expr) => (
        match $condition {
            true => (),
            false => error!($kind)
        }
    );
    ($condition:expr, $kind:expr, $message:expr) => (
        match $condition {
            true => (),
            false => error!($kind, $message)
        }
    );
)

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct Error {
    kind: ErrorKind,
    desc: &'static str,
}

pub enum ErrorKind {
    FormatError,
    IOError,
    OtherError,
}

impl Error {
    pub fn new(kind: ErrorKind, desc: &'static str) -> Error {
        Error { kind: kind, desc: desc }
    }
}

impl fmt::Show for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let desc = if self.desc.is_empty() {
                        match self.kind {
                            FormatError => "Invalid format.",
                            IOError => "Cannot read the file.",
                            _ => "Unknown error."
                        }
                    }
                    else {
                        self.desc
                    };

        write!(formatter, "{}", desc)
    }
}
