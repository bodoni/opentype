use std::io::{Read, Seek};

use Result;
use band::Band;
use compound::Compound;
use consts;
use structs::*;

macro_rules! tag(
    ($value:expr) => (unsafe {
        let mut value: [u8; 4] = ::std::mem::transmute($value);
        value.swap(0, 3);
        value.swap(1, 2);
        value
    })
);

#[derive(Default)]
pub struct Font {
    pub offset_table: OffsetTable,
    pub char_mapping_header: CharMappingHeader,
    pub font_header: FontHeader,
    pub maximum_profile: MaximumProfile,
}

macro_rules! seek(
    ($reader:expr, $position:expr) => (
        try!($reader.seek($position as u64))
    );
);

impl Font {
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Font> {
        let mut font = Font::default();
        try!(font.read_offset_table(reader));
        try!(font.read_table_records(reader));
        Ok(font)
    }

    fn read_offset_table<T: Band>(&mut self, reader: &mut T) -> Result<()> {
        try!(self.offset_table.read(reader));

        if &tag!(self.offset_table.version) != consts::CFF_FORMAT_TAG {
            raise!("the format of the font is not supported");
        }

        Ok(())
    }

    fn read_table_records<T: Band>(&mut self, reader: &mut T) -> Result<()> {
        use utils::checksum;

        let mut records = vec![];

        for _ in 0..self.offset_table.numTables {
            let mut record = TableRecord::default();
            try!(record.read(reader));
            records.push(record);
        }

        for record in records.iter() {
            match &tag!(record.tag) {
                consts::CHAR_MAPPING_TAG => {
                    seek!(reader, record.offset);
                    if !checksum(reader, record, |_, chunk| chunk) {
                        raise!("the character-to-glyph mapping is corrupted");
                    }

                    seek!(reader, record.offset);
                    try!(self.read_char_mapping(reader));
                },
                consts::FONT_HEADER_TAG => {
                    seek!(reader, record.offset);
                    if !checksum(reader, record, |i, chunk| if i == 2 { 0 } else { chunk }) {
                        raise!("the font header is corrupted");
                    }

                    seek!(reader, record.offset);
                    try!(self.read_font_header(reader));
                },
                consts::MAXIMAL_PROFILE_TAG => {
                    seek!(reader, record.offset);
                    if !checksum(reader, record, |_, chunk| chunk) {
                        raise!("the maximal profile is corrupted");
                    }

                    seek!(reader, record.offset);
                    try!(self.read_maximum_profile(reader));
                },
                _ => (),
            }
        }

        Ok(())
    }

    fn read_char_mapping<T: Band>(&mut self, reader: &mut T) -> Result<()> {
        let top = try!(reader.position());
        try!(self.char_mapping_header.read(reader));

        if self.char_mapping_header.version != consts::CHAR_MAPPING_HEADER_VERSION_0_0 {
            raise!("the format of the character-to-glyph mapping header is not supported");
        }

        let mut records = vec![];

        for _ in 0..self.char_mapping_header.numTables {
            let mut record = EncodingRecord::default();
            try!(record.read(reader));
            records.push(record);
        }

        for record in records.iter() {
            let offset = top + record.offset as u64;

            seek!(reader, offset);
            let mut format = CharMappingFormat::default();
            try!(format.read(reader));
            seek!(reader, offset);

            match format.version {
                4 => try!(self.read_char_mapping_format_4(reader)),
                6 => try!(self.read_char_mapping_format_6(reader)),
                _ => raise!("the format of a character-to-glyph mapping is not supported"),
            }
        }

        Ok(())
    }

    fn read_char_mapping_format_4<T: Band>(&mut self, reader: &mut T) -> Result<()> {
        let mut format = CharMappingFormat4::default();
        try!(format.read(reader));
        assert_eq!(format.format, 4);

        Ok(())
    }

    fn read_char_mapping_format_6<T: Band>(&mut self, reader: &mut T) -> Result<()> {
        let mut format = CharMappingFormat6::default();
        try!(format.read(reader));
        assert_eq!(format.format, 6);

        Ok(())
    }

    fn read_font_header<T: Band>(&mut self, reader: &mut T) -> Result<()> {
        try!(self.font_header.read(reader));

        if self.font_header.version != consts::FONT_HEADER_VERSION_1_0 {
            raise!("the format of the font header is not supported");
        }

        if self.font_header.magicNumber != consts::FONT_HEADER_MAGIC_NUMBER {
            raise!("the font header is corrupted");
        }

        Ok(())
    }

    fn read_maximum_profile<T: Band>(&mut self, reader: &mut T) -> Result<()> {
        try!(self.maximum_profile.read(reader));

        if self.maximum_profile.version != consts::MAXIMAL_PROFILE_VERSION_0_5 {
            raise!("the format of the maximum profile is not supported");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Font;
    use tests;

    #[test]
    fn read() {
        let mut file = tests::open("SourceSerifPro-Regular.otf");
        let font = Font::read(&mut file).unwrap();

        assert_eq!(font.font_header.fontRevision.as_f32(), 1.014);
        assert_eq!(font.font_header.unitsPerEm, 1000);
        assert_eq!(font.font_header.macStyle, 0);

        assert_eq!(font.maximum_profile.numGlyphs, 545);
    }
}
