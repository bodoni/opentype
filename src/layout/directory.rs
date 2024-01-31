use crate::layout::feature::Variations;
use crate::layout::{Features, Lookups, Scripts};
use crate::Result;

/// A layout directory.
#[derive(Clone, Debug)]
#[rustfmt::skip]
pub struct Directory<T> {
    pub major_version: u16, // majorVersion
    pub minor_version: u16, // minorVersion
    pub script_offset: u16, // scriptListOffset
    pub feature_offset: u16, // featureListOffset
    pub lookup_offset: u16, // lookupListOffset
    pub variation_offset: u32, // featureVariationsOffset

    pub scripts: Scripts,
    pub features: Features,
    pub lookups: Lookups<T>,
    pub variations: Option<Variations>,
}

impl<U> crate::value::Read for Directory<U>
where
    U: crate::walue::Read<'static, Parameter = u16>,
{
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        let position = tape.position()?;
        let major_version = tape.take()?;
        let minor_version = tape.take()?;
        match (major_version, minor_version) {
            (1, 0) | (1, 1) => {}
            value => raise!("found an unknown version of the directory table {value:?}"),
        }
        let script_offset = tape.take()?;
        let feature_offset = tape.take()?;
        let lookup_offset = tape.take()?;
        let variation_offset = match (major_version, minor_version) {
            (1, 1) => tape.take()?,
            _ => 0,
        };
        #[cfg(not(feature = "ignore-incomplete-directories"))]
        let (scripts, features, lookups) = (
            jump_take!(@unwrap tape, position, script_offset),
            jump_take!(@unwrap tape, position, feature_offset),
            jump_take!(@unwrap tape, position, lookup_offset),
        );
        #[cfg(feature = "ignore-incomplete-directories")]
        let (scripts, features, lookups) = (
            jump_take_maybe!(@unwrap tape, position, script_offset).unwrap_or_default(),
            jump_take_maybe!(@unwrap tape, position, feature_offset).unwrap_or_default(),
            jump_take_maybe!(@unwrap tape, position, lookup_offset).unwrap_or_default(),
        );
        let variations = jump_take_maybe!(@unwrap tape, position, variation_offset);
        Ok(Directory {
            major_version,
            minor_version,
            script_offset,
            feature_offset,
            lookup_offset,
            variation_offset,
            scripts,
            features,
            lookups,
            variations,
        })
    }
}
