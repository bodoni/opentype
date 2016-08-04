use {Result, Tape, Walue};

flags! {
    #[doc = "Value flags."]
    pub Flags(u16) {
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

table! {
    @define
    #[doc = "A single value."]
    pub One { // ValueRecord
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
    #[doc = "A pair of values."]
    pub Pair { // PairValueRecord
        value1 (One), // Value1
        value2 (One), // Value2
    }
}

table! {
    @define
    #[doc = "A set of value pairs."]
    pub PairSet {
        count   (u16      ), // PairValueCount
        records (Vec<Pair>), // PairValueRecord
    }
}

impl Walue<Flags> for One {
    fn read<T: Tape>(tape: &mut T, flags: Flags) -> Result<Self> {
        macro_rules! read(
            ($flag:ident) => (if flags.$flag() { Some(try!(tape.take())) } else { None });
        );
        Ok(One {
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

impl Walue<(Flags, Flags)> for Pair {
    #[inline]
    fn read<T: Tape>(tape: &mut T, (flags1, flags2): (Flags, Flags)) -> Result<Self> {
        Ok(Pair { value1: try!(tape.take_given(flags1)), value2: try!(tape.take_given(flags2)) })
    }
}

impl Walue<(Flags, Flags)> for PairSet {
    fn read<T: Tape>(tape: &mut T, flags: (Flags, Flags)) -> Result<Self> {
        let count = try!(tape.take());
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(try!(tape.take_given(flags)));
        }
        Ok(PairSet { count: count, records: records })
    }
}
