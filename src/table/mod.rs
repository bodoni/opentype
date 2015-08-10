//! Tables making up a font file.

#![allow(non_snake_case)]

use std::collections::HashMap;
use std::mem;

use Result;
use band::{Band, Value};
use primitive::*;

macro_rules! itemize(
    ($code:item) => ($code);
);

macro_rules! table(
    ($structure:ident { $($field:ident ($($kind:tt)+) $(|$this:ident| $body:block)*,)+ }) => (
        declare!($structure { $($field $($kind)+,)+ });
        implement!($structure { $($field ($($kind)+) $(|$this| $body)*,)+ });
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
    ($structure:ident { $($field:ident ($($kind:tt)+) $(|$this:ident| $body:block)*,)+ }) => (
        impl Value for $structure {
            fn read<T: Band>(band: &mut T) -> Result<Self> {
                let mut value = $structure::default();
                $(value.$field = read!($structure, value, band, [$($kind)+] $(|$this| $body)*);)+
                Ok(value)
            }
        }
    );
);

macro_rules! read(
    ($structure:ident, $this:ident, $band:ident, [$kind:ty] |$that:ident| $body:block) => ({
        #[inline(always)]
        fn count($that: &$structure) -> Result<usize> $body
        let count = try!(count(&$this));
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(try!(Value::read($band)));
        }
        values
    });
    ($structure:ident, $this:ident, $band:ident, [$kind:ty]) => ({
        try!(Value::read($band))
    });
);

table!(OffsetTable {
    version       (Fixed ),
    numTables     (USHORT),
    searchRange   (USHORT),
    entrySelector (USHORT),
    rangeShift    (USHORT),
});

table!(TableRecord {
    tag      (ULONG),
    checkSum (ULONG),
    offset   (ULONG),
    length   (ULONG),
});

table!(CharMappingHeader {
    version   (USHORT),
    numTables (USHORT),
});

table!(EncodingRecord {
    platformID (USHORT),
    encodingID (USHORT),
    offset     (ULONG ),
});

pub enum CharMapping {
    Format4(CharMapping4),
    Format6(CharMapping6),
}

table!(CharMapping4 {
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
    glyphIdArray  (Vec<USHORT>) |this| { this.payload() },
});

table!(CharMapping6 {
    format       (USHORT     ),
    length       (USHORT     ),
    language     (USHORT     ),
    firstCode    (USHORT     ),
    entryCount   (USHORT     ),
    glyphIdArray (Vec<USHORT>) |this| { Ok(this.entryCount as usize) },
});

table!(FontHeader {
    version            (Fixed       ),
    fontRevision       (Fixed       ),
    checkSumAdjustment (ULONG       ),
    magicNumber        (ULONG       ),
    flags              (USHORT      ),
    unitsPerEm         (USHORT      ),
    created            (LONGDATETIME),
    modified           (LONGDATETIME),
    xMin               (SHORT       ),
    yMin               (SHORT       ),
    xMax               (SHORT       ),
    yMax               (SHORT       ),
    macStyle           (USHORT      ),
    lowestRecPPEM      (USHORT      ),
    fontDirectionHint  (SHORT       ),
    indexToLocFormat   (SHORT       ),
    glyphDataFormat    (SHORT       ),
});

pub enum MaxProfile {
    Version05(MaxProfile05),
    Version10(MaxProfile10),
}

table!(MaxProfile05 {
    version   (Fixed ),
    numGlyphs (USHORT),
});

table!(MaxProfile10 {
    version               (Fixed ),
    numGlyphs             (USHORT),
    maxPoints             (USHORT),
    maxContours           (USHORT),
    maxCompositePoints    (USHORT),
    maxCompositeContours  (USHORT),
    maxZones              (USHORT),
    maxTwilightPoints     (USHORT),
    maxStorage            (USHORT),
    maxFunctionDefs       (USHORT),
    maxInstructionDefs    (USHORT),
    maxStackElements      (USHORT),
    maxSizeOfInstructions (USHORT),
    maxComponentElements  (USHORT),
    maxComponentDepth     (USHORT),
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
        band.stay(|band| {
            try!(band.jump(self.offset as u64));
            let mut checksum: u64 = 0;
            for i in 0..length {
                checksum += process(i, try!(Value::read(band))) as u64;
            }
            Ok(self.checkSum == checksum as u32)
        })
    }
}

impl CharMapping4 {
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

    fn payload(&self) -> Result<usize> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn table_record_check() {
        use std::io::Cursor;
        use table::TableRecord;

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
