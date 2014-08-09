#![crate_name = "benton"]

use std::fmt;
use std::io;
use std::os;
use std::str;

type Result = std::result::Result<(), Error>;

struct Error {
    message: str::SendStr,
}

impl Error {
    pub fn new<T: str::IntoMaybeOwned<'static>>(message: T) -> Error {
        Error {
            message: message.into_maybe_owned()
        }
    }
}

impl fmt::Show for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

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
        Err(_) => return Err(Error::new("Cannot open the file."))
    };

    let mut buffer = [0, ..4];

    let tag = match reader.read(buffer) {
        Ok(n) => {
            match std::str::from_utf8(buffer.slice_to(n)) {
                Some(string) => string,
                None => return Err(Error::new("Cannot read the file."))
            }
        },
        Err(_) => return Err(Error::new("Cannot read the file."))
    };

    println!("Tag: {}", tag);

    Ok(())
}
