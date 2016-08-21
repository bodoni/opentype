use std::fs;
use std::io::{Read, Seek};
use std::ops::Deref;
use std::path::Path;
use truetype::{Tag, Tape, q32};

use Result;
use font::Font;

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Open a file.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<File> {
        let mut file = try!(fs::File::open(path));
        File::read(&mut file)
    }

    /// Read a file.
    pub fn read<T: Read + Seek>(tape: &mut T) -> Result<File> {
        if Tag::from(try!(Tape::peek::<q32>(tape))) == Tag(*b"ttcf") {
            raise!("TrueType collections are not supported yet");
        }
        Ok(File { fonts: vec![try!(Font::read(tape))] })
    }
}

impl Deref for File {
    type Target = [Font];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.fonts
    }
}
