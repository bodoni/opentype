#![crate_name = "opentype"]
#![crate_type = "rlib"]

#![feature(globs, macro_rules)]

extern crate date;

use std::{default, fmt, io};
use date::Date;

pub use table::Table;
pub use style::Style;

pub mod spec;
mod style;
mod table;

macro_rules! raise(
    () => (return Err(io::IoError {
        kind: io::OtherIoError,
        desc: "Cannot parse the file.",
        detail: None,
    }));
    ($message:expr) => (return Err(io::IoError {
        kind: io::OtherIoError,
        desc: $message,
        detail: None,
    }));
)

#[deriving(PartialEq)]
pub enum Format {
    CFF,
}

impl default::Default for Format {
    fn default() -> Format { CFF }
}

impl fmt::Show for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter)
        -> Result<(), fmt::FormatError> {

        match *self {
            CFF => write!(formatter, "CFF"),
        }
    }
}

#[deriving(Default, Show)]
pub struct Font {
    pub format: Format,
    pub version: f32,
    pub units_per_em: u16,
    pub created_on: Date,
    pub updated_on: Date,
    pub style: Style,
    pub glyph_count: u16,
}

impl Font {
    fn parse(&mut self, stream: &mut io::File) -> io::IoResult<()> {
        let offset_table: spec::OffsetTable = try!(spec::read(stream));

        match offset_table.tag {
            spec::CFF_FORMAT_TAG => try!(self.parse_cff(stream, &offset_table)),
            _ => raise!("The format is not supported.")
        }

        Ok(())
    }

    fn parse_cff(&mut self, stream: &mut io::File,
        offset_table: &spec::OffsetTable) -> io::IoResult<()> {

        let mut table_records: Vec<spec::TableRecord> = Vec::new();

        for _ in range(0, offset_table.numTables) {
            table_records.push(try!(spec::read(stream)));
        }

        for table_record in table_records.iter() {
            match table_record.tag {
                spec::FONT_HEADER_TAG => {
                    try!(stream.seek(table_record.offset as i64, io::SeekSet));
                    if !Table::map_and_check(stream, table_record,
                        |chunk, i| if i == 2 { 0 } else { chunk }) {

                        raise!("The file is corrupted.");
                    }

                    try!(stream.seek(table_record.offset as i64, io::SeekSet));
                    try!(self.parse_font_header(stream));
                }
                spec::MAXIMAL_PROFILE_TAG => {
                    try!(stream.seek(table_record.offset as i64, io::SeekSet));
                    if !Table::check(stream, table_record) {
                        raise!("The file is corrupted.");
                    }

                    try!(stream.seek(table_record.offset as i64, io::SeekSet));
                    try!(self.parse_maximum_profile(stream));
                }
                _ => ()
            }
        }

        self.format = CFF;

        Ok(())
    }

    fn parse_font_header<R: io::Reader>(&mut self, stream: &mut R)
        -> io::IoResult<()> {

        let table: spec::FontHeader = try!(spec::read(stream));

        if table.version != 1.0 {
            raise!("The format version is supported.");
        }

        if table.magicNumber != spec::FONT_HEADER_MAGIC_NUMBER {
            raise!("The file is currupted.");
        }

        self.version = table.fontRevision;
        self.units_per_em = table.unitsPerEm;

        self.created_on = Date::at_since_1904(table.created);
        self.updated_on = Date::at_since_1904(table.modified);

        self.style.parse(table.macStyle);

        Ok(())
    }

    fn parse_maximum_profile<R: io::Reader>(&mut self, stream: &mut R)
        -> io::IoResult<()> {

        let table: spec::MaximumProfile = try!(spec::read(stream));

        if table.version != spec::MAXIMAL_PROFILE_VERSION_0_5 {
            raise!("The format version is supported.");
        }

        self.glyph_count = table.numGlyphs;

        Ok(())
    }
}

pub fn parse(stream: &mut io::File) -> io::IoResult<Font> {
    let mut font: Font = default::Default::default();

    try!(font.parse(stream));

    Ok(font)
}
