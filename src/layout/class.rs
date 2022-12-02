use truetype::GlyphID;

use crate::{Result, Tape, Value};

/// A class definition.
#[derive(Clone, Debug)]
pub enum Class {
    /// Format 1.
    Format1(Class1),
    /// Format 2.
    Format2(Class2),
}

table! {
    #[doc = "A class definition in format 1."]
    pub Class1 { // ClassDefFormat1
        format      (u16    ), // ClassFormat
        start       (GlyphID), // StartGlyph
        value_count (u16    ), // GlyphCount

        values (Vec<u16>) |this, tape| { // ClassValueArray
            tape.take_given(this.value_count as usize)
        },
    }
}

table! {
    #[doc = "A class definition in format 2."]
    pub Class2 { // ClassDefFormat2
        format      (u16), // ClassFormat
        range_count (u16), // ClassRangeCount

        ranges (Vec<ClassRange>) |this, tape| { // ClassRangeRecord
            tape.take_given(this.range_count as usize)
        },
    }
}

table! {
    #[doc = "A class range."]
    #[derive(Copy)]
    pub ClassRange { // ClassRangeRecord
        start (GlyphID), // Start
        end   (GlyphID), // End
        index (u16    ), // Class
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
            _ => raise!("found an unknown format of the class definition"),
        })
    }
}
