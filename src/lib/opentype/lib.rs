#![crate_name = "opentype"]
#![crate_type = "rlib"]

#![feature(phase, macro_rules)]

extern crate input;

use std::io;
use format::{OffsetTable, TableRecord};

pub mod format;

pub struct Font {
    pub offset_table: OffsetTable,
    pub table_records: Vec<TableRecord>,
}

macro_rules! try(
    ($suspect:expr) => (
        match $suspect {
            Ok(result) => result,
            Err(error) => return Err(error)
        }
    )
)

pub fn parse(filename: &str) -> Result<Vec<Box<Font>>, io::IoError> {
    let mut reader = try!(io::File::open(&Path::new(filename)));
    let mut collection = Vec::new();

    for i in range(0u, 1) {
        match parse_font(&mut reader) {
            Ok(result) => collection.push(result),
            Err(error) => return Err(error)
        }
    }

    Ok(collection)
}

fn parse_font(reader: &mut io::Reader) -> Result<Box<Font>, io::IoError> {
    macro_rules! try_load(
        ($reader:ident) => (
            try!(input::Loader::load($reader))
        )
    )

    let offset_table: OffsetTable = try_load!(reader);

    if offset_table.tag != format::CFFFormatTag {
        return Err(io::IoError {
            kind: io::OtherIoError,
            desc: "The format is not supported.",
            detail: None,
        })
    }

    let mut table_records = Vec::new();

    for i in range(0, offset_table.table_count) {
        table_records.push(try_load!(reader));
    }

    Ok(box Font {
        offset_table: offset_table,
        table_records: table_records,
    })
}
