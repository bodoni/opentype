#![crate_name = "opentype"]
#![crate_type = "rlib"]

#![feature(globs, macro_rules)]

use std::{default, fmt, io};
pub use date::Date;

pub mod spec;
pub mod table;
mod date;

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
    pub created_at: Date,
    pub updated_at: Date,
}

impl Font {
    fn parse(&mut self, stream: &mut io::File) -> Result<(), io::IoError> {
        let offset_table: spec::OffsetTable = try!(spec::read(stream));

        match offset_table.tag {
            spec::CFF_FORMAT_TAG => try!(self.parse_cff(stream, &offset_table)),
            _ => raise!("The format is not supported.")
        }

        Ok(())
    }

    fn parse_cff(&mut self, stream: &mut io::File,
        offset_table: &spec::OffsetTable) -> Result<(), io::IoError> {

        let mut table_records: Vec<spec::TableRecord> = Vec::new();

        for _ in range(0, offset_table.numTables) {
            table_records.push(try!(spec::read(stream)));
        }

        for table_record in table_records.iter() {
            match table_record.tag {
                spec::FONT_HEADER_TAG => {
                    try!(stream.seek(table_record.offset as i64, io::SeekSet));
                    if !table::preprocess_and_check(stream, table_record,
                        |chunk, i| if i == 2 { 0 } else { chunk }) {

                        raise!("The file is corrupted.");
                    }

                    try!(stream.seek(table_record.offset as i64, io::SeekSet));
                    try!(self.parse_font_header(stream));
                }
                _ => ()
            }
        }

        self.format = CFF;

        Ok(())
    }

    fn parse_font_header<R: io::Reader>(&mut self, stream: &mut R)
        -> Result<(), io::IoError> {

        let table: spec::FontHeader = try!(spec::read(stream));

        if table.magicNumber != spec::MAGIC_NUMBER {
            raise!("The file is currupted.");
        }

        self.version = table.fontRevision;
        self.units_per_em = table.unitsPerEm;
        self.created_at = Date::new(table.created);
        self.updated_at = Date::new(table.modified);

        Ok(())
    }
}

pub fn parse(stream: &mut io::File) -> Result<Font, io::IoError> {
    let mut font: Font = default::Default::default();

    try!(font.parse(stream));

    Ok(font)
}
