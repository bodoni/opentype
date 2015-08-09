use std::io::{Read, Seek};

use Result;
use band::{Band, Compound};
use compound::*;
use primitive::*;

macro_rules! tag(
    ($value:expr) => (unsafe {
        let mut value: [u8; 4] = ::std::mem::transmute($value);
        value.swap(0, 3);
        value.swap(1, 2);
        value
    })
);

/// A font.
#[derive(Default)]
pub struct Font {
    pub offset_table: OffsetTable,
    pub table_records: Vec<TableRecord>,

    pub char_mapping_header: Option<CharMappingHeader>,
    pub encoding_records: Vec<EncodingRecord>,
    pub char_mappings: Vec<CharMapping>,

    pub font_header: Option<FontHeader>,

    pub max_profile: Option<MaxProfile>,
}

impl Font {
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Font> {
        let mut font = Font::default();
        try!(font.read_offset_table(reader));
        try!(font.read_table_records(reader));
        Ok(font)
    }

    fn read_offset_table<T: Band>(&mut self, band: &mut T) -> Result<()> {
        try!(self.offset_table.read(band));
        if &tag!(self.offset_table.version) != b"OTTO" {
            raise!("the format of the font is not supported");
        }
        Ok(())
    }

    fn read_table_records<T: Band>(&mut self, band: &mut T) -> Result<()> {
        let mut records = vec![];
        for _ in 0..self.offset_table.numTables {
            let mut record = TableRecord::default();
            try!(record.read(band));
            records.push(record);
        }

        for record in records.iter() {
            match &tag!(record.tag) {
                b"cmap" => {
                    if !try!(record.check(band, |_, chunk| chunk)) {
                        raise!("the character-to-glyph mapping is corrupted");
                    }
                    try!(band.jump(record.offset as u64));
                    try!(self.read_char_mappings(band));
                },
                b"head" => {
                    if !try!(record.check(band, |i, chunk| if i == 2 { 0 } else { chunk })) {
                        raise!("the font header is corrupted");
                    }
                    try!(band.jump(record.offset as u64));
                    try!(self.read_font_header(band));
                },
                b"maxp" => {
                    if !try!(record.check(band, |_, chunk| chunk)) {
                        raise!("the maximal profile is corrupted");
                    }
                    try!(band.jump(record.offset as u64));
                    try!(self.read_max_profile(band));
                },
                _ => (),
            }
        }

        self.table_records.extend(records);
        Ok(())
    }

    fn read_char_mappings<T: Band>(&mut self, band: &mut T) -> Result<()> {
        const VERSION_0_0: USHORT = 0;

        let top = try!(band.position());
        let mut header = CharMappingHeader::default();
        try!(header.read(band));

        if header.version != VERSION_0_0 {
            raise!("the format of the character-to-glyph mapping header is not supported");
        }

        let mut records = vec![];
        for _ in 0..header.numTables {
            let mut record = EncodingRecord::default();
            try!(record.read(band));
            records.push(record);
        }

        let mut mappings = vec![];
        for record in records.iter() {
            try!(band.jump(top + record.offset as u64));
            match try!(band.peek::<USHORT>()) {
                4 => {
                    let mut mapping = CharMappingFormat4::default();
                    try!(mapping.read(band));
                    mappings.push(CharMapping::Format4(mapping));
                },
                6 => {
                    let mut mapping = CharMappingFormat6::default();
                    try!(mapping.read(band));
                    mappings.push(CharMapping::Format6(mapping));
                },
                _ => raise!("the format of a character-to-glyph mapping is not supported"),
            }
        }

        self.char_mapping_header = Some(header);
        self.char_mappings.extend(mappings);
        self.encoding_records.extend(records);
        Ok(())
    }

    fn read_font_header<T: Band>(&mut self, band: &mut T) -> Result<()> {
        const MAGIC_NUMBER: ULONG = 0x5F0F3CF5;
        const VERSION_1_0: Fixed = Fixed(0x00010000);

        let mut header = FontHeader::default();
        try!(header.read(band));
        if header.version != VERSION_1_0 {
            raise!("the format of the font header is not supported");
        }
        if header.magicNumber != MAGIC_NUMBER {
            raise!("the font header is corrupted");
        }
        self.font_header = Some(header);
        Ok(())
    }

    fn read_max_profile<T: Band>(&mut self, band: &mut T) -> Result<()> {
        const VERSION_0_5: Fixed = Fixed(0x00005000);
        const VERSION_1_0: Fixed = Fixed(0x00010000);

        match try!(band.peek::<Fixed>()) {
            VERSION_0_5 => {
                let mut profile = MaxProfileVersion05::default();
                try!(profile.read(band));
                self.max_profile = Some(MaxProfile::Version05(profile));
            },
            VERSION_1_0 => {
                let mut profile = MaxProfileVersion10::default();
                try!(profile.read(band));
                self.max_profile = Some(MaxProfile::Version10(profile));
            },
            _ => {
                raise!("the format of the maximum profile is not supported");
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use compound::MaxProfile;
    use super::Font;
    use tests;

    #[test]
    fn read() {
        let mut file = tests::open("SourceSerifPro-Regular.otf");
        let font = Font::read(&mut file).unwrap();

        match font.font_header {
            Some(ref header) => {
                assert_eq!(header.fontRevision.as_f32(), 1.014);
                assert_eq!(header.unitsPerEm, 1000);
                assert_eq!(header.macStyle, 0);
            },
            _ => unreachable!(),
        }

        match font.max_profile {
            Some(MaxProfile::Version05(ref profile)) => {
                assert_eq!(profile.numGlyphs, 545);
            },
            _ => unreachable!(),
        }
    }
}
