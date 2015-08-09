use std::io::{Read, Seek, SeekFrom};

use Result;

#[doc(hidden)]
pub trait Band: Read + Seek + Sized {
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    #[inline]
    fn peek<T: Primitive>(&mut self) -> Result<T> {
        self.save(|band| Primitive::read(band))
    }

    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }

    fn save<F, T>(&mut self, mut body: F) -> Result<T> where F: FnMut(&mut Self) -> Result<T> {
        let position = try!(self.position());
        let value = body(self);
        try!(self.jump(position));
        Ok(try!(value))
    }
}

#[doc(hidden)]
pub trait Compound {
    fn read<T: Band>(&mut self, &mut T) -> Result<()>;
}

#[doc(hidden)]
pub trait Primitive {
    fn read<T: Band>(&mut T) -> Result<Self>;
}

impl<T: Read + Seek> Band for T {
}
