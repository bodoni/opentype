#![crate_name = "benton"]
#![feature(globs, macro_rules)]

extern crate input;
extern crate opentype;

use std::os;
use result::*;
use opentype::{Font, parse};

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

    let font: Font = try!(parse(filename), ParseError);

    println!("Tables:");

    for i in range(0u, font.offset_table.table_count as uint) {
        let tag = input::convert_u32_to_string(font.table_records[i].tag);
        println!("{}", tag);
    }

    Ok(())
}
