//! Compound data types.

#![allow(non_snake_case)]

use Result;
use band::Band;
use primitive::*;

#[doc(hidden)]
pub trait Compound {
    fn read<T: Band>(&mut self, &mut T) -> Result<()>;
}

macro_rules! itemize(
    ($code:item) => ($code);
);

macro_rules! compound(
    ($structure:ident { $($field:ident [$($kind:tt)+] $(|$this:ident| $body:block)*,)+ }) => (
        declare_compound!($structure { $($field $($kind)+,)+ });
        implement_compound!($structure { $($field [$($kind)+] $(|$this| $body)*,)+ });
    );
);

macro_rules! declare_compound(
    ($structure:ident { $($field:ident $kind:ty,)+ }) => (
        itemize! {
            #[derive(Default)]
            pub struct $structure { $(pub $field: $kind,)+ }
        }
    );
);

macro_rules! implement_compound(
    ($structure:ident { $($field:ident [$($kind:tt)+] $(|$this:ident| $body:block)*,)+ }) => (
        impl Compound for $structure {
            fn read<T: Band>(&mut self, band: &mut T) -> Result<()> {
                $(self.$field = read_field!($structure, self, band, $($kind)+ $(|$this| $body)*);)+
                Ok(())
            }
        }
    );
);

macro_rules! read_field(
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

compound!(OffsetTable {
    version       [Fixed ],
    numTables     [USHORT],
    searchRange   [USHORT],
    entrySelector [USHORT],
    rangeShift    [USHORT],
});

compound!(TableRecord {
    tag      [ULONG],
    checkSum [ULONG],
    offset   [ULONG],
    length   [ULONG],
});

compound!(CharMapHeader {
    version   [USHORT],
    numTables [USHORT],
});

compound!(EncodingRecord {
    platformID [USHORT],
    encodingID [USHORT],
    offset     [ULONG ],
});

compound!(CharMapFormat {
    version [USHORT],
});

compound!(CharMapFormat4 {
    format        [USHORT     ],
    length        [USHORT     ],
    language      [USHORT     ],
    segCountX2    [USHORT     ],
    searchRange   [USHORT     ],
    entrySelector [USHORT     ],
    rangeShift    [USHORT     ],
    endCount      [Vec<USHORT>] |this| { 0 },
    reservedPad   [USHORT     ],
    startCount    [Vec<USHORT>] |this| { 0 },
    idDelta       [Vec<SHORT> ] |this| { 0 },
    idRangeOffset [Vec<USHORT>] |this| { 0 },
    glyphIdArray  [Vec<USHORT>] |this| { 0 },
});

compound!(CharMapFormat6 {
    format       [USHORT     ],
    length       [USHORT     ],
    language     [USHORT     ],
    firstCode    [USHORT     ],
    entryCount   [USHORT     ],
    glyphIdArray [Vec<USHORT>] |this| { 0 },
});

compound!(FontHeader {
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

compound!(MaxProfile {
    version   [Fixed ],
    numGlyphs [USHORT],
});

#[cfg(test)]
mod tests {
    use compound::Compound;
    use tests;

    #[test]
    fn offset_table_read() {
        use compound::OffsetTable;

        let mut file = tests::open("SourceSerifPro-Regular.otf");
        let mut table = OffsetTable::default();
        table.read(&mut file).unwrap();

        assert_eq!(table.version.0, 0x4f54544f);
        assert_eq!(table.numTables, 12);
        assert_eq!(table.searchRange, 8 * 16);
        assert_eq!(table.entrySelector, 3);
        assert_eq!(table.rangeShift, table.numTables * 16 - table.searchRange);
    }

    #[test]
    fn char_mapping_read() {
        use compound::{CharMapHeader, EncodingRecord};
        use std::io::{Seek, SeekFrom};

        let mut file = tests::open("SourceSerifPro-Regular.otf");
        file.seek(SeekFrom::Start(15668)).unwrap();
        let mut header = CharMapHeader::default();
        header.read(&mut file).unwrap();

        assert_eq!(header.version, 0);
        assert_eq!(header.numTables, 3);

        let (platforms, encodings) = ([0, 1, 3], [3, 0, 1]);
        for i in 0..3 {
            let mut record = EncodingRecord::default();
            record.read(&mut file).unwrap();

            assert_eq!(record.platformID, platforms[i]);
            assert_eq!(record.encodingID, encodings[i]);
        }
    }
}
