//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! use opentype::Font;
//! use opentype::compound::NamingTable;
//! use std::fs::File;
//!
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let mut file = File::open(path).unwrap();
//! let font = Font::read(&mut file).unwrap();
//!
//! match font.font_header {
//!     Some(ref table) => assert_eq!(table.unitsPerEm, 1000),
//!     _ => unreachable!(),
//! }
//! match font.horizontal_header {
//!     Some(ref table) => assert_eq!(table.Ascender, 918),
//!     _ => unreachable!(),
//! }
//! match font.naming_table {
//!     Some(NamingTable::Format0(ref table)) => {
//!         let strings = table.strings().unwrap();
//!         assert_eq!(&strings[1], "Source Serif Pro");
//!         assert_eq!(&strings[9], "Frank Grießhammer");
//!     },
//!     _ => unreachable!(),
//! }
//! ```

extern crate truetype;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

mod font;

pub use font::Font;
pub use truetype::compound;
