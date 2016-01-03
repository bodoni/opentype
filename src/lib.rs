//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! extern crate opentype;
//! extern crate truetype;
//!
//! use opentype::File;
//! use truetype::NamingTable;
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let file = File::open(path).unwrap();
//!
//! assert_eq!(file.font_header.as_ref().unwrap().units_per_em, 1000);
//! assert_eq!(file.horizontal_header.as_ref().unwrap().ascender, 918);
//! let strings = match file.naming_table {
//!     Some(NamingTable::Format0(ref table)) => table.strings().unwrap(),
//!     _ => unreachable!(),
//! };
//! assert_eq!(&strings[1], "Source Serif Pro");
//! assert_eq!(&strings[9], "Frank Grießhammer");
//! # }
//! ```

extern crate postscript;
extern crate truetype;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

mod file;

pub use file::File;
