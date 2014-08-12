#![crate_name = "benton"]
#![feature(globs, macro_rules)]

extern crate input;
extern crate opentype;

use std::os;
use result::*;
use opentype::Font;

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

    let fonts: Vec<Box<Font>> = try!(opentype::parse(filename), ParseError);

    println!("Number of fonts: {}", fonts.len());

    for i in range(0, fonts.len()) {
        println!("Font #{}", i + 1);
        println!("  Tables:");

        let ref font = fonts[i];

        for j in range(0u, font.offset_table.table_count as uint) {
            let tag = font.table_records[j].tag;

            match input::stringify_u32(tag) {
                Some(name) => println!("  {}", name),
                None => raise!(ParseError)
            }
        }
    }

    Ok(())
}
