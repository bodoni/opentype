#![crate_name = "opentype"]
#![crate_type = "rlib"]

#![feature(globs, phase, macro_rules)]

extern crate input;

use std::io;
use format::{OffsetTable, TableRecord};
use input::read_big_endian;

pub mod format;

pub struct Font {
    pub offset_table: OffsetTable,
    pub table_records: Vec<TableRecord>,
}

pub fn parse(filename: &str) -> Result<Font, io::IoError> {
    macro_rules! try(
        ($suspect:expr) => (
            match $suspect {
                Ok(result) => result,
                Err(error) => return Err(error)
            }
        )
    )

    macro_rules! try_read(
        ($file:ident, $object:ty) => (
            try!(read_big_endian::<$object>(&mut $file))
        )
    )

    let mut file = try!(io::File::open(&Path::new(filename)));

    let offset_table = try_read!(file, OffsetTable);

    if offset_table.tag != format::CFF_TAG {
        return Err(io::IoError {
            kind: io::OtherIoError,
            desc: "The format is not supported.",
            detail: None,
        })
    }

    let mut table_records = Vec::new();

    for i in range(0, offset_table.table_count) {
        table_records.push(try_read!(file, TableRecord));
    }

    Ok(Font {
        offset_table: offset_table,
        table_records: table_records,
    })
}
