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
        nameRecord   (Vec<NameRecord>) |this| { Ok(this.count as usize) },
    }
}

spec! {
    pub NamingTable1 {
        format        (USHORT            ),
        count         (USHORT            ),
        stringOffset  (USHORT            ),
        nameRecord    (Vec<NameRecord>   ) |this| { Ok(this.count as usize) },
        langTagCount  (USHORT            ),
        langTagRecord (Vec<LangTagRecord>) |this| { Ok(this.langTagCount as usize) },
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
