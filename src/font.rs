use std::io::{Read, Seek};
use truetype::offset_table::OffsetTable;
use truetype::{Result, Tag, Tape};

use crate::Table;

/// A font.
pub struct Font {
    /// The offset table.
    pub offset_table: OffsetTable,
}

impl Font {
    /// Read a font.
    #[inline]
    pub fn read<T>(tape: &mut T) -> Result<Font>
    where
        T: Read + Seek,
    {
        Ok(Font {
            offset_table: Tape::take(tape)?,
        })
    }

    /// Find, verify, and read a table.
    #[inline]
    pub fn take<'l, T, U>(&self, tape: &mut T) -> Result<Option<U>>
    where
        T: Read + Seek,
        U: Table<'l, Parameter = ()>,
    {
        self.take_given(tape, ())
    }

    /// Find, verify, and read a table given a parameter.
    pub fn take_given<'l, T, U>(&self, tape: &mut T, parameter: U::Parameter) -> Result<Option<U>>
    where
        T: Read + Seek,
        U: Table<'l>,
    {
        let tag = U::tag();
        for record in &self.offset_table.records {
            if record.tag == tag {
                let check = if tag == Tag(*b"head") {
                    record.checksum(tape, |i, word| if i == 2 { 0 } else { word })?
                } else {
                    record.checksum(tape, |_, word| word)?
                };
                if !check {
                    raise!("found a malformed font table");
                }
                Tape::jump(tape, record.offset as u64)?;
                return Ok(Some(Table::take(tape, parameter)?));
            }
        }
        Ok(None)
    }
}
