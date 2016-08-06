//! The [glyph-substitution table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/GSUB.htm

use layout::Directory;

pub mod table;

use self::table::Table;

/// A glyph-substitution table.
pub type GlyphSubstitution = Directory<Table>;
