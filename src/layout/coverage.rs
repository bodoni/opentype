//! The glyph coverage.

use truetype::GlyphID;

use crate::Result;

/// A coverage.
#[derive(Clone, Debug)]
pub enum Coverage {
    /// Format 1.
    Format1(Coverage1),
    /// Format 2.
    Format2(Coverage2),
}

table! {
    /// A coverage in format 1.
    pub Coverage1 { // CoverageFormat1
        format      (u16), // coverageFormat
        glyph_count (u16), // glyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // glyphArray
            tape.take_given(this.glyph_count as usize)
        },
    }
}

table! {
    /// A coverage in format 2.
    pub Coverage2 { // CoverageFormat2
        format       (u16), // coverageFormat
        record_count (u16), // rangeCount

        records (Vec<Record>) |this, tape| { // rangeRecords
            tape.take_given(this.record_count as usize)
        },
    }
}

table! {
    /// A coverage record.
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

impl crate::value::Read for Coverage {
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Coverage::Format1(tape.take()?),
            2 => Coverage::Format2(tape.take()?),
            value => raise!("found an unknown format of the glyph coverage ({value})"),
        })
    }
}
