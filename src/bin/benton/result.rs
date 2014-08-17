#![macro_escape]

macro_rules! raise(
    ($kind:ident) => (
        return Err(::result::Error {
            kind: $kind,
            message: String::from_str(match $kind {
                ArgumentError => "Wrong arguments.",
                ParseError => "Cannot parse the file."
            })
        })
    );
    ($kind:ident, $($arguments:tt)+) => (
        return Err(::result::Error {
            kind: $kind,
            message: format!($($arguments)+),
        })
    );
)

macro_rules! try(
    ($suspect:expr, $kind:ident) => (
        match $suspect {
            Ok(result) => result,
            Err(error) => raise!($kind, "{}", error_message!(error))
        }
    );
    ($suspect:expr, $kind:ident, $($arguments:tt)+) => (
        match $suspect {
            Ok(result) => result,
            Err(_) => raise!($kind, $($arguments)+)
        }
    );
)

macro_rules! error_message(
    ($error:expr) => (
        match $error.kind {
            ::std::io::FileNotFound => "The file does not exist.",
            _ => $error.desc
        }
    )
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

impl ::std::fmt::Show for Error {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}
