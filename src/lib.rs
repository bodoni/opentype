//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! extern crate opentype;
//! extern crate truetype;
//!
//! use opentype::File;
//! use truetype::{FontHeader, HorizontalHeader, NamingTable};
//!
//! macro_rules! ok(($result:expr) => ($result.unwrap()));
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let mut reader = ok!(std::fs::File::open(path));
//! let file = ok!(File::read(&mut reader));
//!
//! assert_eq!(ok!(ok!(file[0].take::<_, FontHeader>(&mut reader))).units_per_em, 1000);
//! assert_eq!(ok!(ok!(file[0].take::<_, HorizontalHeader>(&mut reader))).ascender, 918);
//! let strings = match ok!(ok!(file[0].take::<_, NamingTable>(&mut reader))) {
//!     NamingTable::Format0(ref table) => ok!(table.strings()),
//!     _ => unreachable!(),
//! };
//! assert_eq!(&strings[1], "Source Serif Pro");
//! assert_eq!(&strings[9], "Frank Grießhammer");
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

pub mod glyph_positioning;
pub mod glyph_substitution;
pub mod glyph_transformation;

pub use file::File;
pub use font::Font;
pub use glyph_positioning::GlyphPositioning;
pub use glyph_substitution::GlyphSubstitution;
pub use table::Table;
