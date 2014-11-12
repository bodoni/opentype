use std::default::Default;
use std::io::{Reader, Seek};

use Result;
use spec::{mod, FontHeader, OffsetTable, MaximumProfile, Table, TableRecord};

macro_rules! tag(
    ($value:expr) => (unsafe {
        let value: [u8, ..4] = ::std::mem::transmute($value);
        [value[3], value[2], value[1], value[0]]
    })
)

#[deriving(Default)]
pub struct Font {
    pub offset_table: OffsetTable,
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
            raise!("the font format is not supported");
        }

        Ok(())
    }

    fn read_table_records<R: Reader + Seek>(&mut self, reader: &mut R) -> Result<()> {
        use utils::checksum;

        macro_rules! seek(
            ($reader:expr, $offset:expr) => (
                try!($reader.seek($offset as i64, ::std::io::SeekSet))
            );
        )

        let mut table_records = vec![];

        for _ in range(0, self.offset_table.numTables) {
            let mut table: TableRecord = Default::default();
            try!(table.read(reader));
            table_records.push(table);
        }

        for record in table_records.iter() {
            match tag!(record.tag) {
                spec::FONT_HEADER_TAG => {
                    seek!(reader, record.offset);
                    if !checksum(reader, record, |i, chunk| if i == 2 { 0 } else { chunk }) {
                        raise!("the file is corrupted");
                    }

                    seek!(reader, record.offset);
                    try!(self.read_font_header(reader));
                },
                spec::MAXIMAL_PROFILE_TAG => {
                    seek!(reader, record.offset);
                    if !checksum(reader, record, |_, chunk| chunk) {
                        raise!("the file is corrupted");
                    }

                    seek!(reader, record.offset);
                    try!(self.read_maximum_profile(reader));
                },
                _ => (),
            }
        }

        Ok(())
    }

    fn read_font_header(&mut self, reader: &mut Reader) -> Result<()> {
        try!(self.font_header.read(reader));

        if self.font_header.version != spec::FONT_HEADER_VERSION_1_0 {
            raise!("the format of the font header table is not supported");
        }

        if self.font_header.magicNumber != spec::FONT_HEADER_MAGIC_NUMBER {
            raise!("the font header is currupted");
        }

        Ok(())
    }

    fn read_maximum_profile(&mut self, reader: &mut Reader) -> Result<()> {
        try!(self.maximum_profile.read(reader));

        if self.maximum_profile.version != spec::MAXIMAL_PROFILE_VERSION_0_5 {
            raise!("the format of the maximum profile table is supported");
        }

        Ok(())
    }
}

#[allow(dead_code)]
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
        let mut file = ::tests::open("SourceSerifPro-Regular.otf");
        let font = ::font::read(&mut file).unwrap();

        assert_eq!(font.font_header.fontRevision.to_f32(), 1.014);
        assert_eq!(font.font_header.unitsPerEm, 1000);
        assert_eq!(Date::at_utc_1904(font.font_header.created), Date::new(2014, 4, 27));
        assert_eq!(Date::at_utc_1904(font.font_header.modified), Date::new(2014, 4, 27));
        assert_eq!(font.font_header.macStyle, 0);

        assert_eq!(font.maximum_profile.numGlyphs, 545);
    }
}
