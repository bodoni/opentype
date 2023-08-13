//! The [glyph substitution][1].
//!
//! [1]: https://learn.microsoft.com/en-gb/typography/opentype/spec/gsub

mod element;

use truetype::GlyphID;

use crate::layout::{Class, Coverage, Directory};
use crate::{Result, Tape, Value, Walue};

pub use element::*;

/// A glyph-substitution table.
pub type GlyphSubstitution = Directory<Table>;

/// An inner table of a glyph-substitution table.
#[derive(Clone, Debug)]
pub enum Table {
    SingleSubstitution(SingleSubstitution),
    MultipleSubstitution(MultipleSubstitution),
    AlternateSubstitution(AlternateSubstitution),
    LigatureSubstitution(LigatureSubstitution),
    ContextSubstitution(ContextSubstitution),
    ChainContextSubstitution(ChainContextSubstitution),
    ExtensionSubstitution(ExtensionSubstitution),
    ReverseChainContextSubstitution(ReverseChainContextSubstitution),
}

/// A table for substituting one glyph with one glyph.
#[derive(Clone, Debug)]
pub enum SingleSubstitution {
    /// Format 1.
    Format1(SingleSubstitution1),
    /// Format 2.
    Format2(SingleSubstitution2),
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one glyph in format 1."]
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
    #[doc = "A table for substituting one glyph with one glyph in format 2."]
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
    #[doc = "A table for substituting one glyph with more than one glyph."]
    pub MultipleSubstitution { // MultipleSubstFormat1
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
    #[doc = "A table for substituting one glyph with one of many glyphs."]
    pub AlternateSubstitution { // AlternateSubstFormat1
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // AlternateSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // AlternateSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<Alternates>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting multiple glyphs with one glyph."]
    pub LigatureSubstitution { // LigatureSubstFormat1
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // LigSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // LigatureSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<Ligatures>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

/// A table for substituting glyphs in a context.
#[derive(Clone, Debug)]
pub enum ContextSubstitution {
    /// Format 1.
    Format1(ContextSubstitution1),
    /// Format 2.
    Format2(ContextSubstitution2),
    /// Format 3.
    Format3(ContextSubstitution3),
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a context in format 1."]
    pub ContextSubstitution1 { // ContextSubstFormat1
        format          (u16), // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // SubRuleSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // SubRuleSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<Rules>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a context in format 2."]
    pub ContextSubstitution2 { // ContextSubstFormat2
        format          (u16), // SubstFormat
        coverage_offset (u16), // Coverage
        class_offset    (u16), // ClassDef
        set_count       (u16), // SubClassSetCnt

        set_offsets (Vec<u16>) |this, tape, _| { // SubClassSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<Option<ClassRules>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a context in format 3."]
    pub ContextSubstitution3 { // ContextSubstFormat3
        format          (u16), // SubstFormat
        glyph_count     (u16), // GlyphCount
        operation_count (u16), // SubstCount

        coverage_offsets (Vec<u16>) |this, tape, _| { // Coverage
            tape.take_given(this.glyph_count as usize)
        },

        operations (Vec<Substitution>) |this, tape, _| { // SubstLookupRecord
            tape.take_given(this.operation_count as usize)
        },

        coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.glyph_count, this.coverage_offsets)
        },
    }
}

/// A table for substituting glyphs in a chaining context.
#[derive(Clone, Debug)]
pub enum ChainContextSubstitution {
    /// Format 1.
    Format1(ChainContextSubstitution1),
    /// Format 2.
    Format2(ChainContextSubstitution2),
    /// Format 3.
    Format3(ChainContextSubstitution3),
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a chaining context in format 1."]
    pub ChainContextSubstitution1 { // ChainContextSubstFormat1
        format          (u16), // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // ChainSubRuleSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // ChainSubRuleSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<ChainRules>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a chaining context in format 2."]
    pub ChainContextSubstitution2 { // ChainContextSubstFormat2
        format                (u16), // SubstFormat
        coverage_offset       (u16), // Coverage
        backward_class_offset (u16), // BacktrackClassDef
        input_class_offset    (u16), // InputClassDef
        forward_class_offset  (u16), // LookaheadClassDef
        set_count             (u16), // ChainSubClassSetCnt

        set_offsets (Vec<u16>) |this, tape, _| { // ChainSubClassSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        backward_class (Class) |this, tape, position| {
            jump_take!(tape, position, this.backward_class_offset)
        },

        input_class (Class) |this, tape, position| {
            jump_take!(tape, position, this.input_class_offset)
        },

        forward_class (Class) |this, tape, position| {
            jump_take!(tape, position, this.forward_class_offset)
        },

        sets (Vec<Option<ChainClassRules>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting glyphs in a chaining context in format 3."]
    pub ChainContextSubstitution3 { // ChainContextSubstFormat3
        format               (u16), // SubstFormat
        backward_glyph_count (u16), // BacktrackGlyphCount

        backward_coverage_offsets (Vec<u16>) |this, tape, _| { // Coverage
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // InputGlyphCount

        input_coverage_offsets (Vec<u16>) |this, tape, _| { // Coverage
            tape.take_given(this.input_glyph_count as usize)
        },

        forward_glyph_count (u16), // LookaheadGlyphCount

        forward_coverage_offsets (Vec<u16>) |this, tape, _| { // Coverage
            tape.take_given(this.forward_glyph_count as usize)
        },

        operation_count (u16), // SubstCount

        operations (Vec<Substitution>) |this, tape, _| { // SubstLookupRecord
            tape.take_given(this.operation_count as usize)
        },

        backward_coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.backward_glyph_count, this.backward_coverage_offsets)
        },

        input_coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.input_glyph_count, this.input_coverage_offsets)
        },

        forward_coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.forward_glyph_count, this.forward_coverage_offsets)
        },
    }
}

table! {
    #[doc = "A table for other types of substitution."]
    pub ExtensionSubstitution { // ExtensionSubstFormat1
        format (u16) = { 1 }, // SubstFormat
        kind   (u16), // ExtensionLookupType
        offset (u32), // ExtensionOffset
    }
}

table! {
    @position
    #[doc = "A table for substituting glyphs in reverse order in a chaining context."]
    pub ReverseChainContextSubstitution { // ReverseChainSingleSubstFormat1
        format               (u16), // SubstFormat
        coverage_offset      (u16), // Coverage
        backward_glyph_count (u16), // BacktrackGlyphCount

        backward_coverage_offsets (Vec<u16>) |this, tape, _| { // Coverage
            tape.take_given(this.backward_glyph_count as usize)
        },

        forward_glyph_count (u16), // LookaheadGlyphCount

        forward_coverage_offsets (Vec<u16>) |this, tape, _| { // Coverage
            tape.take_given(this.forward_glyph_count as usize)
        },

        glyph_count (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape, _| { // Substitute
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
            1 => Table::SingleSubstitution(tape.take()?),
            2 => Table::MultipleSubstitution(tape.take()?),
            3 => Table::AlternateSubstitution(tape.take()?),
            4 => Table::LigatureSubstitution(tape.take()?),
            5 => Table::ContextSubstitution(tape.take()?),
            6 => Table::ChainContextSubstitution(tape.take()?),
            7 => Table::ExtensionSubstitution(tape.take()?),
            8 => Table::ReverseChainContextSubstitution(tape.take()?),
            value => raise!("found an unknown glyph-substitution type ({value})"),
        })
    }
}

impl Value for SingleSubstitution {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => SingleSubstitution::Format1(tape.take()?),
            2 => SingleSubstitution::Format2(tape.take()?),
            value => raise!("found an unknown format of the single-substitution table ({value})"),
        })
    }
}

impl Value for ContextSubstitution {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => ContextSubstitution::Format1(tape.take()?),
            2 => ContextSubstitution::Format2(tape.take()?),
            3 => ContextSubstitution::Format3(tape.take()?),
            value => raise!("found an unknown format of the context-substitution table ({value})"),
        })
    }
}

impl Value for ChainContextSubstitution {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => ChainContextSubstitution::Format1(tape.take()?),
            2 => ChainContextSubstitution::Format2(tape.take()?),
            3 => ChainContextSubstitution::Format3(tape.take()?),
            value => raise!(
                "found an unknown format of the chaining-context-substitution table ({value})"
            ),
        })
    }
}
