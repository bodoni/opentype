//! The [color-palette table][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal

use crate::{Result, Tape, Value};

/// A color-palette table.
pub struct ColorPalettes {
    pub header: Header,
    pub colors: Vec<Color>,
}

/// The header of a color-palette table.
#[derive(Clone, Debug)]
pub enum Header {
    /// Version 0.
    Version0(Header0),
}

table! {
    #[doc = "The header of a color-palette table of version 0."]
    pub Header0 {
        version       (u16) = { 0 }, // version
        entry_count   (u16), // numPaletteEntries
        palette_count (u16), // numPalettes
        color_count   (u16), // numColorRecords
        color_offset  (u32), // colorRecordsArrayOffset

        color_indices (Vec<u16>) |this, tape| { // colorRecordIndices
            tape.take_given(this.palette_count as usize)
        },
    }
}

table! {
    #[doc = "A color."]
    #[derive(Copy)]
    pub Color { // ColorRecord
        blue  (u8), // blue
        green (u8), // green
        red   (u8), // red
        alpha (u8), // alpha
    }
}

impl ColorPalettes {
    /// Iterate over the entries.
    ///
    /// Each item represents a palette given as an iterator over the corresponding colors.
    pub fn iter(&self) -> impl Iterator<Item = impl Iterator<Item = &Color>> {
        match self.header {
            Header::Version0(ref header) => header.color_indices.iter().map(|palette_index| {
                (0..header.entry_count as usize).map(|entry_index| {
                    let color_index = *palette_index as usize + entry_index;
                    debug_assert!(color_index < header.color_count as usize);
                    &self.colors[color_index]
                })
            }),
        }
    }
}

impl Value for ColorPalettes {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = tape.position()?;
        let header = tape.take()?;
        let (offset, count) = match header {
            Header::Version0(ref header) => (header.color_offset, header.color_count as usize),
        };
        let colors = jump_take_given!(@unwrap tape, position, offset, count);
        Ok(Self { header, colors })
    }
}

impl Value for Header {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            0 => Self::Version0(tape.take()?),
            _ => raise!("found an unknown version of the color-palette table"),
        })
    }
}
