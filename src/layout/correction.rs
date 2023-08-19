//! The adjustment correction.

use crate::{Result, Tape, Value};

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
        start_size (u16     ), // startSize
        end_size   (u16     ), // endSize
        format     (u16     ), // deltaFormat
        deltas     (Vec<u16>), // deltaValue
    }
}

table! {
    #[doc = "A variation correction."]
    pub Variation { // VariationIndex
        outer_index (u16), // deltaSetOuterIndex
        inner_index (u16), // deltaSetInnerIndex
        format      (u16), // deltaFormat
    }
}

impl Value for Correction {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<(u32, u16)>()?.1 {
            1 | 2 | 3 => Correction::Device(tape.take()?),
            0x8000 => Correction::Variation(tape.take()?),
            value => raise!("found an unknown format of the adjustment correction ({value})"),
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
            raise!("found a malformed device correction");
        }
        let format = tape.take()?;
        if format == 0 || format > 3 {
            raise!("found an unknown format of the device correction ({format})");
        }
        let count = (end_size - start_size) as usize + 1;
        let bit_count = (1 << format as usize) * count;
        let short_count = (bit_count + 16 - bit_count % 16) >> 4;
        let deltas = tape.take_given(short_count)?;
        Ok(Device {
            start_size,
            end_size,
            format,
            deltas,
        })
    }
}
