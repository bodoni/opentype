use std::io::{Read, Seek};

use Result;
use band::{Band, Value};
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
        let mut font = Font { offset_table: try!(Value::read(reader)), .. Font::default() };
        if &tag!(font.offset_table.version) != b"OTTO" {
            raise!("the format of a font is not supported");
        }
        try!(font.read_table_records(reader));
        Ok(font)
    }

    fn read_table_records<T: Band>(&mut self, band: &mut T) -> Result<()> {
        let mut records = vec![];
        for _ in 0..self.offset_table.numTables {
            records.push(try!(TableRecord::read(band)));
        }
        for record in records.iter() {
            match &tag!(record.tag) {
                b"cmap" => {
                    if !try!(record.check(band, |_, chunk| chunk)) {
                        raise!("the character-to-glyph mappings are corrupted");
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
        let header = try!(CharMappingHeader::read(band));
        if header.version != VERSION_0_0 {
            raise!("the format of the character-to-glyph mapping header is not supported");
        }
        let mut records = vec![];
        for _ in 0..header.numTables {
            records.push(try!(EncodingRecord::read(band)));
        }
        let mut mappings = vec![];
        for record in records.iter() {
            try!(band.jump(top + record.offset as u64));
            mappings.push(match try!(band.peek::<USHORT>()) {
                4 => CharMapping::Format4(try!(Value::read(band))),
                6 => CharMapping::Format6(try!(Value::read(band))),
                _ => raise!("the format of a character-to-glyph mapping is not supported"),
            });
        }
        self.char_mapping_header = Some(header);
        self.char_mappings.extend(mappings);
        self.encoding_records.extend(records);

        Ok(())
    }

    fn read_font_header<T: Band>(&mut self, band: &mut T) -> Result<()> {
        const MAGIC_NUMBER: ULONG = 0x5F0F3CF5;
        const VERSION_1_0: Fixed = Fixed(0x00010000);

        let header = try!(FontHeader::read(band));
        if header.version != VERSION_1_0 {
            raise!("the format of the font header is not supported");
        }
        if header.magicNumber != MAGIC_NUMBER {
            raise!("the font header is malformed");
        }
        self.font_header = Some(header);

        Ok(())
    }

    fn read_max_profile<T: Band>(&mut self, band: &mut T) -> Result<()> {
        const VERSION_0_5: Fixed = Fixed(0x00005000);
        const VERSION_1_0: Fixed = Fixed(0x00010000);

        self.max_profile = Some(match try!(band.peek::<Fixed>()) {
            VERSION_0_5 => MaxProfile::Version05(try!(Value::read(band))),
            VERSION_1_0 => MaxProfile::Version10(try!(Value::read(band))),
            _ => {
                raise!("the format of the maximum profile is not supported");
            },
        });

        Ok(())
    }
}
