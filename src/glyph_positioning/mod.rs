//! The [glyph-positioning table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/gpos.htm

use layout::Directory;

pub mod table;
pub mod value;

use self::table::Table;

/// A glyph-positioning table.
pub type GlyphPositioning = Directory<Table>;
