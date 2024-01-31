use postscript::{self, compact1::FontSet};
use truetype::tables::{
    CharacterMapping, FontHeader, GlyphData, GlyphMapping, HorizontalHeader, HorizontalMetrics,
    MaximumProfile, Names, PostScript, WindowsMetrics,
};
use truetype::{self, Tag};

use crate::tables::{
    ColorPalettes, FontVariations, GlyphDefinition, GlyphPositioning, GlyphSubstitution,
};
use crate::Result;

/// A type representing a font table.
pub trait Table<'l>: Sized {
    #[doc(hidden)]
    type Parameter;

    #[doc(hidden)]
    fn tag() -> Tag;

    #[doc(hidden)]
    fn take<T>(tape: &mut T, parameter: Self::Parameter) -> Result<Self>
    where
        T: crate::tape::Read;
}

macro_rules! table {
    (@one $tag:expr => opentype::$type:ident()) => (
        table! { @one $tag => truetype::$type() }
    );
    (@one $tag:expr => $type:ident()) => (
        impl Table<'static> for $type {
            type Parameter = ();

            #[inline]
            fn tag() -> Tag {
                Tag(*$tag)
            }

            #[inline]
            fn take<T>(tape: &mut T, _: Self::Parameter) -> Result<Self>
            where
                T: $crate::tape::Read,
            {
                $crate::tape::Read::take(tape)
            }
        }
    );
    (@one $tag:expr => $type:ident(..)) => (
        impl<'l> Table<'l> for $type {
            type Parameter = <$type as $crate::walue::Read<'l>>::Parameter;

            #[inline]
            fn tag() -> Tag {
                Tag(*$tag)
            }

            #[inline]
            fn take<T>(tape: &mut T, parameter: Self::Parameter) -> Result<Self>
            where
                T: $crate::tape::Read,
            {
                $crate::tape::Read::take_given(tape, parameter)
            }
        }
    );
    ($($tag:expr => $type:ident($($parameter:tt)*),)+) => (
        $(table! { @one $tag => $type($($parameter)*) })+
    );
}

table! {
    b"CFF " => FontSet(),
    b"CPAL" => ColorPalettes(),
    b"GDEF" => GlyphDefinition(),
    b"GPOS" => GlyphPositioning(),
    b"GSUB" => GlyphSubstitution(),
    b"OS/2" => WindowsMetrics(),
    b"cmap" => CharacterMapping(),
    b"fvar" => FontVariations(),
    b"glyf" => GlyphData(..),
    b"head" => FontHeader(),
    b"hhea" => HorizontalHeader(),
    b"hmtx" => HorizontalMetrics(..),
    b"loca" => GlyphMapping(..),
    b"maxp" => MaximumProfile(),
    b"name" => Names(),
    b"post" => PostScript(),
}
