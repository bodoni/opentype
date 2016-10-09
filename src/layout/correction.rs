use truetype::{Result, Tape, Value};

/// A correction.
#[derive(Clone, Debug)]
pub enum Correction {
    /// A device correction.
    Device(Device),
    /// A variation correction.
    Variation(Variation),
}

table! {
    @define
    #[doc = "A device correction."]
    pub Device { // Device
        start_size   (u16     ), // StartSize
        end_size     (u16     ), // EndSize
        delta_format (u16     ), // DeltaFormat
        delta_data   (Vec<u16>), // DeltaValue
    }
}

table! {
    #[doc = "A variation correction."]
    pub Variation { // VariationIndex
        outer_index  (u16), // DeltaSetOuterIndex
        inner_index  (u16), // DeltaSetInnerIndex
        delta_format (u16), // DeltaFormat
    }
}

table! {
    pub Header {
        _dummy (u32),
        format (u16),
    }
}

impl Value for Correction {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<Header>()).format {
            1 | 2 | 3 => Correction::Device(try!(tape.take())),
            0x8000 => Correction::Variation(try!(tape.take())),
            _ => raise!("found an unknown format of the correction table"),
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
            raise!("found an unknown format of the device table");
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
