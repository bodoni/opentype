use truetype::{Result, Tape, Value};

use layout::Coverage;
use super::ValueRecord;

table! {
    @position
    #[doc = "A table of positioning information defined on a per-glyph bases."]
    pub GlyphInfo { // MathGlyphInfo
        italics_correction_offset      (u16), // MathItalicsCorrectionInfo
        top_accent_offset              (u16), // MathTopAccentAttachment
        extended_shape_coverage_offset (u16), // ExtendedShapeCoverage
        kern_info_offset               (u16), // MathKernInfo

        italics_corrections (Option<ItalicsCorrections>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.italics_correction_offset)        
        },

        top_accent_attachments (Option<TopAccentAttachments>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.top_accent_offset)
        },

        extended_shape_coverage (Option<Coverage>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.extended_shape_coverage_offset)
        },

        kern_info (Option<KernInfo>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.kern_info_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table of italics corrections."]
    pub ItalicsCorrections { // MathItalicsCorrectionInfo
        coverage_offset (u16), // Coverage
        count           (u16), // ItalicsCorrectionCount

        corrections (Vec<ValueRecord>) |this, tape, _| { // ItalicsCorrection
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
    pub TopAccentAttachments { // MathTopAccentAttachment
        coverage_offset (u16), // TopAccentCoverage
        count           (u16), // TopAccentAttachmentCount

        attachments (Vec<ValueRecord>) |this, tape, _| {    
           tape.take_given(this.count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table of kerning information for mathematical glyphs."]
    pub KernInfo {
        coverage_offset (u16), // MathKernCoverage
        count           (u16), // MathKernCount

        kernings (Vec<KernGlyph>) |this, tape, position| {
            let mut kern_glyphs: Vec<KernGlyph> = Vec::with_capacity(this.count as usize);
            for _ in 0..this.count {
                kern_glyphs.push(try!(tape.take()));
            }
            for kern in &mut kern_glyphs {
                kern.top_right = jump_take_maybe!(@unwrap tape, position, kern.top_right_offset);
                kern.top_left = jump_take_maybe!(@unwrap tape, position, kern.top_left_offset);
                kern.bottom_right = jump_take_maybe!(@unwrap tape, position, kern.bottom_right_offset);
                kern.bottom_left = jump_take_maybe!(@unwrap tape, position, kern.bottom_left_offset);
            }
            Ok(kern_glyphs)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @define
    #[doc = "Kerning information for a mathematical glyph."]
    pub KernGlyph { // MathKernInfoRecord
        top_right_offset    (u16), // TopRightMathKern
        top_left_offset     (u16), // TopLeftMathKern
        bottom_right_offset (u16), // BottomRightMathKern
        bottom_left_offset  (u16), // BottomLeftMathKern
        top_right           (Option<Kern>),
        top_left            (Option<Kern>),
        bottom_right        (Option<Kern>),
        bottom_left         (Option<Kern>),
    }
}

table! {
    #[doc = "A table of kerning values for a various glyph heights."]
    pub Kern {
        count (u16), // HeightCount

        correction_heights (Vec<ValueRecord>) |this, tape| {
            tape.take_given(this.count as usize)
        },

        kern_values (Vec<ValueRecord>) |this, tape| {
            tape.take_given(this.count as usize + 1)
        },
    }
}

impl Value for KernGlyph {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let top_right_offset = try!(tape.take());
        let top_left_offset = try!(tape.take());
        let bottom_right_offset = try!(tape.take());
        let bottom_left_offset = try!(tape.take());

        Ok(KernGlyph{
            top_right_offset: top_right_offset,
            top_left_offset: top_left_offset,
            bottom_right_offset: bottom_right_offset,
            bottom_left_offset: bottom_left_offset,
            top_right: None,
            top_left: None,
            bottom_right: None,
            bottom_left: None,
        })
    }
}