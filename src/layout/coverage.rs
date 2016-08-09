use truetype::GlyphID;

use {Result, Tape, Value};
use super::Range;

/// A coverage table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Coverage {
    /// Format 1.
    Format1(Coverage1),
    /// Format 2.
    Format2(Coverage2),
}

table! {
    #[doc = "A coverage table in format 1."]
    pub Coverage1 {
        format (u16), // CoverageFormat
        count  (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // GlyphArray
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage table in format 2."]
    pub Coverage2 {
        format (u16), // CoverageFormat
        count  (u16), // RangeCount

        ranges (Vec<Range>) |this, tape| { // RangeRecord
            tape.take_given(this.count as usize)
        },
    }
}

impl Value for Coverage {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => Coverage::Format1(try!(tape.take())),
            2 => Coverage::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the coverage table"),
        })
    }
}
