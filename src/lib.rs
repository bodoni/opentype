//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! extern crate opentype;
//!
//! use opentype::Font;
//! use opentype::truetype::{FontHeader, HorizontalHeader, NamingTable};
//!
//! macro_rules! ok(($result:expr) => ($result.unwrap()));
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let mut reader = ok!(std::fs::File::open(path));
//! let font = ok!(Font::read(&mut reader));
//!
//! let font_header: FontHeader = ok!(ok!(font.take(&mut reader)));
//! assert!(font_header.units_per_em == 1000);
//!
//! let horizontal_header: HorizontalHeader = ok!(ok!(font.take(&mut reader)));
//! assert!(horizontal_header.ascender == 918);
//!
//! let naming_table: NamingTable = ok!(ok!(font.take(&mut reader)));
//! match naming_table {
//!     NamingTable::Format0(ref table) => {
//!         let strings = ok!(table.strings());
//!         assert!(&strings[1] == "Source Serif Pro");
//!         assert!(&strings[9] == "Frank Grießhammer");
//!     },
//!     _ => unreachable!(),
//! }
//! # }
//! ```

pub extern crate postscript;

#[macro_use(flags)]
pub extern crate truetype;

#[macro_use]
mod macros;

mod file;
mod font;
mod table;

pub mod compact2;
pub mod glyph_definition;
pub mod glyph_positioning;
pub mod glyph_substitution;
pub mod layout;
pub mod variation;

pub use file::File;
pub use font::Font;
pub use glyph_definition::GlyphDefinition;
pub use glyph_positioning::GlyphPositioning;
pub use glyph_substitution::GlyphSubstitution;
pub use table::Table;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::io::Result<T>;
