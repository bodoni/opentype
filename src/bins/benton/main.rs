#![crate_name = "benton"]
#![feature(globs, macro_rules)]

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

    match opentype::parse(filename) {
        Ok(font) => {
            println!("Table count: {}", font.offset_table.table_count);
        },
        Err(error) => {
            match error.kind {
                io::FileNotFound =>
                    raise!(ArgumentError, "The file does not exist."),

                _ =>
                    raise!(ParseError, "{}", error.desc),
            }
        }
    }

    Ok(())
}
