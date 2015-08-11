use Result;
use band::{Band, Value};
use primitive::*;
use table::HorizontalHeader;
use table::MaximumProfile;

declare! {
    pub HorizontalMetrics {
        hMetrics        (Vec<longHorMetric>),
        leftSideBearing (Vec<SHORT>        ),
    }
}

spec! {
    #[derive(Copy)]
    pub longHorMetric {
        advanceWidth (USHORT),
        lsb          (SHORT ),
    }
}

impl HorizontalMetrics {
    #[doc(hidden)]
    pub fn read<T: Band>(band: &mut T, header: &HorizontalHeader, profile: &MaximumProfile)
                         -> Result<Self> {

        let metrics = header.numberOfHMetrics as usize;
        let glyphs = profile.glyphs();
        debug_assert!(metrics <= glyphs);
        let bearings = glyphs - metrics;

        let mut value = HorizontalMetrics {
            hMetrics: Vec::with_capacity(metrics),
            leftSideBearing: Vec::with_capacity(bearings),
        };
        for _ in 0..metrics {
            value.hMetrics.push(try!(Value::read(band)));
        }
        for _ in 0..bearings {
            value.leftSideBearing.push(try!(Value::read(band)));
        }

        Ok(value)
    }
}
