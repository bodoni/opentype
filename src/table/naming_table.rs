use primitive::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NamingTable {
    Format0(NamingTable0),
    Format1(NamingTable1),
}

spec! {
    pub NamingTable0 {
        format       (USHORT         ),
        count        (USHORT         ),
        stringOffset (USHORT         ),
        nameRecord   (Vec<NameRecord>) |band, this| { read_vector!(band, this.count) },
    }
}

spec! {
    pub NamingTable1 {
        format        (USHORT            ),
        count         (USHORT            ),
        stringOffset  (USHORT            ),
        nameRecord    (Vec<NameRecord>   ) |band, this| { read_vector!(band, this.count) },
        langTagCount  (USHORT            ),
        langTagRecord (Vec<LangTagRecord>) |band, this| { read_vector!(band, this.langTagCount) },
    }
}

spec! {
    #[derive(Copy)]
    pub NameRecord {
        platformID (USHORT),
        encodingID (USHORT),
        languageID (USHORT),
        nameID     (USHORT),
        length     (USHORT),
        offset     (USHORT),
    }
}

spec! {
    #[derive(Copy)]
    pub LangTagRecord {
        length (USHORT),
        ffset  (USHORT),
    }
}
