//! The [glyph positioning][1].
//!
//! [1]: https://learn.microsoft.com/en-gb/typography/opentype/spec/gpos

mod element;

pub use element::*;

use crate::layout::{Class, Coverage, Directory};
use crate::{Result, Tape, Value, Walue};

/// A glyph-positioning table.
pub type GlyphPositioning = Directory<Table>;

/// An inner table of a glyph-positioning table.
#[derive(Clone, Debug)]
pub enum Table {
    SingleAdjustment(SingleAdjustment),
    PairAdjustment(PairAdjustment),
    CursiveAttachment(CursiveAttachment),
    MarkToBaseAttachment(MarkToBaseAttachment),
    MarkToLigatureAttachment(MarkToLigatureAttachment),
    MarkToMarkAttachment(MarkToMarkAttachment),
    ContextPositioning(ContextPositioning),
    ChainContextPositioning(ChainContextPositioning),
    ExtensionPositioning(ExtensionPositioning),
}

/// A table for adjusting single glyphs.
#[derive(Clone, Debug)]
pub enum SingleAdjustment {
    /// Format 1.
    Format1(SingleAdjustment1),
    /// Format 2.
    Format2(SingleAdjustment2),
}

table! {
    @position
    #[doc = "A table for adjusting single glyphs in format 1."]
    pub SingleAdjustment1 { // SinglePosFormat1
        format          (u16        ), // posFormat
        coverage_offset (u16        ), // coverageOffset
        value_flags     (SingleFlags), // valueFormat

        value (Single) |this, tape, position| { // valueRecord
            tape.take_given((position, this.value_flags))
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for adjusting single glyphs in format 2."]
    pub SingleAdjustment2 { // SinglePosFormat2
        format          (u16        ), // posFormat
        coverage_offset (u16        ), // coverageOffset
        value_flags     (SingleFlags), // valueFormat
        value_count     (u16        ), // valueCount

        values (Vec<Single>) |this, tape, position| { // valueRecords
            let mut values = Vec::with_capacity(this.value_count as usize);
            for _ in 0..(this.value_count as usize) {
                values.push(tape.take_given((position, this.value_flags))?);
            }
            Ok(values)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

/// A table for adjusting pairs of glyphs.
#[derive(Clone, Debug)]
pub enum PairAdjustment {
    /// Format 1.
    Format1(PairAdjustment1),
    /// Format 2.
    Format2(PairAdjustment2),
}

table! {
    @position
    #[doc = "A table for adjusting pairs of glyphs in format 1."]
    pub PairAdjustment1 { // PairPosFormat1
        format          (u16        ), // posFormat
        coverage_offset (u16        ), // coverageOffset
        value1_flags    (SingleFlags), // valueFormat1
        value2_flags    (SingleFlags), // valueFormat2
        set_count       (u16        ), // pairSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // pairSetOffsets
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<Pair1s>) |this, tape, position| {
            jump_take_given!(tape, position, this.set_count, this.set_offsets,
                             (position, this.value1_flags, this.value2_flags))
        },
    }
}

table! {
    @position
    #[doc = "A table for adjusting pairs of glyphs in format 2."]
    pub PairAdjustment2 { // PairPosFormat2
        format          (u16        ), // posFormat
        coverage_offset (u16        ), // coverageOffset
        value1_flags    (SingleFlags), // valueFormat1
        value2_flags    (SingleFlags), // valueFormat2
        class1_offset   (u16        ), // classDef1Offset
        class2_offset   (u16        ), // classDef2Offset
        class1_count    (u16        ), // class1Count
        class2_count    (u16        ), // class2Count

        sets (Vec<Pair2s>) |this, tape, position| { // class1Records
            let mut values = Vec::with_capacity(this.class1_count as usize);
            for _ in 0..(this.class1_count as usize) {
                values.push(tape.take_given((position, this.class2_count,
                                             this.value1_flags, this.value2_flags))?);
            }
            Ok(values)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        class1 (Class) |this, tape, position| {
            jump_take!(tape, position, this.class1_offset)
        },

        class2 (Class) |this, tape, position| {
            jump_take!(tape, position, this.class2_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for attaching cursive glyphs."]
    pub CursiveAttachment { // CursivePosFormat1
        format          (u16) = { 1 }, // posFormat
        coverage_offset (u16), // coverageOffset
        passage_count   (u16), // entryExitCount

        passages (Vec<Passage>) |this, tape, position| { // entryExitRecords
            let mut values = Vec::with_capacity(this.passage_count as usize);
            for _ in 0..(this.passage_count as usize) {
                values.push(tape.take_given(position)?);
            }
            Ok(values)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for attaching combining marks to base glyphs."]
    pub MarkToBaseAttachment { // MarkBasePosFormat1
        format               (u16) = { 1 }, // posFormat
        mark_coverage_offset (u16), // markCoverageOffset
        base_coverage_offset (u16), // baseCoverageOffset
        mark_class_count     (u16), // markClassCount
        mark_offset          (u16), // markArrayOffset
        base_offset          (u16), // baseArrayOffset

        mark_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark_coverage_offset)
        },

        base_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.base_coverage_offset)
        },

        marks (Mark1s) |this, tape, position| {
            jump_take!(tape, position, this.mark_offset)
        },

        bases (Bases) |this, tape, position| {
            jump_take_given!(tape, position, this.base_offset, this.mark_class_count)
        },
    }
}

table! {
    @position
    #[doc = "A table for attaching combining marks to ligatures."]
    pub MarkToLigatureAttachment { // MarkLigPosFormat1
        format                   (u16) = { 1 }, // posFormat
        mark_coverage_offset     (u16), // markCoverageOffset
        ligature_coverage_offset (u16), // ligatureCoverageOffset
        mark_class_count         (u16), // markClassCount
        mark_offset              (u16), // markArrayOffset
        ligature_offset          (u16), // ligatureArrayOffset

        mark_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark_coverage_offset)
        },

        ligature_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.ligature_coverage_offset)
        },

        marks (Mark1s) |this, tape, position| {
            jump_take!(tape, position, this.mark_offset)
        },

        ligatures (Ligatures) |this, tape, position| {
            jump_take_given!(tape, position, this.ligature_offset, this.mark_class_count)
        },
    }
}

table! {
    @position
    #[doc = "A table for attaching combining marks to other marks."]
    pub MarkToMarkAttachment { // MarkMarkPosFormat1
        format                (u16) = { 1 }, // posFormat
        mark1_coverage_offset (u16), // mark1CoverageOffset
        mark2_coverage_offset (u16), // mark2CoverageOffset
        mark_class_count      (u16), // markClassCount
        mark1_offset          (u16), // mark1ArrayOffset
        mark2_offset          (u16), // mark2ArrayOffset

        mark1_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark1_coverage_offset)
        },

        mark2_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark2_coverage_offset)
        },

        mark1s (Mark1s) |this, tape, position| {
            jump_take!(tape, position, this.mark1_offset)
        },

        mark2s (Mark2s) |this, tape, position| {
            jump_take_given!(tape, position, this.mark2_offset, this.mark_class_count)
        },
    }
}

