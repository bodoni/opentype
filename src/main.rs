#![crate_name = "benton"]

use std::os;
use std::io::File;

fn main() {
    let arguments = os::args();

    if arguments.len() != 2 {
        println!("Usage: {} <file>", arguments[0]);
        return;
    }

    let filename: &str = arguments[1].as_slice();

    println!("Filename: {}", filename);

    let mut reader = match File::open(&Path::new(filename)) {
        Ok(file) => box file as Box<Reader>,
        Err(_) => {
            println!("Cannot open the file.");
            return;
        }
    };

    let mut buffer = [0, ..4];

    let tag = match reader.read(buffer) {
        Ok(n) => {
            match std::str::from_utf8(buffer.slice_to(n)) {
                Some(string) => string,
                None => {
                    println!("Cannot read the file.")
                    return;
                }
            }
        },
        Err(_) => {
            println!("Cannot read the file.");
            return;
        }
    };

    println!("Tag: {}", tag);
}
