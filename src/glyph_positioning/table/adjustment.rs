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
    @define
    #[doc = "A table for adjusting pairs of glyphs in format 1."]
    pub PairAdjustment1 {
        format           (u16         ), // PosFormat
        coverage_offset  (u16         ), // Coverage
        value1_flags     (Flags       ), // ValueFormat1
        value2_flags     (Flags       ), // ValueFormat2
        pair_set_count   (u16         ), // PairSetCount
        pair_set_offsets (Vec<u16>    ), // PairSetOffset
        coverage         (Coverage    ),
        pair_sets        (Vec<PairSet>),
    }
}

table! {
    @define
    #[doc = "A table for adjusting pairs of glyphs in format 2."]
    pub PairAdjustment2 {
        format          (u16           ), // PosFormat
        coverage_offset (u16           ), // Coverage
        value1_flags    (Flags         ), // ValueFormat1
        value2_flags    (Flags         ), // ValueFormat2
        class1_offset   (u16           ), // ClassDef1
        class2_offset   (u16           ), // ClassDef2
        class1_count    (u16           ), // Class1Count
        class2_count    (u16           ), // Class2Count
        pair_sets       (Vec<Vec<Pair>>), // Class1Record
        coverage        (Coverage      ),
        class1          (Class         ),
        class2          (Class         ),
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
    @define
    #[doc = "A table for adjusting single glyphs in format 1."]
    pub SingleAdjustment1 {
        format          (u16     ), // PosFormat
        coverage_offset (u16     ), // Coverage
        value_flags     (Flags   ), // ValueFormat
        value           (One     ), // Value
        coverage        (Coverage),
    }
}

table! {
    @define
    #[doc = "A table for adjusting single glyphs in format 2."]
    pub SingleAdjustment2 {
        format          (u16     ), // PosFormat
        coverage_offset (u16     ), // Coverage
        value_flags     (Flags   ), // ValueFormat
        value_count     (u16     ), // ValueCount
        values          (Vec<One>), // Value
        coverage        (Coverage),
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
            _ => raise!("found a pair-adjustment table in an unknown format"),
        })
    }
}

impl Value for PairAdjustment1 {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let format = try!(tape.take());
        let coverage_offset = try!(tape.take());
        let value1_flags = read_flags!(tape);
        let value2_flags = read_flags!(tape);
        let pair_set_count = try!(tape.take());
        let pair_set_offsets: Vec<u16> = try!(tape.take_given(pair_set_count as usize));
        try!(tape.jump(position + coverage_offset as u64));
        let coverage = try!(tape.take());
        let mut pair_sets = Vec::with_capacity(pair_set_count as usize);
        for i in 0..(pair_set_count as usize) {
            try!(tape.jump(position + pair_set_offsets[i] as u64));
            pair_sets.push(try!(tape.take_given((value1_flags, value2_flags))));
        }
        Ok(PairAdjustment1 {
            format: format,
            coverage_offset: coverage_offset,
            value1_flags: value1_flags,
            value2_flags: value2_flags,
            pair_set_count: pair_set_count,
            pair_set_offsets: pair_set_offsets,
            coverage: coverage,
            pair_sets: pair_sets,
        })
    }
}

impl Value for PairAdjustment2 {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let format = try!(tape.take());
        let coverage_offset = try!(tape.take());
        let value1_flags = try!(tape.take());
        let value2_flags = try!(tape.take());
        let class1_offset = try!(tape.take());
        let class2_offset = try!(tape.take());
        let class1_count = try!(tape.take());
        let class2_count = try!(tape.take());
        let mut pair_sets = Vec::with_capacity(class1_count as usize);
        for i in 0..(class1_count as usize) {
            let mut records = Vec::with_capacity(class2_count as usize);
            for j in 0..(class2_count as usize) {
                records.push(try!(tape.take_given((value1_flags, value2_flags))));
            }
            pair_sets.push(records);
        }
        try!(tape.jump(position + coverage_offset as u64));
        let coverage = try!(tape.take());
        try!(tape.jump(position + class1_offset as u64));
        let class1 = try!(tape.take());
        try!(tape.jump(position + class2_offset as u64));
        let class2 = try!(tape.take());
        Ok(PairAdjustment2 {
            format: format,
            coverage_offset: coverage_offset,
            value1_flags: value1_flags,
            value2_flags: value2_flags,
            class1_offset: class1_offset,
            class2_offset: class2_offset,
            class1_count: class1_count,
            class2_count: class2_count,
            pair_sets: pair_sets,
            coverage: coverage,
            class1: class1,
            class2: class2,
        })
    }
}

impl Value for SingleAdjustment {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => SingleAdjustment::Format1(try!(tape.take())),
            2 => SingleAdjustment::Format2(try!(tape.take())),
            _ => raise!("found a single-adjustment table in an unknown format"),
        })
    }
}

impl Value for SingleAdjustment1 {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let format = try!(tape.take());
        let coverage_offset = try!(tape.take());
        let value_flags = read_flags!(tape);
        let value = try!(tape.take_given(value_flags));
        try!(tape.jump(position + coverage_offset as u64));
        let coverage = try!(tape.take());
        Ok(SingleAdjustment1 {
            format: format,
            coverage_offset: coverage_offset,
            value_flags: value_flags,
            value: value,
            coverage: coverage,
        })
    }
}

impl Value for SingleAdjustment2 {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let format = try!(tape.take());
        let coverage_offset = try!(tape.take());
        let value_flags = read_flags!(tape);
        let value_count = try!(tape.take());
        let mut values = Vec::with_capacity(value_count as usize);
        for i in 0..(value_count as usize) {
            values.push(try!(tape.take_given(value_flags)));
        }
        try!(tape.jump(position + coverage_offset as u64));
        let coverage = try!(tape.take());
        Ok(SingleAdjustment2 {
            format: format,
            coverage_offset: coverage_offset,
            value_flags: value_flags,
            value_count: value_count,
            values: values,
            coverage: coverage,
        })
    }
}
