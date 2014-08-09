#![crate_name = "benton"]
#![feature(macro_rules)]

use std::io;
use std::os;
use std::str;
use result::Result;

mod result;

fn main() {
    let arguments = os::args();

    if arguments.len() != 2 {
        println!("Usage: {} <file>", arguments[0]);
        return;
    }

    let filename: &str = arguments[1].as_slice();

    println!("Filename: {}", filename);

    match parse(filename) {
        Ok(_) => println!("Done."),
        Err(error) => {
            println!("{}", error);
            std::os::set_exit_status(1);
        }
    }
}

fn parse(filename: &str) -> Result<()> {
    let mut reader = ensure!(open(filename), "Cannot open the file.");

    let mut buffer = [0, ..1024];

    let count = ensure!(reader.read(buffer), "Cannot read the file.");

    if count < 4 {
        error!("The file format is unknown.")
    }

    let tag = fetch!(str::from_utf8(buffer.slice_to(4)), "");

    if tag != "OTTO" {
        error!("The file format is unknown.");
    }

    Ok(())
}

fn open(filename: &str) -> std::result::Result<Box<io::Reader>, io::IoError> {
    match io::fs::File::open(&Path::new(filename)) {
        Ok(file) => Ok(box file as Box<io::Reader>),
        Err(error) => Err(error)
    }
}
