use truetype::Tag;

use crate::{Font, Result, Tape};

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Read a file.
    pub fn read<T: Tape>(tape: &mut T) -> Result<File> {
        if Tag::from(Tape::peek::<u32>(tape)?) == Tag(*b"ttcf") {
            raise!("found a TrueType collection, which is not supported yet");
        }
        Ok(File {
            fonts: vec![Font::read(tape)?],
        })
    }
}

dereference! { File::fonts => [Font] }
