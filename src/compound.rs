//! Compound data types.

#![allow(non_snake_case)]

use std::mem;

use Result;
use band::{Band, Compound, Primitive};
use primitive::*;

macro_rules! itemize(
    ($code:item) => ($code);
);

macro_rules! compound(
    ($structure:ident { $($field:ident [$($kind:tt)+] $(|$this:ident| $body:block)*,)+ }) => (
        declare!($structure { $($field $($kind)+,)+ });
        implement!($structure { $($field [$($kind)+] $(|$this| $body)*,)+ });
    );
);

macro_rules! declare(
    ($structure:ident { $($field:ident $kind:ty,)+ }) => (
        itemize! {
            #[derive(Debug, Default)]
            pub struct $structure { $(pub $field: $kind,)+ }
        }
    );
);

macro_rules! implement(
    ($structure:ident { $($field:ident [$($kind:tt)+] $(|$this:ident| $body:block)*,)+ }) => (
        impl Compound for $structure {
            fn read<T: Band>(&mut self, band: &mut T) -> Result<()> {
                $(self.$field = read!($structure, self, band, $($kind)+ $(|$this| $body)*);)+
                Ok(())
            }
        }
    );
);

macro_rules! read(
    ($structure:ident, $this:ident, $band:ident, Vec<$kind:ty> |$that:ident| $body:block) => ({
        #[allow(unused_variables)]
        fn count($that: &$structure) -> usize $body
        let _ = count($this);
        vec![]
    });
    ($structure:ident, $this:ident, $band:ident, $kind:ty) => ({
        try!(Primitive::read($band))
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

pub enum MaxProfile {
    Version05(MaxProfileVersion05),
    Version10(MaxProfileVersion10),
}

compound!(MaxProfileVersion05 {
    version   [Fixed ],
    numGlyphs [USHORT],
});

compound!(MaxProfileVersion10 {
    version               [Fixed ],
    numGlyphs             [USHORT],
    maxPoints             [USHORT],
    maxContours           [USHORT],
    maxCompositePoints    [USHORT],
    maxCompositeContours  [USHORT],
    maxZones              [USHORT],
    maxTwilightPoints     [USHORT],
    maxStorage            [USHORT],
    maxFunctionDefs       [USHORT],
    maxInstructionDefs    [USHORT],
    maxStackElements      [USHORT],
    maxSizeOfInstructions [USHORT],
    maxComponentElements  [USHORT],
    maxComponentDepth     [USHORT],
});

impl TableRecord {
    #[doc(hidden)]
    pub fn check<T, F>(&self, band: &mut T, process: F) -> Result<bool>
        where T: Band, F: Fn(usize, ULONG) -> ULONG
    {
        let length = {
            let size = mem::size_of::<ULONG>();
            ((self.length as usize + size - 1) & !(size - 1)) / size
        };
        band.save(|band| {
            try!(band.jump(self.offset as u64));
            let mut checksum: u64 = 0;
            for i in 0..length {
                checksum += process(i, try!(ULONG::read(band))) as u64;
            }
            Ok(self.checkSum == checksum as u32)
        })
    }
}

#[cfg(test)]
mod tests {
    use tests;

    #[test]
    fn char_map() {
        use band::Compound;
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

    #[test]
    fn offset_table() {
        use band::Compound;
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
    fn table_record() {
        use compound::TableRecord;
        use std::io::Cursor;

        macro_rules! check(
            ($length:expr, $checksum:expr, $data:expr) => ({
                let data: &[u8] = $data;
                let mut reader = Cursor::new(data);
                let table = TableRecord {
                    length: $length,
                    checkSum: $checksum,
                    .. TableRecord::default()
                };
                table.check(&mut reader, |_, chunk| chunk).unwrap()
            })
        );

        assert!(!check!(3 * 4, 1 + 2 + 4, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
        assert!( check!(3 * 4, 1 + 2 + 3, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
