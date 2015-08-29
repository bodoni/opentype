use std::io::{Read, Seek, SeekFrom};

use Result;

pub trait Tape: Read + Seek + Sized {
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }
}

impl<T: Read + Seek> Tape for T {
}
