//! The scripts.

use truetype::Tag;

use {Result, Tape, Value, Walue};

table! {
    @define
    #[doc = "A script list."]
    pub Scripts {
        count   (u16        ), // ScriptCount
        headers (Vec<Header>), // ScriptRecord
        records (Vec<Script>),
    }
}

table! {
    #[doc = "The header of a script-list record."]
    #[derive(Copy)]
    pub Header {
        tag    (Tag), // ScriptTag
        offset (u16), // Script
    }
}

table! {
    @define
    #[doc = "A script."]
    pub Script {
        default_language_offset (u16                ), // DefaultLangSys
        language_count          (u16                ), // LangSysCount
        language_headers        (Vec<LanguageHeader>), // LangSysRecord
        default_language        (Option<Language>   ),
        language_records        (Vec<Language>      ),
    }
}

table! {
    #[doc = "The header of a language system."]
    pub LanguageHeader {
        tag    (Tag), // LangSysTag
        offset (u16), // LangSys
    }
}

table! {
    #[doc = "A language system."]
    pub Language {
        lookup_order (u16) |tape, this| { // LookupOrder
            let value = try!(tape.take());
            if value != 0 {
                raise!("found an unsupported lookup order");
            }
            Ok(value)
        },
        required_feature_index (u16), // ReqFeatureIndex
        feature_count          (u16), // FeatureCount

        feature_indices (Vec<u16>) |tape, this| { // FeatureIndex
            Walue::read(tape, this.feature_count as usize)
        },
    }
}

impl Value for Scripts {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let count = try!(tape.take::<u16>());
        let headers: Vec<Header> = try!(tape.take_given(count as usize));
        let mut records: Vec<Script> = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            try!(tape.jump(position + headers[i].offset as u64));
            records.push(try!(tape.take()));
        }
        Ok(Scripts { count: count, headers: headers, records: records })
    }
}

impl Value for Script {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let default_language_offset = try!(tape.take::<u16>());
        let language_count = try!(tape.take::<u16>());
        let language_headers: Vec<LanguageHeader> = try!(tape.take_given(language_count as usize));
        let default_language = if default_language_offset != 0 {
            try!(tape.jump(position + default_language_offset as u64));
            Some(try!(tape.take()))
        } else {
            None
        };
        let mut language_records: Vec<Language> = Vec::with_capacity(language_count as usize);
        for i in 0..(language_count as usize) {
            try!(tape.jump(position + language_headers[i].offset as u64));
            language_records.push(try!(tape.take()));
        }
        Ok(Script {
            default_language_offset: default_language_offset,
            language_count: language_count,
            language_headers: language_headers,
            default_language: default_language,
            language_records: language_records,
        })
    }
}
