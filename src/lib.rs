//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! use opentype::Font;
//! use opentype::table::NamingTable;
//! use std::fs::File;
//!
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let mut file = File::open(path).unwrap();
//! let font = Font::read(&mut file).unwrap();
//!
//! assert_eq!(font.font_header.unitsPerEm, 1000);
//! assert_eq!(font.horizontal_header.Ascender, 918);
//!
//! let strings = match font.naming_table {
//!     NamingTable::Format0(ref table) => table.strings().unwrap(),
//!     _ => unreachable!(),
//! };
//!
//! assert_eq!(&strings[1], "Source Serif Pro");
//! assert_eq!(&strings[9], "Frank Grießhammer");
//! ```

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (
        return Err(::Error::new(::std::io::ErrorKind::Other, $message));
    );
);

mod band;
mod font;

pub mod primitive;
pub mod table;

pub use font::Font;
