//! The primary tables.

pub mod font_variations;
pub mod glyph_definition;
pub mod glyph_positioning;
pub mod glyph_substitution;

pub use font_variations::FontVariations;
pub use glyph_definition::GlyphDefinition;
pub use glyph_positioning::GlyphPositioning;
pub use glyph_substitution::GlyphSubstitution;
