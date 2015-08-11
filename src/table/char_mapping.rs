use std::collections::HashMap;

use Result;
use primitive::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CharMapping {
    pub header: CharMappingHeader,
    pub records: Vec<CharMappingRecord>,
    pub encodings: Vec<CharMappingEncoding>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharMappingEncoding {
    Format4(CharMappingEncoding4),
    Format6(CharMappingEncoding6),
}

spec! {
    #[derive(Copy)]
    pub CharMappingHeader {
        version   (USHORT),
        numTables (USHORT),
    }
}

spec! {
    #[derive(Copy)]
    pub CharMappingRecord {
        platformID (USHORT),
        encodingID (USHORT),
        offset     (ULONG ),
    }
}

spec! {
    pub CharMappingEncoding4 {
        format        (USHORT     ),
        length        (USHORT     ),
        language      (USHORT     ),
        segCountX2    (USHORT     ),
        searchRange   (USHORT     ),
        entrySelector (USHORT     ),
        rangeShift    (USHORT     ),
        endCode       (Vec<USHORT>) |this| { Ok(this.segments()) },
        reservedPad   (USHORT     ),
        startCode     (Vec<USHORT>) |this| { Ok(this.segments()) },
        idDelta       (Vec<SHORT> ) |this| { Ok(this.segments()) },
        idRangeOffset (Vec<USHORT>) |this| { Ok(this.segments()) },
        glyphIdArray  (Vec<USHORT>) |this| { this.mappings() },
    }
}

spec! {
    pub CharMappingEncoding6 {
        format       (USHORT     ),
        length       (USHORT     ),
        language     (USHORT     ),
        firstCode    (USHORT     ),
        entryCount   (USHORT     ),
        glyphIdArray (Vec<USHORT>) |this| { Ok(this.entryCount as usize) },
    }
}

impl CharMappingEncoding4 {
    pub fn mapping(&self) -> HashMap<USHORT, USHORT> {
        let segments = self.segments();

        let mut map = HashMap::new();
        for i in 0..(segments - 1) {
            let startCode = self.startCode[i];
            let idDelta = self.idDelta[i];
            let idRangeOffset = self.idRangeOffset[i];
            for j in startCode..(self.endCode[i] + 1) {
                let index = if idRangeOffset > 0 {
                    let offset = (idRangeOffset / 2 + (j - startCode)) - (segments - i) as USHORT;
                    self.glyphIdArray[offset as usize]
                } else {
                    (idDelta + j as SHORT) as USHORT
                };
                map.insert(j, index);
            }
        }

        map
    }

    fn mappings(&self) -> Result<usize> {
        let segments = self.segments();

        if segments == 0 {
            raise!("a character-to-glyph mapping has no segments");
        }
        if self.startCode[segments - 1] != 0xFFFF || self.endCode[segments - 1] != 0xFFFF {
            raise!("a character-to-glyph mapping is malformed");
        }

        let mut count = 0;
        for i in 0..(segments - 1) {
            let startCode = self.startCode[i];
            let idRangeOffset = self.idRangeOffset[i];
            for j in startCode..(self.endCode[i] + 1) {
                if idRangeOffset > 0 {
                    let offset = (idRangeOffset / 2 + (j - startCode)) - (segments - i) as USHORT;
                    if offset >= count {
                        count = offset + 1;
                    }
                }
            }
        }

        Ok(count as usize)
    }

    #[inline]
    fn segments(&self) -> usize {
        self.segCountX2 as usize / 2
    }
}
