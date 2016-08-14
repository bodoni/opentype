#![allow(unused_mut, unused_variables)]

use {Result, Tape, Value, Walue};
use glyph_positioning::{Anchor, Connection, Pair, PairSet, Single, ValueFlags};
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
        format          (u16       ) = { 1 }, // PosFormat
        coverage_offset (u16       ), // Coverage
        value_flags     (ValueFlags), // ValueFormat

        value (Single) |this, tape, position| { // Value
            tape.take_given(this.value_flags)
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
        format          (u16       ) = { 2 }, // PosFormat
        coverage_offset (u16       ), // Coverage
        value_flags     (ValueFlags), // ValueFormat
        value_count     (u16       ), // ValueCount

        values (Vec<Single>) |this, tape, position| { // Value
            let mut values = Vec::with_capacity(this.value_count as usize);
            for i in 0..(this.value_count as usize) {
                values.push(try!(tape.take_given(this.value_flags)));
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
        format          (u16       ) = { 1 }, // PosFormat
        coverage_offset (u16       ), // Coverage
        value1_flags    (ValueFlags), // ValueFormat1
        value2_flags    (ValueFlags), // ValueFormat2
        pair_set_count  (u16       ), // PairSetCount

        pair_set_offsets (Vec<u16>) |this, tape, position| { // PairSetOffset
            tape.take_given(this.pair_set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        pair_sets (Vec<PairSet>) |this, tape, position| {
            let mut values = Vec::with_capacity(this.pair_set_count as usize);
            for i in 0..(this.pair_set_count as usize) {
                try!(tape.jump(position + this.pair_set_offsets[i] as u64));
                values.push(try!(tape.take_given((this.value1_flags, this.value2_flags))));
            }
            Ok(values)
        },
    }
}

table! {
    @position
    #[doc = "A table for adjusting pairs of glyphs in format 2."]
    pub PairAdjustment2 { // PairPosFormat2
        format          (u16       ) = { 2 }, // PosFormat
        coverage_offset (u16       ), // Coverage
        value1_flags    (ValueFlags), // ValueFormat1
        value2_flags    (ValueFlags), // ValueFormat2
        class1_offset   (u16       ), // ClassDef1
        class2_offset   (u16       ), // ClassDef2
        class1_count    (u16       ), // Class1Count
        class2_count    (u16       ), // Class2Count

        pair_sets (Vec<Vec<Pair>>) |this, tape, position| { // Class1Record
            let mut values = Vec::with_capacity(this.class1_count as usize);
            for i in 0..(this.class1_count as usize) {
                let mut records = Vec::with_capacity(this.class2_count as usize);
                for j in 0..(this.class2_count as usize) {
                    records.push(try!(tape.take_given((this.value1_flags, this.value2_flags))));
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
        format           (u16) = { 1 }, // PosFormat
        coverage_offset  (u16), // Coverage
        connection_count (u16), // EntryExitCount

        connections (Vec<Connection>) |this, tape, _| { // EntryExitRecord
            tape.take_given(this.connection_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        entires (Vec<Anchor>) |this, tape, position| {
            jump_take!(tape, position, this.connection_count,
                       i => this.connections[i].entry_offset)
        },

        exits (Vec<Anchor>) |this, tape, position| {
            jump_take!(tape, position, this.connection_count,
                       i => this.connections[i].exit_offset)
        },
    }
}

table! {
    #[doc = "A table for attaching combining marks to base glyphs."]
    pub MarkToBaseAttachment {
    }
}

table! {
    #[doc = "A table for attaching combining marks to ligatures."]
    pub MarkToLigatureAttachment {
    }
}

table! {
    #[doc = "A table for attaching combining marks to other marks."]
    pub MarkToMarkAttachment {
    }
}

table! {
    #[doc = "A table for positioning glyphs in a context."]
    pub ContextPositioning {
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
