//! Parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! use opentype::truetype::tables::names::{Names, NameID};
//! use opentype::truetype::tables::{FontHeader, HorizontalHeader};
//! use opentype::File;
//!
//! macro_rules! ok(($result:expr) => ($result.unwrap()));
//!
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let mut tape = ok!(std::fs::File::open(path));
//! let File { mut fonts } = ok!(File::read(&mut tape));
//!
//! let font_header: FontHeader = ok!(ok!(fonts[0].take(&mut tape)));
//! assert_eq!(font_header.units_per_em, 1000);
//!
//! let horizontal_header: HorizontalHeader = ok!(ok!(fonts[0].take(&mut tape)));
//! assert_eq!(horizontal_header.ascender, 918);
//!
//! let names: Names = ok!(ok!(fonts[0].take(&mut tape)));
//! let names = names
//!     .iter()
//!     .map(|((name_id, _), value)| (name_id, value))
//!     .collect::<std::collections::HashMap<_, _>>();
//! assert_eq!(ok!(names[&NameID::FullFontName].as_ref()), "Source Serif Pro");
//! assert_eq!(ok!(names[&NameID::DesignerName].as_ref()), "Frank Grießhammer");
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
    table
)]
extern crate typeface;

pub mod layout;
pub mod tables;
pub mod variations;

mod file;
mod font;
mod table;

pub use typeface::{Error, Result, Tape, Value, Walue};

pub use file::File;
pub use font::Font;
pub use table::Table;

/// Check if a tag is recognized.
#[inline]
pub fn accept(tag: &truetype::Tag) -> bool {
    matches!(&tag.0, b"ttcf") || truetype::accept(tag)
}
