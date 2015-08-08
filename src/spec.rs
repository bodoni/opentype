#![allow(non_snake_case)]

use Result;
use input::Read;

/// A 16-bit unsigned integer.
pub type USHORT = u16;

/// A 16-bit signed integer.
pub type SHORT = i16;

/// A 32-bit unsigned integer.
pub type ULONG = u32;

/// A 32-bit signed fixed-point number (Q16.16).
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Fixed(u32);

/// A date represented in seconds since 12:00 midnight, January 1, 1904.
pub type LONGDATETIME = i64;

/// A vector of `USHORT`.
pub type VecUSHORT = Vec<USHORT>;

/// A vector of `SHORT`.
pub type VecSHORT = Vec<SHORT>;

pub const CFF_FORMAT_TAG: &'static [u8; 4] = b"OTTO";

pub const CHAR_MAPPING_TAG: &'static [u8; 4] = b"cmap";
pub const CHAR_MAPPING_HEADER_VERSION_0_0: USHORT = 0;

pub const FONT_HEADER_TAG: &'static [u8; 4] = b"head";
pub const FONT_HEADER_VERSION_1_0: Fixed = Fixed(0x00010000);
pub const FONT_HEADER_MAGIC_NUMBER: ULONG = 0x5F0F3CF5;

pub const MAXIMAL_PROFILE_TAG: &'static [u8; 4] = b"maxp";
pub const MAXIMAL_PROFILE_VERSION_0_5: Fixed = Fixed(0x00005000);

pub trait Table {
    fn read<R: Read>(&mut self, reader: &mut R) -> Result<()>;
}

macro_rules! define(
    ($name:ident: $($class:ident $field:ident,)+) => (
        #[derive(Default)]
        pub struct $name { $(pub $field: $class,)+ }
        implement!($name: $($field as $class,)+);
    )
);

macro_rules! implement(
    ($name:ident: $($field:ident as $class:ident,)+) => (
        impl Table for $name {
            fn read<R: Read>(&mut self, reader: &mut R) -> Result<()> {
                $(self.$field = read!(reader as $class);)+
                Ok(())
            }
        }
    )
);

macro_rules! read(
    ($reader:ident as USHORT) => (try!($reader.read_u16()));
    ($reader:ident as SHORT) => (try!($reader.read_i16()));
    ($reader:ident as ULONG) => (try!($reader.read_u32()));
    ($reader:ident as Fixed) => (Fixed(try!($reader.read_u32())));
    ($reader:ident as LONGDATETIME) => (try!($reader.read_i64()));
    ($reader:ident as VecUSHORT) => ({
        vec![]
    });
    ($reader:ident as VecSHORT) => ({
        vec![]
    });
);

define!(
    OffsetTable:

    Fixed version,
    USHORT numTables,
    USHORT searchRange,
    USHORT entrySelector,
    USHORT rangeShift,
);

define!(
    TableRecord:

    ULONG tag,
    ULONG checkSum,
    ULONG offset,
    ULONG length,
);

define!(
    CharMappingHeader:

    USHORT version,
    USHORT numTables,
);

define!(
    EncodingRecord:

    USHORT platformID,
    USHORT encodingID,
    ULONG offset,
);

define!(
    CharMappingFormat:

    USHORT version,
);

define!(
    CharMappingFormat4:

    USHORT format,
    USHORT length,
    USHORT language,
    USHORT segCountX2,
    USHORT searchRange,
    USHORT entrySelector,
    USHORT rangeShift,
    VecUSHORT endCount,
    USHORT reservedPad,
    VecUSHORT startCount,
    VecSHORT idDelta,
    VecUSHORT idRangeOffset,
    VecUSHORT glyphIdArray,
);

define!(
    CharMappingFormat6:

    USHORT format,
    USHORT length,
    USHORT language,
    USHORT firstCode,
    USHORT entryCount,
    VecUSHORT glyphIdArray,
);

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
);

define!(
    MaximumProfile:

    Fixed version,
    USHORT numGlyphs,
);

impl Fixed {
    #[cfg(test)]
    pub fn as_f32(&self) -> f32 {
        ((self.0 as f32) * 0.0000152587890625 * 1000.0).round() / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use input::Reader;
    use spec::Table;
    use tests;

    macro_rules! assert_ok(
        ($result:expr) => (assert!($result.is_ok()));
    );

    #[test]
    fn offset_table_read() {
        use spec::OffsetTable;

        let mut file = tests::open("SourceSerifPro-Regular.otf");
        let mut reader = Reader::new(&mut file);

        let mut table = OffsetTable::default();
        assert_ok!(table.read(&mut reader));
        assert_eq!(table.version.0, 0x4f54544f);
        assert_eq!(table.numTables, 12);
        assert_eq!(table.searchRange, 8 * 16);
        assert_eq!(table.entrySelector, 3);
        assert_eq!(table.rangeShift, table.numTables * 16 - table.searchRange);
    }

    #[test]
    fn char_mapping_read() {
        use std::io::{Seek, SeekFrom};
        use spec::{CharMappingHeader, EncodingRecord};

        let mut file = tests::open("SourceSerifPro-Regular.otf");
        assert_ok!(file.seek(SeekFrom::Start(15668)));
        let mut reader = Reader::new(&mut file);

        let mut table = CharMappingHeader::default();
        assert_ok!(table.read(&mut reader));
        assert_eq!(table.version, 0);
        assert_eq!(table.numTables, 3);

        let (platforms, encodings) = ([0, 1, 3], [3, 0, 1]);
        for i in 0..3 {
            let mut table = EncodingRecord::default();
            assert_ok!(table.read(&mut reader));
            assert_eq!(table.platformID, platforms[i]);
            assert_eq!(table.encodingID, encodings[i]);
        }
    }
}
