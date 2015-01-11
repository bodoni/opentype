use std::default::Default;
use std::io::{Reader, Seek};

use Result;

use spec::{self, Table, TableRecord};

use spec::OffsetTable;

use spec::{CharMappingHeader, EncodingRecord};
use spec::{CharMappingFormat, CharMappingFormat4, CharMappingFormat6};
use spec::FontHeader;
use spec::MaximumProfile;

macro_rules! tag(
    ($value:expr) => (unsafe {
        let value: [u8; 4] = ::std::mem::transmute($value);
        [value[3], value[2], value[1], value[0]].as_slice()
    })
);

macro_rules! seek(
    ($reader:expr, $offset:expr) => (
        try!($reader.seek($offset as i64, ::std::io::SeekSet))
    );
);

#[derive(Default)]
pub struct Font {
    pub offset_table: OffsetTable,

    pub char_mapping_header: CharMappingHeader,
    pub font_header: FontHeader,
    pub maximum_profile: MaximumProfile,
}

impl Font {
    #[inline]
    fn new() -> Font {
        Default::default()
    }

    fn read<R: Reader + Seek>(&mut self, reader: &mut R) -> Result<()> {
        try!(self.read_offset_table(reader));
        try!(self.read_table_records(reader));

        Ok(())
    }

    fn read_offset_table(&mut self, reader: &mut Reader) -> Result<()> {
        try!(self.offset_table.read(reader));

        if tag!(self.offset_table.version) != spec::CFF_FORMAT_TAG {
            raise!("the format of the font is not supported");
        }

        Ok(())
    }

    fn read_table_records<R: Reader + Seek>(&mut self, reader: &mut R) -> Result<()> {
        use utils::checksum;

        let mut records = vec![];

        for _ in range(0, self.offset_table.numTables) {
            let mut table: TableRecord = Default::default();
            try!(table.read(reader));
            records.push(table);
        }

        for record in records.iter() {
            match tag!(record.tag) {
                spec::CHAR_MAPPING_TAG => {
                    seek!(reader, record.offset);
                    if !checksum(reader, record, |_, chunk| chunk) {
                        raise!("the character-to-glyph mapping is corrupted");
                    }

                    seek!(reader, record.offset);
                    try!(self.read_char_mapping(reader));
                },
                spec::FONT_HEADER_TAG => {
                    seek!(reader, record.offset);
                    if !checksum(reader, record, |i, chunk| if i == 2 { 0 } else { chunk }) {
                        raise!("the font header is corrupted");
                    }

                    seek!(reader, record.offset);
                    try!(self.read_font_header(reader));
                },
                spec::MAXIMAL_PROFILE_TAG => {
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

    fn read_char_mapping<R: Reader + Seek>(&mut self, reader: &mut R) -> Result<()> {
        let top = try!(reader.tell());
        try!(self.char_mapping_header.read(reader));

        if self.char_mapping_header.version != spec::CHAR_MAPPING_HEADER_VERSION_0_0 {
            raise!("the format of the character-to-glyph mapping header is not supported");
        }

        let mut records = vec![];

        for _ in range(0, self.char_mapping_header.numTables as usize) {
            let mut table: EncodingRecord = Default::default();
            try!(table.read(reader));
            records.push(table);
        }

        for record in records.iter() {
            let offset = top + record.offset as u64;

            seek!(reader, offset);
            let mut table: CharMappingFormat = Default::default();
            try!(table.read(reader));
            seek!(reader, offset);

            match table.version {
                4 => try!(self.read_char_mapping_format_4(reader)),
                6 => try!(self.read_char_mapping_format_6(reader)),
                _ => raise!("the format of a character-to-glyph mapping is not supported"),
            }
        }

        Ok(())
    }

    fn read_char_mapping_format_4(&mut self, reader: &mut Reader) -> Result<()> {
        let mut table: CharMappingFormat4 = Default::default();
        try!(table.read(reader));
        assert_eq!(table.format, 4);

        Ok(())
    }

    fn read_char_mapping_format_6(&mut self, reader: &mut Reader) -> Result<()> {
        let mut table: CharMappingFormat6 = Default::default();
        try!(table.read(reader));
        assert_eq!(table.format, 6);

        Ok(())
    }

    fn read_font_header(&mut self, reader: &mut Reader) -> Result<()> {
        try!(self.font_header.read(reader));

        if self.font_header.version != spec::FONT_HEADER_VERSION_1_0 {
            raise!("the format of the font header is not supported");
        }

        if self.font_header.magicNumber != spec::FONT_HEADER_MAGIC_NUMBER {
            raise!("the font header is corrupted");
        }

        Ok(())
    }

    fn read_maximum_profile(&mut self, reader: &mut Reader) -> Result<()> {
        try!(self.maximum_profile.read(reader));

        if self.maximum_profile.version != spec::MAXIMAL_PROFILE_VERSION_0_5 {
            raise!("the format of the maximum profile is not supported");
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[inline]
pub fn read<R: Reader + Seek>(reader: &mut R) -> Result<Font> {
    let mut font = Font::new();
    try!(font.read(reader));
    Ok(font)
}

#[cfg(test)]
mod tests {
    use date::Date;

    #[test]
    fn read() {
        macro_rules! assert_date(
            ($seconds:expr, $year:expr, $month:expr, $day:expr) => (
                assert_eq!(Date::at_utc_1904($seconds), Date::new($year, $month, $day));
            );
        );

        let mut file = ::tests::open("SourceSerifPro-Regular.otf");
        let font = ::font::read(&mut file).unwrap();

        assert_eq!(font.font_header.fontRevision.to_f32(), 1.014);
        assert_eq!(font.font_header.unitsPerEm, 1000);
        assert_date!(font.font_header.created, 2014, 4, 27);
        assert_date!(font.font_header.modified, 2014, 4, 27);
        assert_eq!(font.font_header.macStyle, 0);

        assert_eq!(font.maximum_profile.numGlyphs, 545);
    }
}
