use truetype::{Result, Tape, Value};

table! {
    @define
    #[doc = "A device adjustment."]
    pub Device { // Device
        start_size   (u16     ), // StartSize
        end_size     (u16     ), // EndSize
        delta_format (u16     ), // DeltaFormat
        delta_data   (Vec<u16>), // DeltaValue
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
