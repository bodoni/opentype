use truetype::{GlyphID, Result, Tape, Value};

/// A coverage table.
#[derive(Clone, Debug)]
pub enum Coverage {
    /// Format 1.
    Format1(Coverage1),
    /// Format 2.
    Format2(Coverage2),
}

table! {
    #[doc = "A coverage table in format 1."]
    pub Coverage1 { // CoverageFormat1
        format (u16), // CoverageFormat
        count  (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // GlyphArray
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage table in format 2."]
    pub Coverage2 { // CoverageFormat2
        format (u16), // CoverageFormat
        count  (u16), // RangeCount

        ranges (Vec<CoverageRange>) |this, tape| { // RangeRecord
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage range."]
    #[derive(Copy)]
    pub CoverageRange { // RangeRecord
        start (GlyphID), // Start
        end   (GlyphID), // End
        index (u16    ), // StartCoverageIndex
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
            _ => raise!("found an unknown format of the coverage table"),
        })
    }
}
