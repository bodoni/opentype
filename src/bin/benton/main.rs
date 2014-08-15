#![crate_name = "benton"]
#![feature(globs, macro_rules)]

extern crate input;
extern crate opentype;

use std::os;
use std::io;
use result::*;

mod result;

fn main() {
    match start(&os::args()) {
        Ok(()) => println!("Done."),
        Err(error) => {
            println!("{}", error);
            os::set_exit_status(1);
        }
    }
}

fn start(arguments: &Vec<String>) -> Result<()> {
    if arguments.len() != 2 {
        raise!(ArgumentError, "Usage: {} <file>", arguments[0]);
    }

    let filename: &str = arguments[1].as_slice();

    println!("Filename: {}", filename);

    let mut reader = try!(io::File::open(&Path::new(filename)), ArgumentError);
    let font = try!(opentype::parse(&mut reader), ParseError);

    println!("Tables:");

    for table in font.tables.iter() {
        match input::stringify_le_u32(table.record.tag) {
            Some(name) => println!("{}", name),
            None => raise!(ParseError)
        }
    }

    Ok(())
}
