//! The [glyph-substitution table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/GSUB.htm

use layout::Directory;

mod table;

pub use self::table::*;

/// A glyph-substitution table.
pub type GlyphSubstitution = Directory<Table>;
