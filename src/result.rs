#![macro_escape]

use std::fmt;
use std::str;

macro_rules! error {
    ($message:expr) => { return Err(::result::Error::new($message)) }
}

pub type Result = ::std::result::Result<(), Error>;

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
