//! The [glyph-positioning table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/gpos.htm

use layout::Directory;

mod table;
mod value;

pub use self::table::*;
pub use self::value::*;

/// A glyph-positioning table.
pub type GlyphPositioning = Directory<Table>;