/// A table for positioning glyphs in a context.
#[derive(Clone, Debug)]
pub enum ContextPositioning {
    /// Format 1.
    Format1(ContextPositioning1),
    /// Format 2.
    Format2(ContextPositioning2),
    /// Format 3.
    Format3(ContextPositioning3),
}

table! {
    @position
    #[doc = "A table for positioning glyphs in a context in format 1."]
    pub ContextPositioning1 { // SequenceContextFormat1
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        set_count       (u16), // seqRuleSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // seqRuleSetOffsets
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
    #[doc = "A table for positioning glyphs in a context in format 2."]
    pub ContextPositioning2 { // SequenceContextFormat2
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        class_offset    (u16), // classDefOffset
        set_count       (u16), // classSeqRuleSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // classSeqRuleSetOffsets
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
    #[doc = "A table for positioning glyphs in a context in format 3."]
    pub ContextPositioning3 { // SequenceContextFormat3
        format          (u16), // format
        glyph_count     (u16), // glyphCount
        operation_count (u16), // seqLookupCount

        coverage_offsets (Vec<u16>) |this, tape, _| { // coverageOffsets
            tape.take_given(this.glyph_count as usize)
        },

        operations (Vec<Positioning>) |this, tape, _| { // seqLookupRecords
            tape.take_given(this.operation_count as usize)
        },

        coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.glyph_count, this.coverage_offsets)
        },
    }
}

