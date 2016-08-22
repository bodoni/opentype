//! The [glyph-substitution table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/GSUB.htm

use glyph_transformation::Directory;

mod element;
mod table;

pub use self::element::*;
pub use self::table::*;

/// A glyph-substitution table.
pub type GlyphSubstitution = Directory<Table>;
