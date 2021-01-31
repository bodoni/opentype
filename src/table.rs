use postscript;
use postscript::compact1::FontSet;
use std::io::{Read, Seek};
use truetype::{self, Result, Tag};
use truetype::{
    CharMapping, FontHeader, GlyphData, GlyphMapping, HorizontalHeader, HorizontalMetrics,
    MaximumProfile, NamingTable, PostScript, WindowsMetrics,
};

use crate::{GlyphDefinition, GlyphPositioning, GlyphSubstitution};

/// A font table.
pub trait Table<'l>: Sized {
    #[doc(hidden)]
    type Parameter;

    #[doc(hidden)]
    fn tag() -> Tag;

    #[doc(hidden)]
    fn take<T>(tape: &mut T, parameter: Self::Parameter) -> Result<Self>
    where
        T: Read + Seek;
}

macro_rules! table {
    (@one $tag:expr => opentype::$kind:ident()) => (
        table! { @one $tag => truetype::$kind() }
    );
    (@one $tag:expr => $scope:ident::$kind:ident()) => (
        impl Table<'static> for $kind {
            type Parameter = ();

            #[inline]
            fn tag() -> Tag { Tag(*$tag) }

            #[inline]
            fn take<T>(tape: &mut T, _: Self::Parameter) -> Result<Self>
                where T: Read + Seek
            {
                $scope::Tape::take(tape)
            }
        }
    );
    (@one $tag:expr => $scope:ident::$kind:ident(..)) => (
        impl<'l> Table<'l> for $kind {
            type Parameter = <$kind as $scope::Walue<'l>>::Parameter;

            #[inline]
            fn tag() -> Tag { Tag(*$tag) }

            #[inline]
            fn take<T>(tape: &mut T, parameter: Self::Parameter) -> Result<Self>
                where T: Read + Seek
            {
                $scope::Tape::take_given(tape, parameter)
            }
        }
    );
    ($($tag:expr => $scope:ident::$kind:ident($($parameter:tt)*),)+) => (
        $(table! { @one $tag => $scope::$kind($($parameter)*) })+
    );
}

table! {
    b"CFF " => postscript::FontSet(),
    b"GDEF" => opentype::GlyphDefinition(),
    b"GPOS" => opentype::GlyphPositioning(),
    b"GSUB" => opentype::GlyphSubstitution(),
    b"OS/2" => truetype::WindowsMetrics(),
    b"cmap" => truetype::CharMapping(),
    b"glyf" => truetype::GlyphData(..),
    b"head" => truetype::FontHeader(),
    b"hhea" => truetype::HorizontalHeader(),
    b"hmtx" => truetype::HorizontalMetrics(..),
    b"loca" => truetype::GlyphMapping(..),
    b"maxp" => truetype::MaximumProfile(),
    b"name" => truetype::NamingTable(),
    b"post" => truetype::PostScript(),
}
