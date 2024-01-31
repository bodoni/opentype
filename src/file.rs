use truetype::Tag;

use crate::tape::Read;
use crate::{Font, Result};

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
}

impl File {
    /// Read a file.
    pub fn read<T: crate::tape::Read>(tape: &mut T) -> Result<File> {
        if &Read::peek::<Tag>(tape)?.0 == b"ttcf" {
            raise!("found a TrueType collection, which is not supported yet");
        }
        Ok(File {
            fonts: vec![Font::read(tape)?],
        })
    }
}

dereference! { File::fonts => [Font] }
