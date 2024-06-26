//! The [glyph substitution][1].
//!
//! [1]: https://learn.microsoft.com/en-gb/typography/opentype/spec/gsub

mod element;

use truetype::GlyphID;

use crate::layout::{ChainedContext, Context, Coverage, Directory};
use crate::Result;

pub use element::*;

/// A glyph substitution.
pub type GlyphSubstitution = Directory<Type>;

/// A glyph-substitution type.
#[derive(Clone, Debug)]
pub enum Type {
    SingleSubstitution(SingleSubstitution),
    MultipleSubstitution(MultipleSubstitution),
    AlternateSubstitution(AlternateSubstitution),
    LigatureSubstitution(LigatureSubstitution),
    ContextualSubstitution(Context),
    ChainedContextualSubstitution(ChainedContext),
    ExtensionSubstitution(ExtensionSubstitution),
    ReverseChainedContextualSubstibution(ReverseChainedContextualSubstibution),
}

/// A single substitution.
#[derive(Clone, Debug)]
pub enum SingleSubstitution {
    /// Format 1.
    Format1(SingleSubstitution1),
    /// Format 2.
    Format2(SingleSubstitution2),
}

table! {
    @position
    /// A single substitution in format 1.
    pub SingleSubstitution1 { // SingleSubstFormat1
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
    /// A single substitution in format 2.
    pub SingleSubstitution2 { // SingleSubstFormat2
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
    /// A multiple substitution.
    pub MultipleSubstitution { // MultipleSubstFormat1
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        record_count    (u16), // SequenceCount

        record_offsets (Vec<u16>) |this, tape, _| { // Sequence
            tape.take_given(this.record_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        records (Vec<Sequence>) |this, tape, position| {
            jump_take!(tape, position, this.record_count, this.record_offsets)
        },
    }
}

table! {
    @position
    /// An alternate substitution.
    pub AlternateSubstitution { // AlternateSubstFormat1
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        record_count    (u16), // AlternateSetCount

        record_offsets (Vec<u16>) |this, tape, _| { // AlternateSet
            tape.take_given(this.record_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        records (Vec<Alternates>) |this, tape, position| {
            jump_take!(tape, position, this.record_count, this.record_offsets)
        },
    }
}

table! {
    @position
    /// A ligature substitution.
    pub LigatureSubstitution { // LigatureSubstFormat1
        format          (u16) = { 1 }, // substFormat
        coverage_offset (u16), // coverageOffset
        record_count    (u16), // ligatureSetCount

        record_offsets (Vec<u16>) |this, tape, _| { // ligatureSetOffsets
            tape.take_given(this.record_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        records (Vec<Ligatures>) |this, tape, position| {
            jump_take!(tape, position, this.record_count, this.record_offsets)
        },
    }
}

table! {
    /// An extension substitution.
    pub ExtensionSubstitution { // ExtensionSubstFormat1
        format (u16) = { 1 }, // SubstFormat
        r#type (u16), // ExtensionLookupType
        offset (u32), // ExtensionOffset
    }
}

table! {
    @position
    /// A reversed chained contextual substitution.
    pub ReverseChainedContextualSubstibution { // ReverseChainSingleSubstFormat1
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

impl crate::walue::Read<'static> for Type {
    type Parameter = u16;

    fn read<T: crate::tape::Read>(tape: &mut T, r#type: u16) -> Result<Self> {
        Ok(match r#type {
            1 => Self::SingleSubstitution(tape.take()?),
            2 => Self::MultipleSubstitution(tape.take()?),
            3 => Self::AlternateSubstitution(tape.take()?),
            4 => Self::LigatureSubstitution(tape.take()?),
            5 => Self::ContextualSubstitution(tape.take()?),
            6 => Self::ChainedContextualSubstitution(tape.take()?),
            7 => Self::ExtensionSubstitution(tape.take()?),
            8 => Self::ReverseChainedContextualSubstibution(tape.take()?),
            value => raise!("found an unknown type of glyph substitution ({value})"),
        })
    }
}

impl crate::value::Read for SingleSubstitution {
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => Self::Format1(tape.take()?),
            2 => Self::Format2(tape.take()?),
            value => raise!("found an unknown format of the single substitution ({value})"),
        })
    }
}
