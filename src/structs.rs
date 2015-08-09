//! Structures.

#![allow(non_snake_case)]

use types::*;

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

compound!(CharMappingHeader {
    version   [USHORT],
    numTables [USHORT],
});

compound!(EncodingRecord {
    platformID [USHORT],
    encodingID [USHORT],
    offset     [ULONG ],
});

compound!(CharMappingFormat {
    version [USHORT],
});

compound!(CharMappingFormat4 {
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

compound!(CharMappingFormat6 {
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

compound!(MaximumProfile {
    version   [Fixed ],
    numGlyphs [USHORT],
});

#[cfg(test)]
mod tests {
    use compound::Compound;
    use tests;

    #[test]
    fn offset_table_read() {
        use structs::OffsetTable;

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
        use std::io::{Seek, SeekFrom};
        use structs::{CharMappingHeader, EncodingRecord};

        let mut file = tests::open("SourceSerifPro-Regular.otf");
        file.seek(SeekFrom::Start(15668)).unwrap();
        let mut header = CharMappingHeader::default();
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
