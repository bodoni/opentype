//! The glyph class.

use truetype::GlyphID;

use crate::Result;

/// A class.
#[derive(Clone, Debug)]
pub enum Class {
    /// Format 1.
    Format1(Class1),
    /// Format 2.
    Format2(Class2),
}

table! {
    /// A class in format 1.
    pub Class1 { // ClassDefFormat1
        format         (u16    ), // classFormat
        start_glyph_id (GlyphID), // startGlyphID
        glyph_count    (u16    ), // glyphCount

        indices (Vec<u16>) |this, tape| { // classValueArray
            tape.take_given(this.glyph_count as usize)
        },
    }
}

table! {
    /// A class in format 2.
    pub Class2 { // ClassDefFormat2
        format       (u16), // classFormat
        record_count (u16), // classRangeCount

        records (Vec<Record>) |this, tape| { // classRangeRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    /// A class record.
    #[derive(Copy)]
    pub Record { // ClassRangeRecord
        start_glyph_id (GlyphID), // startGlyphID
        end_glyph_id   (GlyphID), // endGlyphID
        index          (u16    ), // class
    }
}

impl Default for Class {
    #[inline]
    fn default() -> Self {
        Class::Format1(Class1::default())
    }
}

impl crate::value::Read for Class {
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Class::Format1(tape.take()?),
            2 => Class::Format2(tape.take()?),
            value => raise!("found an unknown format of the glyph class ({value})"),
        })
    }
}
