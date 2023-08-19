//! The glyph coverage.

use truetype::GlyphID;

use crate::{Result, Tape, Value};

/// A coverage.
#[derive(Clone, Debug)]
pub enum Coverage {
    /// Format 1.
    Format1(Coverage1),
    /// Format 2.
    Format2(Coverage2),
}

table! {
    #[doc = "A coverage in format 1."]
    pub Coverage1 { // CoverageFormat1
        format (u16), // coverageFormat
        count  (u16), // glyphCount

        records (Vec<GlyphID>) |this, tape| { // glyphArray
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage in format 2."]
    pub Coverage2 { // CoverageFormat2
        format (u16), // coverageFormat
        count  (u16), // rangeCount

        records (Vec<Record>) |this, tape| { // rangeRecords
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage record."]
    #[derive(Copy)]
    pub Record { // RangeRecord
        start_glyph_id (GlyphID), // startGlyphID
        end_glyph_id   (GlyphID), // endGlyphID
        index          (u16    ), // startCoverageIndex
    }
}

impl Default for Coverage {
    #[inline]
    fn default() -> Self {
        Coverage::Format1(Coverage1::default())
    }
}

impl Value for Coverage {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Coverage::Format1(tape.take()?),
            2 => Coverage::Format2(tape.take()?),
            value => raise!("found an unknown format of the glyph coverage ({value})"),
        })
    }
}
