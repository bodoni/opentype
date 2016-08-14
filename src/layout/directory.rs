use truetype::q32;

use {Result, Tape, Value, Walue};
use super::{Features, Lookups, Scripts};

/// A directory of scripts, features, and lookups.
#[derive(Clone, Debug)]
pub struct Directory<T> {
    pub header: Header,
    pub scripts: Scripts,
    pub features: Features,
    pub lookups: Lookups<T>,
}

table! {
    #[doc = "The header of a directory table."]
    #[derive(Copy)]
    pub Header {
        version        (q32) = { q32(0x00010000) }, // Version
        script_offset  (u16), // ScriptList
        feature_offset (u16), // FeatureList
        lookup_offset  (u16), // LookupList
    }
}

impl<U> Value for Directory<U> where U: Walue<u16> {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let header = try!(tape.take::<Header>());
        let scripts = jump_take!(@unwrap tape, position, header.script_offset);
        let features = jump_take!(@unwrap tape, position, header.feature_offset);
        let lookups = jump_take!(@unwrap tape, position, header.lookup_offset);
        Ok(Directory { header: header, scripts: scripts, features: features, lookups: lookups })
    }
}
