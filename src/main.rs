#![crate_name = "benton"]

use std::os;

fn main() {
    let arguments = os::args();

    if arguments.len() != 2 {
        println!("Usage: benton <file>");
        return;
    }

    let ref filename = arguments[1];

    println!("Filename: {}", filename);
}
