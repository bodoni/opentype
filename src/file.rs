use std::io::{Read, Seek};
use std::ops::Deref;
use truetype::{Result, Tag, Tape};

use crate::Font;

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

impl Deref for File {
    type Target = [Font];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.fonts
    }
}
