use truetype::{Result, Tape, Value, Walue};

use layout::{Features, Lookups, Scripts};
use layout::feature::Variations;

/// A layout directory.
#[derive(Clone, Debug)]
pub struct Directory<T> {
    pub major_version:     (u16), // MajorVersion
    pub minor_version:     (u16), // MinorVersion
    pub scripts_offset:    (u16), // ScriptList
    pub features_offset:   (u16), // FeatureList
    pub lookups_offset:    (u16), // LookupList
    pub variations_offset: (u32), // FeatureVariations

    pub scripts:    (Scripts           ),
    pub features:   (Features          ),
    pub lookups:    (Lookups<T>        ),
    pub variations: (Option<Variations>),
}

impl<U> Value for Directory<U> where U: Walue<'static, Parameter=u16> {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let major_version = try!(tape.take());
        let minor_version = try!(tape.take());
        match (major_version, minor_version) {
            (1, 0) => {},
            _ => raise!("found an unknown version of the directory table"),
        }
        let scripts_offset = try!(tape.take());
        let features_offset = try!(tape.take());
        let lookups_offset = try!(tape.take());
        let variations_offset = match (major_version, minor_version) {
            (1, 1) => try!(tape.take()),
            _ => 0,
        };
        let scripts = jump_take!(@unwrap tape, position, scripts_offset);
        let features = jump_take!(@unwrap tape, position, features_offset);
        let lookups = jump_take!(@unwrap tape, position, lookups_offset);
        let variations = jump_take_maybe!(@unwrap tape, position, variations_offset);
        Ok(Directory {
            major_version: major_version,
            minor_version: minor_version,
            scripts_offset: scripts_offset,
            features_offset: features_offset,
            lookups_offset: lookups_offset,
            variations_offset: variations_offset,
            scripts: scripts,
            features: features,
            lookups: lookups,
            variations: variations,
        })
    }
}
