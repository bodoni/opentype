//! The [glyph substitution][1].
//!
//! [1]: https://learn.microsoft.com/en-gb/typography/opentype/spec/gsub

mod element;

use truetype::GlyphID;

use crate::layout::{ChainedContext, Context, Coverage, Directory};
use crate::{Result, Tape, Value, Walue};

pub use element::*;

/// A glyph-substitution table.
pub type GlyphSubstitution = Directory<Table>;

/// An inner table of a glyph-substitution table.
#[derive(Clone, Debug)]
pub enum Table {
    Single(Single),
    Multiple(Multiple),
    Alternate(Alternate),
    Ligature(Ligature),
    Context(Context),
    ChainedContext(ChainedContext),
    Extension(Extension),
    ReverseChainedContext(ReverseChainedContext),
}

/// A table for substituting one glyph with one glyph.
#[derive(Clone, Debug)]
pub enum Single {
    /// Format 1.
    Format1(Single1),
    /// Format 2.
    Format2(Single2),
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one glyph in format 1."]
    pub Single1 { // SingleSubstFormat1
        format          (u16), // SubstFormat
        coverage_offset (u16), // Coverage
        delta_glyph_id  (i16), // DeltaGlyphID

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one glyph in format 2."]
    pub Single2 { // SingleSubstFormat2
        format          (u16), // SubstFormat
        coverage_offset (u16), // Coverage
        glyph_count     (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape, _| { // Substitute
            tape.take_given(this.glyph_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with more than one glyph."]
    pub Multiple { // MultipleSubstFormat1
        format           (u16) = { 1 }, // SubstFormat
        coverage_offset  (u16), // Coverage
        sequence_count   (u16), // SequenceCount

        sequence_offsets (Vec<u16>) |this, tape, _| { // Sequence
            tape.take_given(this.sequence_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sequences (Vec<Sequence>) |this, tape, position| {
            jump_take!(tape, position, this.sequence_count, this.sequence_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one of several glyphs."]
    pub Alternate { // AlternateSubstFormat1
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        rule_count      (u16), // AlternateSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // AlternateSet
            tape.take_given(this.rule_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        rules (Vec<Alternates>) |this, tape, position| {
            jump_take!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting multiple glyphs with one glyph."]
    pub Ligature { // LigatureSubstFormat1
        format          (u16) = { 1 }, // substFormat
        coverage_offset (u16), // coverageOffset
        rule_count      (u16), // ligatureSetCount

        rule_offsets (Vec<u16>) |this, tape, _| { // ligatureSetOffsets
            tape.take_given(this.rule_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        rules (Vec<Ligatures>) |this, tape, position| {
            jump_take!(tape, position, this.rule_count, this.rule_offsets)
        },
    }
}

table! {
    #[doc = "A table for other types of substitution."]
    pub Extension { // ExtensionSubstFormat1
        format (u16) = { 1 }, // SubstFormat
        kind   (u16), // ExtensionLookupType
        offset (u32), // ExtensionOffset
    }
}

table! {
    @position
    #[doc = "A table for reversed chained contextual substitution."]
    pub ReverseChainedContext { // ReverseChainSingleSubstFormat1
        format               (u16), // substFormat
        coverage_offset      (u16), // coverageOffset
        backward_glyph_count (u16), // backtrackGlyphCount

        backward_coverage_offsets (Vec<u16>) |this, tape, _| { // backtrackCoverageOffsets
            tape.take_given(this.backward_glyph_count as usize)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_coverage_offsets (Vec<u16>) |this, tape, _| { // lookaheadCoverageOffsets
            tape.take_given(this.forward_glyph_count as usize)
        },

        glyph_count (u16), // glyphCount

        glyph_ids (Vec<GlyphID>) |this, tape, _| { // substituteGlyphIDs
            tape.take_given(this.glyph_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        backward_coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.backward_glyph_count, this.backward_coverage_offsets)
        },

        forward_coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.forward_glyph_count, this.forward_coverage_offsets)
        },
    }
}

impl Walue<'static> for Table {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Self::Single(tape.take()?),
            2 => Self::Multiple(tape.take()?),
            3 => Self::Alternate(tape.take()?),
            4 => Self::Ligature(tape.take()?),
            5 => Self::Context(tape.take()?),
            6 => Self::ChainedContext(tape.take()?),
            7 => Self::Extension(tape.take()?),
            8 => Self::ReverseChainedContext(tape.take()?),
            value => raise!("found an unknown glyph-substitution type ({value})"),
        })
    }
}

impl Value for Single {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            value => raise!("found an unknown format of the single-substitution table ({value})"),
        })
    }
}
