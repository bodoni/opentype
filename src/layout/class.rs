//! The class.

use truetype::GlyphID;

use crate::{Result, Tape, Value};

/// A class.
#[derive(Clone, Debug)]
pub enum Class {
    /// Format 1.
    Format1(Class1),
    /// Format 2.
    Format2(Class2),
}

table! {
    #[doc = "A class in format 1."]
    pub Class1 { // ClassDefFormat1
        format         (u16    ), // classFormat
        start_glyph_id (GlyphID), // startGlyphID
        count          (u16    ), // glyphCount

        records (Vec<u16>) |this, tape| { // classValueArray
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A class in format 2."]
    pub Class2 { // ClassDefFormat2
        format (u16), // classFormat
        count  (u16), // classRangeCount

        records (Vec<Record>) |this, tape| { // classRangeRecords
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A class range."]
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

impl Value for Class {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Class::Format1(tape.take()?),
            2 => Class::Format2(tape.take()?),
            value => raise!("found an unknown format of the class table ({value})"),
        })
    }
}
