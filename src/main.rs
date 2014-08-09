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

    match read(filename) {
        Ok(_) => println!("Done."),
        Err(error) => {
            println!("{}", error);
            std::os::set_exit_status(1);
        }
    }
}

fn read(filename: &str) -> Result {
    let mut reader = match io::File::open(&Path::new(filename)) {
        Ok(file) => box file as Box<Reader>,
        Err(_) => error!("Cannot open the file.")
    };

    let mut buffer = [0, ..4];

    let tag = match reader.read(buffer) {
        Ok(n) => {
            match str::from_utf8(buffer.slice_to(n)) {
                Some(string) => string,
                None => error!("Cannot read the file.")
            }
        },
        Err(_) => error!("Cannot read the file.")
    };

    println!("Tag: {}", tag);

    Ok(())
}