/// A table for positioning glyphs in a chaining context.
#[derive(Clone, Debug)]
pub enum ChainContextPositioning {
    /// Format 1.
    Format1(ChainContextPositioning1),
    /// Format 2.
    Format2(ChainContextPositioning2),
    /// Format 3.
    Format3(ChainContextPositioning3),
}

table! {
    @position
    #[doc = "A table for positioning glyphs in a chaining context in format 1."]
    pub ChainContextPositioning1 { // ChainedSequenceContextFormat1
        format          (u16), // format
        coverage_offset (u16), // coverageOffset
        set_count       (u16), // chainedSeqRuleSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // chainedSeqRuleSetOffsets
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
    #[doc = "A table for positioning glyphs in a chaining context in format 2."]
    pub ChainContextPositioning2 { // ChainedSequenceContextFormat2
        format                (u16), // format
        coverage_offset       (u16), // coverageOffset
        backward_class_offset (u16), // backtrackClassDefOffset
        input_class_offset    (u16), // inputClassDefOffset
        forward_class_offset  (u16), // lookaheadClassDefOffset
        set_count             (u16), // chainedClassSeqRuleSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // chainedClassSeqRuleSetOffsets
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
    #[doc = "A table for positioning glyphs in a chaining context in format 3."]
    pub ChainContextPositioning3 { // ChainedSequenceContextFormat3
        format               (u16), // format
        backward_glyph_count (u16), // backtrackGlyphCount  

        backward_coverage_offsets (Vec<u16>) |this, tape, _| { // backtrackCoverageOffsets
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // inputGlyphCount

        input_coverage_offsets (Vec<u16>) |this, tape, _| { // inputCoverageOffsets
            tape.take_given(this.input_glyph_count as usize)
        },

        forward_glyph_count (u16), // lookaheadGlyphCount

        forward_coverage_offsets (Vec<u16>) |this, tape, _| { // lookaheadCoverageOffsets
            tape.take_given(this.forward_glyph_count as usize)
        },

        operation_count (u16), // seqLookupCount

        operations (Vec<Positioning>) |this, tape, _| { // seqLookupRecords
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
    #[doc = "A table for other types of positioning."]
    pub ExtensionPositioning { // ExtensionPosFormat1
        format (u16) = { 1 }, // posFormat
        kind   (u16), // extensionLookupType
        offset (u32), // extensionOffset
    }
}

impl Walue<'static> for Table {
    type Parameter = u16;

    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Table::SingleAdjustment(tape.take()?),
            2 => Table::PairAdjustment(tape.take()?),
            3 => Table::CursiveAttachment(tape.take()?),
            4 => Table::MarkToBaseAttachment(tape.take()?),
            5 => Table::MarkToLigatureAttachment(tape.take()?),
            6 => Table::MarkToMarkAttachment(tape.take()?),
            7 => Table::ContextPositioning(tape.take()?),
            8 => Table::ChainContextPositioning(tape.take()?),
            9 => Table::ExtensionPositioning(tape.take()?),
            value => raise!("found an unknown glyph-positioning type ({value})"),
        })
    }
}

impl Value for SingleAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => SingleAdjustment::Format1(tape.take()?),
            2 => SingleAdjustment::Format2(tape.take()?),
            value => raise!("found an unknown format of the single-adjustment table ({value})"),
        })
    }
}

impl Value for PairAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => PairAdjustment::Format1(tape.take()?),
            2 => PairAdjustment::Format2(tape.take()?),
            value => raise!("found an unknown format of the pair-adjustment table ({value})"),
        })
    }
}

impl Value for ContextPositioning {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => ContextPositioning::Format1(tape.take()?),
            2 => ContextPositioning::Format2(tape.take()?),
            3 => ContextPositioning::Format3(tape.take()?),
            value => raise!("found an unknown format of the context-positioning table ({value})"),
        })
    }
}

impl Value for ChainContextPositioning {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            1 => ChainContextPositioning::Format1(tape.take()?),
            2 => ChainContextPositioning::Format2(tape.take()?),
            3 => ChainContextPositioning::Format3(tape.take()?),
            value => raise!(
                "found an unknown format of the chaining-context-positioning table ({value})"
            ),
        })
    }
}
