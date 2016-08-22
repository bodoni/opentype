//! The [glyph-positioning table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/gpos.htm

use glyph_transformation::Directory;

mod element;
mod table;

pub use self::element::*;
pub use self::table::*;

/// A glyph-positioning table.
pub type GlyphPositioning = Directory<Table>;
