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

pub fn parse(filename: &str) -> Result<Font, io::IoError> {
    macro_rules! try(
        ($suspect:expr) => (
            match $suspect {
                Ok(result) => result,
                Err(error) => return Err(error)
            }
        )
    )

    macro_rules! try_load(
        ($file:ident) => (
            try!(input::Loader::load(&mut $file))
        )
    )

    let mut file = try!(io::File::open(&Path::new(filename)));

    let offset_table: OffsetTable = try_load!(file);

    if offset_table.tag != format::CFF_TAG {
        return Err(io::IoError {
            kind: io::OtherIoError,
            desc: "The format is not supported.",
            detail: None,
        })
    }

    let mut table_records = Vec::new();

    for i in range(0, offset_table.table_count) {
        table_records.push(try_load!(file));
    }

    Ok(Font {
        offset_table: offset_table,
        table_records: table_records,
    })
}
