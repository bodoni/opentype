use truetype::{GlyphID, Result, Tape, Value};

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
    pub Class1 {
        format      (u16    ) = { 1 }, // ClassFormat
        start       (GlyphID), // StartGlyph
        value_count (u16    ), // GlyphCount

        values (Vec<u16>) |this, tape| { // ClassValueArray
            tape.take_given(this.value_count as usize)
        },
    }
}

table! {
    #[doc = "A class definition in format 2."]
    pub Class2 {
        format      (u16) = { 2 }, // ClassFormat
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

impl Value for Class {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => Class::Format1(try!(tape.take())),
            2 => Class::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the class definition"),
        })
    }
}
