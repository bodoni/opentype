#![crate_name = "opentype"]
#![crate_type = "rlib"]

#![feature(phase, macro_rules)]

extern crate input;

use std::io;
use input::Loader;
use format::{OffsetTable, TableRecord, TableContent, Table};

pub mod format;

pub struct Font {
    pub offset_table: OffsetTable,
    pub tables: Vec<Table>,
}

macro_rules! parse_error(
    () => (
        io::IoError {
            kind: io::OtherIoError,
            desc: "Cannot parse the file.",
            detail: None,
        }
    );
    ($message:expr) => (
        io::IoError {
            kind: io::OtherIoError,
            desc: $message,
            detail: None,
        }
    );
)

pub fn parse(reader: &mut io::File) -> Result<Font, io::IoError> {
    macro_rules! try(
        ($suspect:expr) => (
            match $suspect {
                Ok(result) => result,
                Err(error) => return Err(error)
            }
        )
    )

    let offset_table: OffsetTable = try!(Loader::from(reader));

    if offset_table.tag != format::CFFFormatTag {
        return Err(parse_error!("The format is not supported."));
    }

    let mut table_records: Vec<TableRecord> = Vec::new();

    for _ in range(0, offset_table.table_count) {
        table_records.push(try!(Loader::from(reader)));
    }

    let mut tables = Vec::new();

    for table_record in table_records.iter() {
        tables.push(Table {
            record: *table_record,
            content: try!(read_table_content(reader, table_record)),
        });
    }

    for table in tables.iter() {
        if !table.is_valid() { parse_error!(); }
    }

    Ok(Font {
        offset_table: offset_table,
        tables: tables,
    })
}

fn read_table_content(reader: &mut io::File, table_record: &TableRecord)
    -> Result<TableContent, io::IoError> {

    match reader.seek(table_record.offset as i64, io::SeekSet) {
        Ok(_) => (),
        Err(error) => return Err(error)
    }

    input::read_be_u16(reader, Table::length_for(table_record))
}
