use postscript::{self, compact1::FontSet};
use truetype::tables::{
    CharacterMapping, FontHeader, GlyphData, GlyphMapping, HorizontalHeader, HorizontalMetrics,
    MaximumProfile, Names, PostScript, WindowsMetrics,
};
use truetype::{self, Tag};

use crate::tables::{
    ColorPalettes, FontVariations, GlyphDefinition, GlyphPositioning, GlyphSubstitution,
};

/// A type representing a font table.
pub trait Table {
    #[doc(hidden)]
    fn tag() -> Tag;
}

macro_rules! implement {
    ($($tag:expr => $type:ident,)+) => {
        $(impl Table for $type {
            #[inline]
            fn tag() -> Tag {
                Tag(*$tag)
            }
        })+
    };
}

implement! {
    b"CFF " => FontSet,
    b"CPAL" => ColorPalettes,
    b"GDEF" => GlyphDefinition,
    b"GPOS" => GlyphPositioning,
    b"GSUB" => GlyphSubstitution,
    b"OS/2" => WindowsMetrics,
    b"cmap" => CharacterMapping,
    b"fvar" => FontVariations,
    b"glyf" => GlyphData,
    b"head" => FontHeader,
    b"hhea" => HorizontalHeader,
    b"hmtx" => HorizontalMetrics,
    b"loca" => GlyphMapping,
    b"maxp" => MaximumProfile,
    b"name" => Names,
    b"post" => PostScript,
}
