//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! extern crate opentype;
//!
//! use std::collections::HashMap;
//!
//! use opentype::Font;
//! use opentype::truetype::{FontHeader, HorizontalHeader};
//! use opentype::truetype::naming_table::{NameID, NamingTable};
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
//! let names: HashMap<_, _> = naming_table
//!     .iter()
//!     .map(|((name_id, _), value)| (name_id, value))
//!     .collect();
//! assert_eq!(ok!(names[&NameID::FullFontName].as_ref()), "Source Serif Pro");
//! assert_eq!(ok!(names[&NameID::DesignerName].as_ref()), "Frank Grießhammer");
//! # }
//! ```

pub extern crate postscript;
pub extern crate truetype;

#[macro_use(
    dereference,
    flags,
    jump_take,
    jump_take_maybe,
    jump_take_given,
    raise,
    table,
)]
extern crate typeface;

pub mod compact2;
pub mod glyph_definition;
pub mod glyph_positioning;
pub mod glyph_substitution;
pub mod layout;
pub mod variation;

mod file;
mod font;
mod table;

pub use typeface::{Error, Result, Tape, Value, Walue};

pub use file::File;
pub use font::Font;
pub use glyph_definition::GlyphDefinition;
pub use glyph_positioning::GlyphPositioning;
pub use glyph_substitution::GlyphSubstitution;
pub use table::Table;
