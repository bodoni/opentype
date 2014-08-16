#![crate_name = "opentype"]
#![crate_type = "rlib"]

#![feature(globs, phase, macro_rules)]

extern crate input;

use std::{io, mem, fmt};
use std::default::Default;
use input::Structure;
use format::*;

pub mod format;

macro_rules! raise(
    () => (return Err(io::IoError {
        kind: io::OtherIoError,
        desc: "Cannot digest the file.",
        detail: None,
    }));
    ($message:expr) => (return Err(io::IoError {
        kind: io::OtherIoError,
        desc: $message,
        detail: None,
    }));
)

macro_rules! try(
    ($suspect:expr) => (match $suspect {
        Ok(result) => result,
        Err(error) => return Err(error)
    })
)

pub struct Table {
    pub checksum: u32,
    pub content: Vec<u32>,
}

impl Table {
    pub fn read(stream: &mut io::File, table_record: &TableRecord)
        -> Result<Table, io::IoError> {

        let length = Table::measure(table_record);

        Ok(Table {
            checksum: table_record.checkSum,
            content: try!(input::read_be_u32(stream, length))
        })
    }

    pub fn measure(table_record: &TableRecord) -> uint {
        let length = table_record.length as uint;
        let size = mem::size_of::<u32>();
        ((length + size - 1) & !(size - 1)) / size
    }

    pub fn check(&self) -> bool {
        let mut checksum: u32 = 0;

        for chunk in self.content.iter() {
            checksum += *chunk as u32;
        }

        self.checksum == checksum
    }
}

pub enum Format {
    CFF,
}

impl Default for Format {
    fn default() -> Format { CFF }
}

impl fmt::Show for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter)
        -> Result<(), fmt::FormatError> {

        match *self {
            CFF => write!(formatter, "CFF")
        }
    }
}

#[deriving(Default, Show)]
pub struct Description {
    pub version: f32,
    pub units_per_em: u16,
    pub created_at: i64,
    pub modified_at: i64,
}

#[deriving(Default, Show)]
pub struct Font {
    pub format: Format,
    pub description: Description,
}

impl Font {
    fn digest(&mut self, stream: &mut io::File) -> Result<(), io::IoError> {
        let offset_table: OffsetTable = try!(Structure::read(stream));

        match offset_table.tag {
            CFF_FORMAT_TAG => try!(self.digest_cff(stream, &offset_table)),
            _ => raise!("The format is not supported.")
        }

        Ok(())
    }

    fn digest_cff(&mut self, stream: &mut io::File, offset_table: &OffsetTable)
        -> Result<(), io::IoError> {

        let mut table_records: Vec<TableRecord> = Vec::new();

        for _ in range(0, offset_table.numTables) {
            table_records.push(try!(Structure::read(stream)));
        }

        for table_record in table_records.iter() {
            match table_record.tag {
                FONT_HEADER_TAG => {
                    try!(stream.seek(table_record.offset as i64, io::SeekSet));
                    let mut table = try!(Table::read(stream, table_record));
                    *table.content.get_mut(2) = 0;
                    if !table.check() { raise!("The file is corrupted."); }

                    try!(stream.seek(table_record.offset as i64, io::SeekSet));
                    try!(self.digest_font_header(stream));
                },
                _ => {}
            }
        }

        self.format = CFF;

        Ok(())
    }

    fn digest_font_header(&mut self, stream: &mut io::File)
        -> Result<(), io::IoError> {

        let table: FontHeader = try!(Structure::read(stream));

        self.description.version = table.fontRevision;
        self.description.units_per_em = table.unitsPerEm;
        self.description.created_at = table.created;
        self.description.modified_at = table.modified;

        Ok(())
    }
}

pub fn parse(stream: &mut io::File) -> Result<Font, io::IoError> {
    let mut font: Font = Default::default();

    try!(font.digest(stream));

    Ok(font)
}
