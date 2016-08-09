//! The feature list.

use truetype::Tag;

table! {
    @position
    #[doc = "A feature list."]
    pub Features {
        count (u16), // FeatureCount

        headers (Vec<Header>) |this, tape, __| { // FeatureRecord
            tape.take_given(this.count as usize)
        },

        records (Vec<Record>) |this, tape, position| {
            let mut values = Vec::with_capacity(this.count as usize);
            for i in 0..(this.count as usize) {
                try!(tape.jump(position + this.headers[i].offset as u64));
                values.push(try!(tape.take()));
            }
            Ok(values)
        },
    }
}

table! {
    #[doc = "A feature header."]
    #[derive(Copy)]
    pub Header {
        tag    (Tag), // FeatureTag
        offset (u16), // Feature
    }
}

table! {
    @position
    #[doc = "A feature record."]
    pub Record {
        parameter_offset (u16), // FeatureParams
        lookup_count     (u16), // LookupCount

        lookup_indices (Vec<u16>) |this, tape, __| { // LookupListIndex
            tape.take_given(this.lookup_count as usize)
        },

        parameters (Option<Vec<u8>>) |this, tape, position| {
            if this.parameter_offset != 0 {
                try!(tape.jump(position + this.parameter_offset as u64));
                Ok(Some(try!(tape.take_bytes(0))))
            } else {
                Ok(None)
            }
        },
    }
}
