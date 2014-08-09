#![macro_escape]

use std::fmt;
use std::str;

macro_rules! error(
    ($message:expr) => (
        return Err(::result::Error::new($message))
    )
)

macro_rules! ensure(
    ($suspect:expr, $message:expr) => (
        match $suspect {
            Ok(result) => result,
            Err(_) => error!($message)
        }
    )
)

macro_rules! fetch(
    ($subject:expr, $default:expr) => (
        match $subject {
            Some(result) => result,
            None => $default
        }
    )
)

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct Error {
    message: str::SendStr,
}

impl Error {
    pub fn new<T: str::IntoMaybeOwned<'static>>(message: T) -> Error {
        Error { message: message.into_maybe_owned() }
    }
}

impl fmt::Show for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}
