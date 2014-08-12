#![crate_name = "opentype"]
#![crate_type = "rlib"]

#![feature(phase, macro_rules)]

extern crate input;

use std::io;
use format::{OffsetTable, TableRecord};

pub mod format;

pub struct Table {
    pub table_record: TableRecord,
}

pub struct Font {
    pub offset_table: OffsetTable,
    pub tables: Vec<Table>,
}

macro_rules! try(
    ($suspect:expr) => (
        match $suspect {
            Ok(result) => result,
            Err(error) => return Err(error)
        }
    )
)

pub fn parse(reader: &mut io::Reader) -> Result<Font, io::IoError> {
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

    let mut tables = Vec::new();

    for i in range(0, offset_table.table_count) {
        tables.push(Table {
           table_record: try_load!(reader)
        });
    }

    for table in tables.iter() {
    }

    Ok(Font {
        offset_table: offset_table,
        tables: tables,
    })
}
