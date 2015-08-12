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
        version   (UShort),
        numTables (UShort),
    }
}

spec! {
    #[derive(Copy)]
    pub CharMappingRecord {
        platformID (UShort),
        encodingID (UShort),
        offset     (ULong ),
    }
}

spec! {
    pub CharMappingEncoding4 {
        format        (UShort     ),
        length        (UShort     ),
        language      (UShort     ),
        segCountX2    (UShort     ),
        searchRange   (UShort     ),
        entrySelector (UShort     ),
        rangeShift    (UShort     ),
        endCode       (Vec<UShort>) |band, this| { read_vector!(band, this.segments()) },
        reservedPad   (UShort     ),
        startCode     (Vec<UShort>) |band, this| { read_vector!(band, this.segments()) },
        idDelta       (Vec<Short> ) |band, this| { read_vector!(band, this.segments()) },
        idRangeOffset (Vec<UShort>) |band, this| { read_vector!(band, this.segments()) },
        glyphIdArray  (Vec<UShort>) |band, this| { read_vector!(band, try!(this.array_length())) },
    }
}

spec! {
    pub CharMappingEncoding6 {
        format       (UShort     ),
        length       (UShort     ),
        language     (UShort     ),
        firstCode    (UShort     ),
        entryCount   (UShort     ),
        glyphIdArray (Vec<UShort>) |band, this| { read_vector!(band, this.entryCount) },
    }
}

impl CharMappingEncoding4 {
    pub fn mapping(&self) -> HashMap<UShort, UShort> {
        let segments = self.segments();

        let mut map = HashMap::new();
        for i in 0..(segments - 1) {
            let startCode = self.startCode[i];
            let idDelta = self.idDelta[i];
            let idRangeOffset = self.idRangeOffset[i];
            for j in startCode..(self.endCode[i] + 1) {
                let index = if idRangeOffset > 0 {
                    let offset = (idRangeOffset / 2 + (j - startCode)) - (segments - i) as UShort;
                    self.glyphIdArray[offset as usize]
                } else {
                    (idDelta + j as Short) as UShort
                };
                map.insert(j, index);
            }
        }

        map
    }

    fn array_length(&self) -> Result<usize> {
        let segments = self.segments();

        if segments == 0 {
            raise!("a character-to-glyph mapping has no segments");
        }
        if self.startCode[segments - 1] != 0xFFFF || self.endCode[segments - 1] != 0xFFFF {
            raise!("a character-to-glyph mapping is malformed");
        }

        let mut length = 0;
        for i in 0..(segments - 1) {
            let startCode = self.startCode[i];
            let idRangeOffset = self.idRangeOffset[i];
            for j in startCode..(self.endCode[i] + 1) {
                if idRangeOffset > 0 {
                    let end = (idRangeOffset / 2 + (j - startCode)) - (segments - i) as UShort + 1;
                    if end > length {
                        length = end;
                    }
                }
            }
        }

        Ok(length as usize)
    }

    #[inline]
    fn segments(&self) -> usize {
        self.segCountX2 as usize / 2
    }
}
