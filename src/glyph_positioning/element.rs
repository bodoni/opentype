#![allow(unused_mut, unused_variables)]

use {Result, Tape, Value, Walue};

/// An anchor.
#[derive(Clone, Debug)]
pub enum Anchor {
    /// Format 1.
    Format1(Anchor1),
    /// Format 2.
    Format2(Anchor2),
    /// Format 3.
    Format3(Anchor3),
}

table! {
    #[doc = "An anchor in format 1."]
    #[derive(Copy)]
    pub Anchor1 { // AnchorFormat1
    }
}

table! {
    #[doc = "An anchor in format 2."]
    #[derive(Copy)]
    pub Anchor2 { // AnchorFormat2
    }
}

table! {
    #[doc = "An anchor in format 3."]
    #[derive(Copy)]
    pub Anchor3 { // AnchorFormat3
    }
}

table! {
    #[doc = "An entry-exit record."]
    #[derive(Copy)]
    pub Connection { // EntryExitRecord
        entry_offset (u16), // EntryAnchor
        exit_offset  (u16), // ExitAnchor
    }
}

table! {
    @define
    #[doc = "A single value."]
    #[derive(Copy)]
    pub SingleValue { // ValueRecord
        x_placement               (Option<i16>), // XPlacement
        y_placement               (Option<i16>), // YPlacement
        x_advance                 (Option<i16>), // XAdvance
        y_advance                 (Option<i16>), // YAdvance
        device_x_placement_offset (Option<u16>), // XPlaDevice
        device_y_placement_offset (Option<u16>), // YPlaDevice
        device_x_advance_offset   (Option<u16>), // XAdvDevice
        device_y_advance_offset   (Option<u16>), // YAdvDevice
    }
}

table! {
    @define
    #[doc = "A value pair."]
    #[derive(Copy)]
    pub PairValue { // PairValueRecord
        value1 (SingleValue), // Value1
        value2 (SingleValue), // Value2
    }
}

table! {
    @define
    #[doc = "A set of value pairs."]
    pub PairValueSet { // PairSet
        count   (u16           ), // PairValueCount
        records (Vec<PairValue>), // PairValueRecord
    }
}

flags! {
    #[doc = "Value flags."]
    pub ValueFlags(u16) {
        0b0000_0000_0000_0001 => has_x_placement,
        0b0000_0000_0000_0010 => has_y_placement,
        0b0000_0000_0000_0100 => has_x_advance,
        0b0000_0000_0000_1000 => has_y_advance,
        0b0000_0000_0001_0000 => has_device_x_placement,
        0b0000_0000_0010_0000 => has_device_y_placement,
        0b0000_0000_0100_0000 => has_device_x_advance,
        0b0000_0000_1000_0000 => has_device_y_advance,
        0b1111_1111_0000_0000 => is_invalid,
    }
}

impl Value for Anchor {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => Anchor::Format1(try!(tape.take())),
            2 => Anchor::Format2(try!(tape.take())),
            3 => Anchor::Format3(try!(tape.take())),
            _ => raise!("found an unknown format of the anchor table"),
        })
    }
}

impl Walue<(ValueFlags, ValueFlags)> for PairValue {
    #[inline]
    fn read<T: Tape>(tape: &mut T, (flags1, flags2): (ValueFlags, ValueFlags)) -> Result<Self> {
        Ok(PairValue {
            value1: try!(tape.take_given(flags1)),
            value2: try!(tape.take_given(flags2)),
        })
    }
}

impl Walue<(ValueFlags, ValueFlags)> for PairValueSet {
    fn read<T: Tape>(tape: &mut T, flags: (ValueFlags, ValueFlags)) -> Result<Self> {
        let count = try!(tape.take());
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(try!(tape.take_given(flags)));
        }
        Ok(PairValueSet { count: count, records: records })
    }
}

impl Walue<ValueFlags> for SingleValue {
    fn read<T: Tape>(tape: &mut T, flags: ValueFlags) -> Result<Self> {
        macro_rules! read(
            ($flag:ident) => (if flags.$flag() { Some(try!(tape.take())) } else { None });
        );
        Ok(SingleValue {
            x_placement: read!(has_x_placement),
            y_placement: read!(has_y_placement),
            x_advance: read!(has_x_advance),
            y_advance: read!(has_y_advance),
            device_x_placement_offset: read!(has_device_x_placement),
            device_y_placement_offset: read!(has_device_y_placement),
            device_x_advance_offset: read!(has_device_x_advance),
            device_y_advance_offset: read!(has_device_y_advance),
        })
    }
}
