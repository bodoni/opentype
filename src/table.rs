use postscript::{self, compact1::FontSet};
use truetype::tables::{
    CharacterMapping, FontHeader, GlyphData, GlyphMapping, HorizontalHeader, HorizontalMetrics,
    MaximumProfile, Names, PostScript, WindowsMetrics,
};
use truetype::{self, Tag};
use typeface::Tape;

use crate::{FontVariations, GlyphDefinition, GlyphPositioning, GlyphSubstitution, Result};

/// A type representing a font table.
pub trait Table<'l>: Sized {
    #[doc(hidden)]
    type Parameter;

    #[doc(hidden)]
    fn tag() -> Tag;

    #[doc(hidden)]
    fn take<T>(tape: &mut T, parameter: Self::Parameter) -> Result<Self>
    where
        T: Tape;
}

macro_rules! table {
    (@one $tag:expr => opentype::$type:ident()) => (
        table! { @one $tag => truetype::$type() }
    );
    (@one $tag:expr => $scope:ident::$type:ident()) => (
        impl Table<'static> for $type {
            type Parameter = ();

            #[inline]
            fn tag() -> Tag {
                Tag(*$tag)
            }

            #[inline]
            fn take<T>(tape: &mut T, _: Self::Parameter) -> Result<Self>
            where
                T: Tape,
            {
                $scope::Tape::take(tape)
            }
        }
    );
    (@one $tag:expr => $scope:ident::$type:ident(..)) => (
        impl<'l> Table<'l> for $type {
            type Parameter = <$type as $scope::Walue<'l>>::Parameter;

            #[inline]
            fn tag() -> Tag {
                Tag(*$tag)
            }

            #[inline]
            fn take<T>(tape: &mut T, parameter: Self::Parameter) -> Result<Self>
            where
                T: Tape,
            {
                $scope::Tape::take_given(tape, parameter)
            }
        }
    );
    ($($tag:expr => $scope:ident::$type:ident($($parameter:tt)*),)+) => (
        $(table! { @one $tag => $scope::$type($($parameter)*) })+
    );
}

table! {
    b"CFF " => postscript::FontSet(),
    b"GDEF" => opentype::GlyphDefinition(),
    b"GPOS" => opentype::GlyphPositioning(),
    b"GSUB" => opentype::GlyphSubstitution(),
    b"OS/2" => truetype::WindowsMetrics(),
    b"cmap" => truetype::CharacterMapping(),
    b"fvar" => opentype::FontVariations(),
    b"glyf" => truetype::GlyphData(..),
    b"head" => truetype::FontHeader(),
    b"hhea" => truetype::HorizontalHeader(),
    b"hmtx" => truetype::HorizontalMetrics(..),
    b"loca" => truetype::GlyphMapping(..),
    b"maxp" => truetype::MaximumProfile(),
    b"name" => truetype::Names(),
    b"post" => truetype::PostScript(),
}
