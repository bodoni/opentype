use {Result, Tape, Value};
use glyph_positioning::value::{Flags, One, Pair, PairSet};
use layout::{Class, Coverage};

/// A table for adjusting pairs of glyphs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PairAdjustment {
    /// Format 1.
    Format1(PairAdjustment1),
    /// Format 2.
    Format2(PairAdjustment2),
}

table! {
    #[doc = "A table for adjusting pairs of glyphs in format 1."]
    pub PairAdjustment1 {
        format          (u16  ), // PosFormat
        coverage_offset (u16  ), // Coverage
        value1_flags    (Flags), // ValueFormat1
        value2_flags    (Flags), // ValueFormat2
        pair_set_count  (u16  ), // PairSetCount

        pair_set_offsets (Vec<u16>) |this, tape, position| { // PairSetOffset
            tape.take_given(this.pair_set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            try!(tape.jump(position + this.coverage_offset as u64));
            tape.take()
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
    #[doc = "A table for adjusting pairs of glyphs in format 2."]
    pub PairAdjustment2 {
        format          (u16  ), // PosFormat
        coverage_offset (u16  ), // Coverage
        value1_flags    (Flags), // ValueFormat1
        value2_flags    (Flags), // ValueFormat2
        class1_offset   (u16  ), // ClassDef1
        class2_offset   (u16  ), // ClassDef2
        class1_count    (u16  ), // Class1Count
        class2_count    (u16  ), // Class2Count

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
            try!(tape.jump(position + this.coverage_offset as u64));
            tape.take()
        },

        class1 (Class) |this, tape, position| {
            try!(tape.jump(position + this.class1_offset as u64));
            tape.take()
        },

        class2 (Class) |this, tape, position| {
            try!(tape.jump(position + this.class2_offset as u64));
            tape.take()
        },
    }
}

/// A table for adjusting single glyphs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SingleAdjustment {
    /// Format 1.
    Format1(SingleAdjustment1),
    /// Format 2.
    Format2(SingleAdjustment2),
}

table! {
    #[doc = "A table for adjusting single glyphs in format 1."]
    pub SingleAdjustment1 {
        format          (u16  ), // PosFormat
        coverage_offset (u16  ), // Coverage
        value_flags     (Flags), // ValueFormat

        value (One) |this, tape, position| { // Value
            tape.take_given(this.value_flags)
        },

        coverage (Coverage) |this, tape, position| {
            try!(tape.jump(position + this.coverage_offset as u64));
            tape.take()
        },
    }
}

table! {
    #[doc = "A table for adjusting single glyphs in format 2."]
    pub SingleAdjustment2 {
        format          (u16  ), // PosFormat
        coverage_offset (u16  ), // Coverage
        value_flags     (Flags), // ValueFormat
        value_count     (u16  ), // ValueCount

        values (Vec<One>) |this, tape, position| { // Value
            let mut values = Vec::with_capacity(this.value_count as usize);
            for i in 0..(this.value_count as usize) {
                values.push(try!(tape.take_given(this.value_flags)));
            }
            Ok(values)
        },

        coverage (Coverage) |this, tape, position| {
            try!(tape.jump(position + this.coverage_offset as u64));
            tape.take()
        },
    }
}

macro_rules! read_flags(
    ($tape:ident) => ({
        let flags: Flags = try!($tape.take());
        if flags.is_invalid() {
            raise!("found a malformed adjustment table");
        }
        flags
    });
);

impl Value for PairAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => PairAdjustment::Format1(try!(tape.take())),
            2 => PairAdjustment::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the pair-adjustment table"),
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
