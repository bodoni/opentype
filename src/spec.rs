#![allow(non_snake_case)]

use std::io::Reader;

use Result;

/// A 16-bit unsigned integer.
pub type USHORT = u16;

/// A 16-bit signed integer.
pub type SHORT = i16;

/// A 32-bit unsigned integer.
pub type ULONG = u32;

/// A 32-bit signed fixed-point number (16.16).
#[deriving(Default, Eq, PartialEq)]
pub struct Fixed(u32);

/// A date represented in number of seconds since 12:00 midnight, January 1,
/// 1904. The value is represented as a signed 64-bit integer.
pub type LONGDATETIME = i64;

pub const CFF_FORMAT_TAG: &'static [u8] = b"OTTO";

pub const FONT_HEADER_TAG: &'static [u8] = b"head";
pub const FONT_HEADER_VERSION_1_0: Fixed = Fixed(0x00010000);
pub const FONT_HEADER_MAGIC_NUMBER: ULONG = 0x5F0F3CF5;

pub const MAXIMAL_PROFILE_TAG: &'static [u8] = b"maxp";
pub const MAXIMAL_PROFILE_VERSION_0_5: Fixed = Fixed(0x00005000);

pub trait Table {
    fn read(&mut self, reader: &mut Reader) -> Result<()>;
}

macro_rules! define(
    ($name:ident: $($class:ident $field:ident,)+) => (
        #[deriving(Default)]
        pub struct $name { $(pub $field: $class,)+ }
        implement!($name: $($field as $class,)+)
    )
)

macro_rules! implement(
    ($name:ident: $($field:ident as $class:ident,)+) => (
        impl Table for $name {
            fn read(&mut self, reader: &mut Reader) -> Result<()> {
                $(self.$field = read!(reader as $class);)+
                Ok(())
            }
        }
    )
)

macro_rules! read(
    ($reader:ident as USHORT) => (try!($reader.read_be_u16()));
    ($reader:ident as SHORT) => (try!($reader.read_be_i16()));
    ($reader:ident as ULONG) => (try!($reader.read_be_u32()));
    ($reader:ident as Fixed) => (Fixed(try!($reader.read_be_u32())));
    ($reader:ident as LONGDATETIME) => (try!($reader.read_be_i64()));
)

define!(
    OffsetTable:

    Fixed version,
    USHORT numTables,
    USHORT searchRange,
    USHORT entrySelector,
    USHORT rangeShift,
)

define!(
    TableRecord:

    ULONG tag,
    ULONG checkSum,
    ULONG offset,
    ULONG length,
)

define!(
    FontHeader:

    Fixed version,
    Fixed fontRevision,
    ULONG checkSumAdjustment,
    ULONG magicNumber,
    USHORT flags,
    USHORT unitsPerEm,
    LONGDATETIME created,
    LONGDATETIME modified,
    SHORT xMin,
    SHORT yMin,
    SHORT xMax,
    SHORT yMax,
    USHORT macStyle,
    USHORT lowestRecPPEM,
    SHORT fontDirectionHint,
    SHORT indexToLocFormat,
    SHORT glyphDataFormat,
)

define!(
    MaximumProfile:

    Fixed version,
    USHORT numGlyphs,
)

impl Fixed {
    /// Convert `Fixed` into `f32`.
    #[inline]
    pub fn to_f32(&self) -> f32 {
        use std::num::Float;
        ((self.0 as f32) * 0.0000152587890625 * 1000.0).round() / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use spec::{OffsetTable, Table};

    #[test]
    fn offset_table_read() {
        let mut file = ::tests::open("SourceSerifPro-Bold.otf");
        let mut table: OffsetTable = Default::default();

        assert!(table.read(&mut file).is_ok());

        assert_eq!(table.version.0, 0x4f54544f);
        assert_eq!(table.numTables, 12);
        assert_eq!(table.searchRange, 8 * 16);
        assert_eq!(table.entrySelector, 3);
        assert_eq!(table.rangeShift, table.numTables * 16 - table.searchRange);
    }
}
