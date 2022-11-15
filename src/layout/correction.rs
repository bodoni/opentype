use truetype::{Tape, Value};

use crate::Result;

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
        start_size (u16     ), // StartSize
        end_size   (u16     ), // EndSize
        format     (u16     ), // DeltaFormat
        deltas     (Vec<u16>), // DeltaValue
    }
}

table! {
    #[doc = "A variation correction."]
    pub Variation { // VariationIndex
        outer_index (u16), // DeltaSetOuterIndex
        inner_index (u16), // DeltaSetInnerIndex
        format      (u16), // DeltaFormat
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
        Ok(match tape.peek::<Header>()?.format {
            1 | 2 | 3 => Correction::Device(tape.take()?),
            0x8000 => Correction::Variation(tape.take()?),
            _ => raise!("found an unknown format of the correction table"),
        })
    }
}

impl Default for Correction {
    #[inline]
    fn default() -> Self {
        Correction::Device(Device::default())
    }
}

impl Value for Device {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let start_size = tape.take()?;
        let end_size = tape.take()?;
        if start_size > end_size {
            raise!("found a malformed device table");
        }
        let format = tape.take()?;
        if format == 0 || format > 3 {
            raise!("found an unknown format of the device table");
        }
        let count = (end_size - start_size) as usize + 1;
        let bit_count = (1 << format as usize) * count;
        let short_count = (bit_count + 16 - bit_count % 16) >> 4;
        let deltas = tape.take_given(short_count)?;
        Ok(Device {
            start_size: start_size,
            end_size: end_size,
            format: format,
            deltas: deltas,
        })
    }
}
