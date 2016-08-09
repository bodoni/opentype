use {Result, Tape, Value, Walue, q32};
use super::{Features, Lookups, Scripts};

/// A directory of scripts, features, and lookups.
#[derive(Clone, Debug, Eq, PartialEq)]
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
        version (q32) |_, tape| { // Version
            let value = try!(tape.take());
            if value != q32(0x00010000) {
                raise!("found an unknown format of the directory table");
            }
            Ok(value)
        },

        script_offset  (u16), // ScriptList
        feature_offset (u16), // FeatureList
        lookup_offset  (u16), // LookupList
    }
}

impl<U> Value for Directory<U> where U: Walue<u16> {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let header = try!(tape.take::<Header>());
        try!(tape.jump(position + header.script_offset as u64));
        let scripts = try!(tape.take());
        try!(tape.jump(position + header.feature_offset as u64));
        let features = try!(tape.take());
        try!(tape.jump(position + header.lookup_offset as u64));
        let lookups = try!(tape.take());
        Ok(Directory { header: header, scripts: scripts, features: features, lookups: lookups })
    }
}
