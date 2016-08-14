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
        format (u16) = { 1 }, // AnchorFormat
        x      (i16), // XCoordinate
        y      (i16), // YCoordinate
    }
}

table! {
    #[doc = "An anchor in format 2."]
    #[derive(Copy)]
    pub Anchor2 { // AnchorFormat2
        format (u16) = { 2 }, // AnchorFormat
        x      (i16), // XCoordinate
        y      (i16), // YCoordinate
        index  (u16), // AnchorPoint
    }
}

table! {
    @position
    #[doc = "An anchor in format 3."]
    pub Anchor3 { // AnchorFormat3
        format          (u16) = { 3 }, // AnchorFormat
        x               (i16), // XCoordinate
        y               (i16), // YCoordinate
        device_x_offset (u16), // XDeviceTable
        device_y_offset (u16), // YDeviceTable

        device_x (Option<Device>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.device_x_offset)
        },

        device_y (Option<Device>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.device_y_offset)
        },
    }
}

table! {
    @define
    #[doc = "A device table."]
    pub Device { // Device
        start_size   (u16     ), // StartSize
        end_size     (u16     ), // EndSize
        delta_format (u16     ), // DeltaFormat
        delta_data   (Vec<u16>), // DeltaValue
    }
}

table! {
    @define
    #[doc = "An entry-exit record."]
    pub Gesture { // EntryExitRecord
        entry_offset (u16   ), // EntryAnchor
        exit_offset  (u16   ), // ExitAnchor
        entry        (Anchor),
        exit         (Anchor),
    }
}

table! {
    @define
    #[doc = "A single value."]
    pub Single { // ValueRecord
        x_placement               (Option<i16>   ), // XPlacement
        y_placement               (Option<i16>   ), // YPlacement
        x_advance                 (Option<i16>   ), // XAdvance
        y_advance                 (Option<i16>   ), // YAdvance
        device_x_placement_offset (Option<u16>   ), // XPlaDevice
        device_y_placement_offset (Option<u16>   ), // YPlaDevice
        device_x_advance_offset   (Option<u16>   ), // XAdvDevice
        device_y_advance_offset   (Option<u16>   ), // YAdvDevice
        device_x_placement        (Option<Device>),
        device_y_placement        (Option<Device>),
        device_x_advance          (Option<Device>),
        device_y_advance          (Option<Device>),
    }
}

table! {
    @define
    #[doc = "A value pair."]
    pub Pair { // PairValueRecord
        value1 (Single), // Value1
        value2 (Single), // Value2
    }
}

table! {
    @define
    #[doc = "A set of value pairs."]
    pub PairSet { // PairSet
        count   (u16      ), // PairValueCount
        records (Vec<Pair>), // PairValueRecord
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

impl Value for Device {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let start_size = try!(tape.take());
        let end_size = try!(tape.take());
        if start_size > end_size {
            raise!("found a malformed device table");
        }
        let delta_format = try!(tape.take());
        if delta_format == 0 || delta_format > 3 {
            raise!("found an unknow format of the device table");
        }
        let count = (end_size - start_size) as usize + 1;
        let bit_count = (1 << delta_format as usize) * count;
        let short_count = (bit_count + 16 - bit_count % 16) >> 4;
        let delta_data = try!(tape.take_given(short_count));
        Ok(Device {
            start_size: start_size,
            end_size: end_size,
            delta_format: delta_format,
            delta_data: delta_data,
        })
    }
}

impl Walue<u64> for Gesture {
    fn read<T: Tape>(tape: &mut T, position: u64) -> Result<Self> {
        let entry_offset = try!(tape.take());
        let exit_offset = try!(tape.take());
        let entry = jump_take!(@unwrap tape, position, entry_offset);
        let exit = jump_take!(@unwrap tape, position, exit_offset);
        Ok(Gesture {
            entry_offset: entry_offset,
            exit_offset: exit_offset,
            entry: entry,
            exit: exit,
        })
    }
}

impl Walue<(u64, ValueFlags, ValueFlags)> for Pair {
    #[inline]
    fn read<T: Tape>(tape: &mut T, (position, flags1, flags2): (u64, ValueFlags, ValueFlags))
                     -> Result<Self> {

        Ok(Pair {
            value1: try!(tape.take_given((position, flags1))),
            value2: try!(tape.take_given((position, flags2))),
        })
    }
}

impl Walue<(u64, ValueFlags, ValueFlags)> for PairSet {
    fn read<T: Tape>(tape: &mut T, parameters: (u64, ValueFlags, ValueFlags)) -> Result<Self> {
        let count = try!(tape.take());
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            records.push(try!(tape.take_given(parameters)));
        }
        Ok(PairSet { count: count, records: records })
    }
}

impl Walue<(u64, ValueFlags)> for Single {
    fn read<T: Tape>(tape: &mut T, (position, flags): (u64, ValueFlags)) -> Result<Self> {
        macro_rules! read(
            ($flag:ident) => (if flags.$flag() { Some(try!(tape.take())) } else { None });
        );
        let x_placement = read!(has_x_placement);
        let y_placement = read!(has_y_placement);
        let x_advance = read!(has_x_advance);
        let y_advance = read!(has_y_advance);
        let device_x_placement_offset = read!(has_device_x_placement);
        let device_y_placement_offset = read!(has_device_y_placement);
        let device_x_advance_offset = read!(has_device_x_advance);
        let device_y_advance_offset = read!(has_device_y_advance);
        macro_rules! read(
            ($offset:ident) => (
                match $offset {
                    Some(offset) => Some(jump_take!(@unwrap tape, position, offset)),
                    _ => None,
                }
            );
        );
        let device_x_placement = read!(device_x_placement_offset);
        let device_y_placement = read!(device_y_placement_offset);
        let device_x_advance = read!(device_x_advance_offset);
        let device_y_advance = read!(device_y_advance_offset);
        Ok(Single {
            x_placement: x_placement,
            y_placement: y_placement,
            x_advance: x_advance,
            y_advance: y_advance,
            device_x_placement_offset: device_x_placement_offset,
            device_y_placement_offset: device_y_placement_offset,
            device_x_advance_offset: device_x_advance_offset,
            device_y_advance_offset: device_y_advance_offset,
            device_x_placement: device_x_placement,
            device_y_placement: device_y_placement,
            device_x_advance: device_x_advance,
            device_y_advance: device_y_advance,
        })
    }
}
