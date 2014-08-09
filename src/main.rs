#![crate_name = "benton"]
#![feature(globs, macro_rules)]

use std::io;
use std::os;
use result::*;

mod result;

static CFF_TAG: u32 = 0x4F54544F;

fn main() {
    let arguments = os::args();

    if arguments.len() != 2 {
        println!("Usage: {} <file>", arguments[0]);
        return;
    }

    match parse(arguments[1].as_slice()) {
        Ok(_) => println!("Done."),
        Err(error) => {
            println!("{}", error);
            os::set_exit_status(1);
        }
    }
}

fn parse(filename: &str) -> Result<()> {
    let mut reader = try!(open(filename), IOError);

    let tag = try!(reader.read_be_u32(), FormatError);
    assert!(tag == CFF_TAG, FormatError, "Unsupported format.");

    Ok(())
}

fn open(filename: &str) -> std::result::Result<Box<io::Reader>, io::IoError> {
    match io::fs::File::open(&Path::new(filename)) {
        Ok(file) => Ok(box file as Box<io::Reader>),
        Err(error) => Err(error)
    }
}
