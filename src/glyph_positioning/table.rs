#![allow(unused_mut, unused_variables)]

use {Result, Tape, Value, Walue};
use glyph_positioning::{
    BaseSet,
    ClassRuleSet,
    LigatureSet,
    Mark1Set,
    Mark2Set,
    Pair,
    PairSet,
    Passage,
    Positioning,
    RuleSet,
    Single,
    SingleFlags,
};
use layout::{Class, Coverage};

/// A table.
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
        format          (u16        ) = { 1 }, // PosFormat
        coverage_offset (u16        ), // Coverage
        value_flags     (SingleFlags), // ValueFormat

        value (Single) |this, tape, position| { // Value
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
        format          (u16        ) = { 2 }, // PosFormat
        coverage_offset (u16        ), // Coverage
        value_flags     (SingleFlags), // ValueFormat
        value_count     (u16        ), // ValueCount

        values (Vec<Single>) |this, tape, position| { // Value
            let mut values = Vec::with_capacity(this.value_count as usize);
            for i in 0..(this.value_count as usize) {
                values.push(try!(tape.take_given((position, this.value_flags))));
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
        format          (u16        ) = { 1 }, // PosFormat
        coverage_offset (u16        ), // Coverage
        value1_flags    (SingleFlags), // ValueFormat1
        value2_flags    (SingleFlags), // ValueFormat2
        pair_set_count  (u16        ), // PairSetCount

        pair_set_offsets (Vec<u16>) |this, tape, position| { // PairSetOffset
            tape.take_given(this.pair_set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        pair_sets (Vec<PairSet>) |this, tape, position| {
            jump_take_given!(tape, position, this.pair_set_count, this.pair_set_offsets,
                             (position, this.value1_flags, this.value2_flags))
        },
    }
}

table! {
    @position
    #[doc = "A table for adjusting pairs of glyphs in format 2."]
    pub PairAdjustment2 { // PairPosFormat2
        format          (u16        ) = { 2 }, // PosFormat
        coverage_offset (u16        ), // Coverage
        value1_flags    (SingleFlags), // ValueFormat1
        value2_flags    (SingleFlags), // ValueFormat2
        class1_offset   (u16        ), // ClassDef1
        class2_offset   (u16        ), // ClassDef2
        class1_count    (u16        ), // Class1Count
        class2_count    (u16        ), // Class2Count

        pair_sets (Vec<Vec<Pair>>) |this, tape, position| { // Class1Record
            let mut values = Vec::with_capacity(this.class1_count as usize);
            for i in 0..(this.class1_count as usize) {
                let mut records = Vec::with_capacity(this.class2_count as usize);
                for j in 0..(this.class2_count as usize) {
                    records.push(try!(tape.take_given((position, this.value1_flags,
                                                       this.value2_flags))));
                }
                values.push(records);
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
        format          (u16) = { 1 }, // PosFormat
        coverage_offset (u16), // Coverage
        passage_count   (u16), // EntryExitCount

        passages (Vec<Passage>) |this, tape, position| { // EntryExitRecord
            let mut values = Vec::with_capacity(this.passage_count as usize);
            for i in 0..(this.passage_count as usize) {
                values.push(try!(tape.take_given(position)));
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
        format               (u16) = { 1 }, // PosFormat
        mark_coverage_offset (u16), // MarkCoverage
        base_coverage_offset (u16), // BaseCoverage
        class_count          (u16), // ClassCount
        mark_set_offset      (u16), // MarkArray
        base_set_offset      (u16), // BaseArray

        mark_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark_coverage_offset)
        },

        base_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.base_coverage_offset)
        },

        mark_set (Mark1Set) |this, tape, position| {
            jump_take!(tape, position, this.mark_set_offset)
        },

        base_set (BaseSet) |this, tape, position| {
            jump_take_given!(tape, position, this.base_set_offset, this.class_count)
        },
    }
}

table! {
    @position
    #[doc = "A table for attaching combining marks to ligatures."]
    pub MarkToLigatureAttachment { // MarkLigPosFormat1
        format                   (u16) = { 1 }, // PosFormat
        mark_coverage_offset     (u16), // MarkCoverage
        ligature_coverage_offset (u16), // LigatureCoverage
        class_count              (u16), // ClassCount
        mark_set_offset          (u16), // MarkArray
        ligature_set_offset      (u16), // LigatureArray

        mark_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark_coverage_offset)
        },

        ligature_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.ligature_coverage_offset)
        },

        mark_set (Mark1Set) |this, tape, position| {
            jump_take!(tape, position, this.mark_set_offset)
        },

        ligature_set (LigatureSet) |this, tape, position| {
            jump_take_given!(tape, position, this.ligature_set_offset, this.class_count)
        },
    }
}

