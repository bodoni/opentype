use Result;
use primitive::*;
use table::HorizontalHeader;
use tape::{Tape, Value};
use truetype::compound::MaximumProfile;

declare! {
    pub HorizontalMetrics {
        hMetrics        (Vec<LongHorizontalMetric>),
        leftSideBearing (Vec<Short>               ),
    }
}

spec! {
    #[derive(Copy)]
    pub LongHorizontalMetric {
        advanceWidth (UShort),
        lsb          (Short ),
    }
}

impl HorizontalMetrics {
    #[doc(hidden)]
    pub fn read<T: Tape>(tape: &mut T, header: &HorizontalHeader, profile: &MaximumProfile)
                         -> Result<Self> {

        let metrics = header.numberOfHMetrics as usize;
        let glyphs = profile.glyphs();
        debug_assert!(metrics <= glyphs);
        let bearings = glyphs - metrics;

        let mut table = HorizontalMetrics {
            hMetrics: Vec::with_capacity(metrics),
            leftSideBearing: Vec::with_capacity(bearings),
        };
        for _ in 0..metrics {
            table.hMetrics.push(try!(Value::read(tape)));
        }
        for _ in 0..bearings {
            table.leftSideBearing.push(try!(Value::read(tape)));
        }

        Ok(table)
    }
}
