#![allow(unused_parens)]

use truetype::{Tape, Value, Walue};

use crate::layout::feature::Variations;
use crate::layout::{Features, Lookups, Scripts};
use crate::Result;

/// A layout directory.
#[derive(Clone, Debug)]
#[rustfmt::skip]
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

impl<U> Value for Directory<U>
where
    U: Walue<'static, Parameter = u16>,
{
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = tape.position()?;
        let major_version = tape.take()?;
        let minor_version = tape.take()?;
        match (major_version, minor_version) {
            (1, 0) | (1, 1) => {}
            _ => raise!("found an unknown version of the directory table"),
        }
        let scripts_offset = tape.take()?;
        let features_offset = tape.take()?;
        let lookups_offset = tape.take()?;
        let variations_offset = match (major_version, minor_version) {
            (1, 1) => tape.take()?,
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
