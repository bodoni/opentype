//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! extern crate opentype;
//! extern crate truetype;
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let mut file = std::fs::File::open(path).unwrap();
//! let opentype::File { fonts, .. } = opentype::File::read(&mut file).unwrap();
//!
//! assert_eq!(fonts[0].font_header(&mut file).unwrap().unwrap().units_per_em, 1000);
//! assert_eq!(fonts[0].horizontal_header(&mut file).unwrap().unwrap().ascender, 918);
//! let strings = match fonts[0].naming_table(&mut file).unwrap().unwrap() {
//!     truetype::NamingTable::Format0(ref table) => table.strings().unwrap(),
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

pub mod glyph_definition;
pub mod glyph_positioning;
pub mod glyph_substitution;
pub mod layout;

pub use file::File;
pub use font::Font;
pub use glyph_definition::GlyphDefinition;
pub use glyph_positioning::GlyphPositioning;
pub use glyph_substitution::GlyphSubstitution;
