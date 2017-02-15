use truetype::{Result, Tape, Value};

use layout::{Correction, Coverage};

table! {
    #[doc = "A glyph attachment."]
    pub Attachment { // AttachPoint
        index_count (u16), // PointCount

        indices (Vec<u16>) |this, tape| { // PointIndex
            tape.take_given(this.index_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of glyph attachments."]
    pub Attachments { // AttachList
        coverage_offset (u16), // Coverage
        count           (u16), // GlyphCount

        offsets (Vec<u16>) |this, tape, _| { // AttachPoint
            tape.take_given(this.count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        records (Vec<Attachment>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

/// A ligature caret.
#[derive(Clone, Debug)]
pub enum Caret {
    /// Format 1.
    Format1(Caret1),
    /// Format 2.
    Format2(Caret2),
    /// Format 3.
    Format3(Caret3),
}

table! {
    /// A ligature caret in format 1.
    #[derive(Copy)]
    pub Caret1 {
        format     (u16), // CaretValueFormat
        coordinate (i16), // Coordinate
    }
}

table! {
    /// A ligature caret in format 2.
    #[derive(Copy)]
    pub Caret2 {
        format (u16), // CaretValueFormat
        index  (u16), // CaretValuePoint
    }
}

table! {
    @position
    /// A ligature caret in format 3.
    pub Caret3 {
        format            (u16), // CaretValueFormat
        coordinate        (i16), // Coordinate
        correction_offset (u16), // DeviceTable

        correction (Correction) |this, tape, position| {
            jump_take!(tape, position, this.correction_offset)
        },
    }
}

table! {
    @position
    #[doc = "A ligature."]
    pub Ligature { // LigGlyph
        caret_count (u16), // CaretCount

        caret_offsets (Vec<u16>) |this, tape, _| { // CaretValue
            tape.take_given(this.caret_count as usize)
        },

        carets (Vec<Caret>) |this, tape, position| {
            jump_take!(tape, position, this.caret_count, this.caret_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A set of ligatures."]
    pub Ligatures { // LigCaretList
        coverage_offset (u16), // Coverage
        count           (u16), // LigGlyphCount

        offsets (Vec<u16>) |this, tape, _| { // LigGlyph
            tape.take_given(this.count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        records (Vec<Ligature>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    @position
    #[doc = "A set of marks."]
    pub Marks { // MarkGlyphSetsTable
        format (u16) = { 1 }, // MarkSetTableFormat
        count  (u16), // MarkSetCount

        coverage_offsets (Vec<u32>) |this, tape, _| { // Coverage
            tape.take_given(this.count as usize)
        },

        coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.coverage_offsets)
        },
    }
}

impl Value for Caret {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u32>()? {
            1 => Caret::Format1(tape.take()?),
            2 => Caret::Format2(tape.take()?),
            3 => Caret::Format3(tape.take()?),
            _ => raise!("found an unknown format of the caret-value table"),
        })
    }
}