table! {
    @position
    #[doc = "A table for attaching combining marks to other marks."]
    pub MarkToMarkAttachment { // MarkMarkPosFormat1
        format                (u16) = { 1 }, // PosFormat
        mark1_coverage_offset (u16), // Mark1Coverage
        mark2_coverage_offset (u16), // Mark2Coverage
        class_count           (u16), // ClassCount
        mark1_set_offset      (u16), // Mark1Array
        mark2_set_offset      (u16), // Mark2Array

        mark1_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark1_coverage_offset)
        },

        mark2_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.mark2_coverage_offset)
        },

        mark1_set (Mark1Set) |this, tape, position| {
            jump_take!(tape, position, this.mark1_set_offset)
        },

        mark2_set (Mark2Set) |this, tape, position| {
            jump_take_given!(tape, position, this.mark2_set_offset, this.class_count)
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
    pub ContextPositioning1 { // ContextPosFormat1
        format          (u16) = { 1 }, // PosFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // PosRuleSetCount

        set_offsets (Vec<u16>) |this, tape, position| { // PosRuleSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<RuleSet>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for positioning glyphs in a context in format 2."]
    pub ContextPositioning2 { // ContextPosFormat2
        format          (u16) = { 2 }, // PosFormat
        coverage_offset (u16), // Coverage
        class_offset    (u16), // ClassDef
        set_count       (u16), // PosClassSetCnt

        set_offsets (Vec<u16>) |this, tape, _| { // PosClassSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<Option<ClassRuleSet>>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for positioning glyphs in a context in format 3."]
    pub ContextPositioning3 { // ContextPosFormat3
        format          (u16) = { 3 }, // PosFormat
        glyph_count     (u16), // GlyphCount
        operation_count (u16), // PosCount

        coverage_offsets (Vec<u16>) |this, tape, _| { // Coverage
            tape.take_given(this.glyph_count as usize)
        },

        operations (Vec<Positioning>) |this, tape, _| { // PosLookupRecord
            tape.take_given(this.operation_count as usize)
        },

        coverages (Vec<Coverage>) |this, tape, position| {
            jump_take!(tape, position, this.glyph_count, this.coverage_offsets)
        },
    }
}

table! {
    #[doc = "A table for positioning glyphs in a chaining context."]
    pub ChainContextPositioning {
    }
}

table! {
    #[doc = "A table for other types of positioning."]
    pub ExtensionPositioning {
    }
}

impl Walue<u16> for Table {
    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Table::SingleAdjustment(try!(tape.take())),
            2 => Table::PairAdjustment(try!(tape.take())),
            3 => Table::CursiveAttachment(try!(tape.take())),
            4 => Table::MarkToBaseAttachment(try!(tape.take())),
            5 => Table::MarkToLigatureAttachment(try!(tape.take())),
            6 => Table::MarkToMarkAttachment(try!(tape.take())),
            7 => Table::ContextPositioning(try!(tape.take())),
            8 => Table::ChainContextPositioning(try!(tape.take())),
            9 => Table::ExtensionPositioning(try!(tape.take())),
            _ => raise!("found an unknown glyph-positioning type"),
        })
    }
}

impl Value for SingleAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => SingleAdjustment::Format1(try!(tape.take())),
            2 => SingleAdjustment::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the single-adjustment table"),
        })
    }
}

impl Value for PairAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => PairAdjustment::Format1(try!(tape.take())),
            2 => PairAdjustment::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the pair-adjustment table"),
        })
    }
}

impl Value for ContextPositioning {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => ContextPositioning::Format1(try!(tape.take())),
            2 => ContextPositioning::Format2(try!(tape.take())),
            3 => ContextPositioning::Format3(try!(tape.take())),
            _ => raise!("found an unknown format of the context-positioning table"),
        })
    }
}
