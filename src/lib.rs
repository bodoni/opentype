//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! extern crate opentype;
//! extern crate truetype;
//!
//! use opentype::Font;
//! use truetype::{FontHeader, HorizontalHeader, NamingTable};
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
//! assert_eq!(font_header.units_per_em, 1000);
//!
//! let horizontal_header: HorizontalHeader = ok!(ok!(font.take(&mut reader)));
//! assert_eq!(horizontal_header.ascender, 918);
//!
//! let naming_table: NamingTable = ok!(ok!(font.take(&mut reader)));
//! match naming_table {
//!     NamingTable::Format0(ref table) => {
//!         let strings = ok!(table.strings());
//!         assert_eq!(&strings[1], "Source Serif Pro");
//!         assert_eq!(&strings[9], "Frank Grießhammer");
//!     },
//!     _ => unreachable!(),
//! }
//! # }
//! ```

extern crate postscript;

#[macro_use(flags)]
extern crate truetype;

#[macro_use]
mod macros;

mod file;
mod font;
mod table;

pub mod glyph_definition;
pub mod glyph_positioning;
pub mod glyph_substitution;
pub mod glyph_transformation;

pub use file::File;
pub use font::Font;
pub use glyph_definition::GlyphDefinition;
pub use glyph_positioning::GlyphPositioning;
pub use glyph_substitution::GlyphSubstitution;
pub use table::Table;
