#![allow(non_snake_case)]

use Result;
use band::Band;

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

pub const CFF_FORMAT_TAG: &'static [u8; 4] = b"OTTO";

pub const CHAR_MAPPING_TAG: &'static [u8; 4] = b"cmap";
pub const CHAR_MAPPING_HEADER_VERSION_0_0: USHORT = 0;

pub const FONT_HEADER_TAG: &'static [u8; 4] = b"head";
pub const FONT_HEADER_VERSION_1_0: Fixed = Fixed(0x00010000);
pub const FONT_HEADER_MAGIC_NUMBER: ULONG = 0x5F0F3CF5;

pub const MAXIMAL_PROFILE_TAG: &'static [u8; 4] = b"maxp";
pub const MAXIMAL_PROFILE_VERSION_0_5: Fixed = Fixed(0x00005000);

pub trait Table {
    fn read<T: Band>(&mut self, band: &mut T) -> Result<()>;
}

macro_rules! table(
    ($structure:ident { $($field:ident [$($kind:tt)+] $(|$this:ident| $body:block)*,)+ }) => (
        define!($structure { $($field $($kind)+,)+ });
        implement!($structure { $($field [$($kind)+] $(|$this| $body)*,)+ });
    );
);

macro_rules! define(
    ($structure:ident { $($field:ident $kind:ty,)+ }) => (
        itemize! {
            #[derive(Default)]
            pub struct $structure { $(pub $field: $kind,)+ }
        }
    );
);

macro_rules! itemize(
    ($code:item) => ($code);
);

macro_rules! implement(
    ($structure:ident { $($field:ident [$($kind:tt)+] $(|$this:ident| $body:block)*,)+ }) => (
        impl Table for $structure {
            fn read<T: Band>(&mut self, band: &mut T) -> Result<()> {
                $(self.$field = read!($structure, self, band, $($kind)+ $(|$this| $body)*);)+
                Ok(())
            }
        }
    );
);

macro_rules! read(
    ($structure:ident, $this:ident, $band:ident, USHORT) => ({
        try!($band.read_u16())
    });
    ($structure:ident, $this:ident, $band:ident, SHORT) => ({
        try!($band.read_i16())
    });
    ($structure:ident, $this:ident, $band:ident, ULONG) => ({
        try!($band.read_u32())
    });
    ($structure:ident, $this:ident, $band:ident, Fixed) => ({
        Fixed(try!($band.read_u32()))
    });
    ($structure:ident, $this:ident, $band:ident, LONGDATETIME) => ({
        try!($band.read_i64())
    });
    ($structure:ident, $this:ident, $band:ident, Vec<USHORT> |$that:ident| $body:block) => ({
        #[allow(unused_variables)]
        fn count($that: &$structure) -> usize $body
        let _ = count($this);
        vec![]
    });
    ($structure:ident, $this:ident, $band:ident, Vec<SHORT> |$that:ident| $body:block) => ({
        #[allow(unused_variables)]
        fn count($that: &$structure) -> usize $body
        let _ = count($this);
        vec![]
    });
);

table!(OffsetTable {
    version       [Fixed ],
    numTables     [USHORT],
    searchRange   [USHORT],
    entrySelector [USHORT],
    rangeShift    [USHORT],
});

table!(TableRecord {
    tag      [ULONG],
    checkSum [ULONG],
    offset   [ULONG],
    length   [ULONG],
});

table!(CharMappingHeader {
    version   [USHORT],
    numTables [USHORT],
});

table!(EncodingRecord {
    platformID [USHORT],
    encodingID [USHORT],
    offset     [ULONG ],
});

table!(CharMappingFormat {
    version [USHORT],
});

table!(CharMappingFormat4 {
    format        [USHORT     ],
    length        [USHORT     ],
    language      [USHORT     ],
    segCountX2    [USHORT     ],
    searchRange   [USHORT     ],
    entrySelector [USHORT     ],
    rangeShift    [USHORT     ],
    endCount      [Vec<USHORT>] |table| { 0 },
    reservedPad   [USHORT     ],
    startCount    [Vec<USHORT>] |table| { 0 },
    idDelta       [Vec<SHORT> ] |table| { 0 },
    idRangeOffset [Vec<USHORT>] |table| { 0 },
    glyphIdArray  [Vec<USHORT>] |table| { 0 },
});

table!(CharMappingFormat6 {
    format       [USHORT     ],
    length       [USHORT     ],
    language     [USHORT     ],
    firstCode    [USHORT     ],
    entryCount   [USHORT     ],
    glyphIdArray [Vec<USHORT>] |table| { 0 },
});

table!(FontHeader {
    version            [Fixed       ],
    fontRevision       [Fixed       ],
    checkSumAdjustment [ULONG       ],
    magicNumber        [ULONG       ],
    flags              [USHORT      ],
    unitsPerEm         [USHORT      ],
    created            [LONGDATETIME],
    modified           [LONGDATETIME],
    xMin               [SHORT       ],
    yMin               [SHORT       ],
    xMax               [SHORT       ],
    yMax               [SHORT       ],
    macStyle           [USHORT      ],
    lowestRecPPEM      [USHORT      ],
    fontDirectionHint  [SHORT       ],
    indexToLocFormat   [SHORT       ],
    glyphDataFormat    [SHORT       ],
});

table!(MaximumProfile {
    version   [Fixed ],
    numGlyphs [USHORT],
});

impl Fixed {
    #[cfg(test)]
    pub fn as_f32(&self) -> f32 {
        ((self.0 as f32) * 0.0000152587890625 * 1000.0).round() / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use spec::Table;
    use tests;

    macro_rules! assert_ok(
        ($result:expr) => (assert!($result.is_ok()));
    );

    #[test]
    fn offset_table_read() {
        use spec::OffsetTable;

        let mut file = tests::open("SourceSerifPro-Regular.otf");

        let mut table = OffsetTable::default();
        assert_ok!(table.read(&mut file));
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

        let mut table = CharMappingHeader::default();
        assert_ok!(table.read(&mut file));
        assert_eq!(table.version, 0);
        assert_eq!(table.numTables, 3);

        let (platforms, encodings) = ([0, 1, 3], [3, 0, 1]);
        for i in 0..3 {
            let mut table = EncodingRecord::default();
            assert_ok!(table.read(&mut file));
            assert_eq!(table.platformID, platforms[i]);
            assert_eq!(table.encodingID, encodings[i]);
        }
    }
}
