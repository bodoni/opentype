#![allow(non_snake_case)]

use std::io;

pub const CFF_FORMAT_TAG: u32 = 0x4F54544F;

pub const FONT_HEADER_TAG: u32 = 0x68656164;
pub const FONT_HEADER_MAGIC_NUMBER: u32 = 0x5F0F3CF5;

pub const MAXIMAL_PROFILE_TAG: u32 = 0x6d617870;
pub const MAXIMAL_PROFILE_VERSION_0_5: u32 = 0x00005000;

pub trait Spec {
    fn read(stream: &mut io::Reader) -> io::IoResult<Self>;
}

#[inline(always)]
pub fn read<S: Spec, R: io::Reader>(stream: &mut R) -> io::IoResult<S> {
    Spec::read(stream)
}

macro_rules! read_field(
    ($stream:ident, i16) => (try!($stream.read_be_i16()));
    ($stream:ident, u16) => (try!($stream.read_be_u16()));
    ($stream:ident, i32) => (try!($stream.read_be_i32()));
    ($stream:ident, u32) => (try!($stream.read_be_u32()));
    ($stream:ident, f32) => ({
        let value = try!($stream.read_be_u32()) as f32;
        (value * 0.0000152587890625 * 1000.0).round() / 1000.0
    });
    ($stream:ident, i64) => (try!($stream.read_be_i64()));
)

macro_rules! implement_spec(
    ($subject:ident, $($field:ident as $class:ident,)+) => (
        impl Spec for $subject {
            fn read(stream: &mut ::std::io::Reader)
                -> ::std::io::IoResult<$subject> {

                Ok($subject {
                    $($field: read_field!(stream, $class),)+
                })
            }
        }
    )
)

macro_rules! define_spec(
    ($name:ident, $($field:ident as $class:ident,)+) => (
        #[deriving(Default, Show)]
        pub struct $name { $(pub $field: $class,)+ }
        implement_spec!($name, $($field as $class,)+)
    )
)

define_spec!(OffsetTable,
    tag           as u32,
    numTables     as u16,
    searchRange   as u16,
    entrySelector as u16,
    rangeShift    as u16,
)

define_spec!(TableRecord,
    tag      as u32,
    checkSum as u32,
    offset   as u32,
    length   as u32,
)

define_spec!(FontHeader,
    version            as f32,
    fontRevision       as f32,
    checkSumAdjustment as u32,
    magicNumber        as u32,
    flags              as u16,
    unitsPerEm         as u16,
    created            as i64,
    modified           as i64,
    xMin               as i16,
    yMin               as i16,
    xMax               as i16,
    yMax               as i16,
    macStyle           as u16,
    lowestRecPPEM      as u16,
    fontDirectionHint  as i16,
    indexToLocFormat   as i16,
    glyphDataFormat    as i16,
)

define_spec!(MaximumProfile,
    version   as u32,
    numGlyphs as u16,
)
