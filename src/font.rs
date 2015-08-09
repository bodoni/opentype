use std::io::{Read, Seek};

use Result;
use band::Band;
use compound::*;
use constant::*;

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

    pub char_map_header: Option<CharMapHeader>,
    pub font_header: Option<FontHeader>,
    pub max_profile: Option<MaxProfile>,
}

macro_rules! seek(
    ($band:expr, $position:expr) => (
        try!($band.seek($position as u64))
    );
);

impl Font {
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Font> {
        let mut font = Font::default();
        try!(font.read_offset_table(reader));
        try!(font.read_table_records(reader));
        Ok(font)
    }

    fn read_offset_table<T: Band>(&mut self, band: &mut T) -> Result<()> {
        try!(self.offset_table.read(band));
        if &tag!(self.offset_table.version) != CFF_FORMAT_TAG {
            raise!("the format of the font is not supported");
        }
        Ok(())
    }

    fn read_table_records<T: Band>(&mut self, band: &mut T) -> Result<()> {
        use utils::checksum;

        let mut records = vec![];
        for _ in 0..self.offset_table.numTables {
            let mut record = TableRecord::default();
            try!(record.read(band));
            records.push(record);
        }

        for record in records.iter() {
            match &tag!(record.tag) {
                CHAR_MAP_TAG => {
                    seek!(band, record.offset);
                    if !checksum(band, record, |_, chunk| chunk) {
                        raise!("the character-to-glyph mapping is corrupted");
                    }
                    seek!(band, record.offset);
                    try!(self.read_char_map(band));
                },
                FONT_HEADER_TAG => {
                    seek!(band, record.offset);
                    if !checksum(band, record, |i, chunk| if i == 2 { 0 } else { chunk }) {
                        raise!("the font header is corrupted");
                    }
                    seek!(band, record.offset);
                    try!(self.read_font_header(band));
                },
                MAX_PROFILE_TAG => {
                    seek!(band, record.offset);
                    if !checksum(band, record, |_, chunk| chunk) {
                        raise!("the maximal profile is corrupted");
                    }
                    seek!(band, record.offset);
                    try!(self.read_max_profile(band));
                },
                _ => (),
            }
        }

        self.table_records.extend(records);
        Ok(())
    }

    fn read_char_map<T: Band>(&mut self, band: &mut T) -> Result<()> {
        let top = try!(band.position());
        let mut header = CharMapHeader::default();
        try!(header.read(band));

        if header.version != CHAR_MAP_HEADER_VERSION_0_0 {
            raise!("the format of the character-to-glyph mapping header is not supported");
        }

        let mut records = vec![];
        for _ in 0..header.numTables {
            let mut record = EncodingRecord::default();
            try!(record.read(band));
            records.push(record);
        }

        for record in records.iter() {
            let offset = top + record.offset as u64;

            seek!(band, offset);
            let mut format = CharMapFormat::default();
            try!(format.read(band));
            seek!(band, offset);

            match format.version {
                4 => try!(self.read_char_map_format_4(band)),
                6 => try!(self.read_char_map_format_6(band)),
                _ => raise!("the format of a character-to-glyph mapping is not supported"),
            }
        }

        self.char_map_header = Some(header);
        Ok(())
    }

    fn read_char_map_format_4<T: Band>(&mut self, band: &mut T) -> Result<()> {
        let mut format = CharMapFormat4::default();
        try!(format.read(band));
        assert_eq!(format.format, 4);
        Ok(())
    }

    fn read_char_map_format_6<T: Band>(&mut self, band: &mut T) -> Result<()> {
        let mut format = CharMapFormat6::default();
        try!(format.read(band));
        assert_eq!(format.format, 6);
        Ok(())
    }

    fn read_font_header<T: Band>(&mut self, band: &mut T) -> Result<()> {
        let mut header = FontHeader::default();
        try!(header.read(band));
        if header.version != FONT_HEADER_VERSION_1_0 {
            raise!("the format of the font header is not supported");
        }
        if header.magicNumber != FONT_HEADER_MAGIC_NUMBER {
            raise!("the font header is corrupted");
        }
        self.font_header = Some(header);
        Ok(())
    }

    fn read_max_profile<T: Band>(&mut self, band: &mut T) -> Result<()> {
        let mut profile = MaxProfile::default();
        try!(profile.read(band));
        if profile.version != MAX_PROFILE_VERSION_0_5 {
            raise!("the format of the maximum profile is not supported");
        }
        self.max_profile = Some(profile);
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

        let header = font.font_header.as_ref().unwrap();
        assert_eq!(header.fontRevision.as_f32(), 1.014);
        assert_eq!(header.unitsPerEm, 1000);
        assert_eq!(header.macStyle, 0);

        let profile = font.max_profile.as_ref().unwrap();
        assert_eq!(profile.numGlyphs, 545);
    }
}
