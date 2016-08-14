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
//! let File { fonts, .. } = File::open(path).unwrap();
//!
//! assert_eq!(fonts[0].font_header.as_ref().unwrap().units_per_em, 1000);
//! assert_eq!(fonts[0].horizontal_header.as_ref().unwrap().ascender, 918);
//! let strings = match fonts[0].naming_table {
//!     Some(NamingTable::Format0(ref table)) => table.strings().unwrap(),
//!     _ => unreachable!(),
//! };
//! assert_eq!(&strings[1], "Source Serif Pro");
//! assert_eq!(&strings[9], "Frank Grießhammer");
//! # }
//! ```

extern crate postscript;

#[macro_use(flags)]
extern crate truetype;

pub use truetype::{Tape, Value, Walue};

#[macro_use]
mod macros;

mod file;
mod font;

pub mod glyph_definition;
pub mod glyph_positioning;
pub mod glyph_substitution;
pub mod layout;

pub use file::File;
pub use font::Font;
pub use glyph_definition::GlyphDefinition;
pub use glyph_positioning::GlyphPositioning;
pub use glyph_substitution::GlyphSubstitution;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::io::Result<T>;
