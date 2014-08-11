#![crate_name = "opentype"]
#![crate_type = "rlib"]

#![feature(globs, phase)]

extern crate input;

use std::io;
use format::OffsetTable;

pub mod format;

pub struct Font {
    pub offset_table: OffsetTable,
}

pub fn parse(filename: &str) -> Result<Font, io::IoError> {
    let mut file = match io::File::open(&Path::new(filename)) {
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let offset_table = match input::read_big_endian::<OffsetTable>(&mut file) {
        Ok(result) => result,
        Err(error) => return Err(error)
    };

    if offset_table.tag != format::CFF_TAG {
        return Err(io::IoError {
            kind: io::OtherIoError,
            desc: "The format is not supported.",
            detail: None,
        })
    }

    Ok(Font {
        offset_table: offset_table,
    })
}
