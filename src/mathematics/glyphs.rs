use layout::Coverage;
use super::Quantity;

table! {
    @position
    #[doc = "A table of positioning information defined on a per-glyph basis."]
    pub Glyphs { // MathGlyphInfo
        italics_corrections_offset     (u16), // MathItalicsCorrectionInfo
        top_accent_attachments_offset  (u16), // MathTopAccentAttachment
        extended_shape_coverage_offset (u16), // ExtendedShapeCoverage
        kernings_offset                (u16), // MathKernInfo

        corrections (Corrections) |this, tape, position| {
            jump_take!(tape, position, this.italics_corrections_offset)
        },

        attachments (Attachments) |this, tape, position| {
            jump_take!(tape, position, this.top_accent_attachments_offset)
        },

        extended_shape_coverage (Option<Coverage>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.extended_shape_coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table of italics corrections."]
    pub Corrections { // MathItalicsCorrectionInfo
        coverage_offset (u16), // Coverage
        count           (u16), // ItalicsCorrectionCount

        values (Vec<Quantity>) |this, tape, _| { // ItalicsCorrection
            tape.take_given(this.count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table of horizontal positioning for top accents."]
    pub Attachments { // MathTopAccentAttachment
        coverage_offset (u16), // TopAccentCoverage
        count           (u16), // TopAccentAttachmentCount

        values (Vec<Quantity>) |this, tape, _| {
           tape.take_given(this.count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}
