use truetype::tables::offsets::Offsets;

use crate::tape::Read;
use crate::{Result, Table};

/// A font.
pub struct Font {
    /// The offset table.
    pub offsets: Offsets,
}

impl Font {
    /// Read a file.
    #[inline]
    pub fn read<T>(tape: &mut T) -> Result<Self>
    where
        T: crate::tape::Read,
    {
        Read::take(tape)
    }

    /// Read a table.
    #[inline]
    pub fn take<'l, T, U>(&self, tape: &mut T) -> Result<Option<U>>
    where
        T: crate::tape::Read,
        U: Table<'l, Parameter = ()>,
    {
        self.take_given(tape, ())
    }

    /// Read a table given a parameter.
    pub fn take_given<'l, T, U>(&self, tape: &mut T, parameter: U::Parameter) -> Result<Option<U>>
    where
        T: crate::tape::Read,
        U: Table<'l>,
    {
        let tag = U::tag();
        for record in &self.offsets.records {
            if record.tag == tag {
                #[cfg(not(feature = "ignore-invalid-checksums"))]
                if record.checksum != record.checksum(tape)? {
                    raise!("found a malformed font table with {:?}", record.tag);
                }
                Read::jump(tape, record.offset as u64)?;
                return Ok(Some(Table::take(tape, parameter)?));
            }
        }
        Ok(None)
    }
}

impl crate::value::Read for Font {
    #[inline]
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok(Self {
            offsets: tape.take()?,
        })
    }
}
