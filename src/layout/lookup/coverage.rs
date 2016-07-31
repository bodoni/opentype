use truetype::GlyphID;

use {Result, Tape, Value};

/// A coverage table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Coverage {
    /// A coverage table of format 1.
    Format1(Coverage1),
    /// A coverage table of format 2.
    Format2(Coverage2),
}

table! {
    #[doc = "A coverage table of format 1."]
    pub Coverage1 {
        format (u16), // CoverageFormat
        count  (u16), // GlyphCount

        glyphs (Vec<GlyphID>) |tape, this| { // GlyphArray
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage table of format 2."]
    pub Coverage2 {
        format (u16), // CoverageFormat
        count  (u16), // RangeCount

        ranges (Vec<Range>) |tape, this| { // RangeRecord
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A glyph range."]
    #[derive(Copy)]
    pub Range {
        start (GlyphID), // Start
        end   (GlyphID), // End
        index (u16    ), // StartCoverageIndex
    }
}

impl Value for Coverage {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => Coverage::Format1(try!(tape.take())),
            2 => Coverage::Format2(try!(tape.take())),
            _ => raise!("found a coverage table of an unsupported format"),
        })
    }
}
