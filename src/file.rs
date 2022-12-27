use std::io::{Read, Seek};

use truetype::Tag;

use crate::{Font, Result, Tape};

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Read a file.
    pub fn read<T: Read + Seek>(tape: &mut T) -> Result<File> {
        if Tag::from(Tape::peek::<u32>(tape)?) == Tag(*b"ttcf") {
            raise!("TrueType collections are not supported yet");
        }
        Ok(File {
            fonts: vec![Font::read(tape)?],
        })
    }
}

dereference! { File::fonts => [Font] }
