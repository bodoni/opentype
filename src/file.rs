use std::fs;
use std::io::{Read, Seek};
use std::ops::Deref;
use std::path::Path;

use truetype::{self, Fixed, Tag};

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
        match try!(truetype::Tape::peek::<Fixed>(tape)) {
            Fixed(0x00010000) => {},
            version => {
                let tag = Tag::from(version);
                if tag == Tag::from(b"OTTO") {
                } else if tag == Tag::from(b"ttcf") {
                    raise!("TrueType collections are not supported yet");
                } else {
                    raise!("the font format is invalid");
                }
            },
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
