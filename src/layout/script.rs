//! The script list.

use truetype::Tag;

use {Result, Tape, Value, Walue};

table! {
    @define
    #[doc = "A script list."]
    pub Scripts {
        count   (u16        ), // ScriptCount
        headers (Vec<Header>), // ScriptRecord
        records (Vec<Record>),
    }
}

table! {
    #[doc = "A script header."]
    #[derive(Copy)]
    pub Header {
        tag    (Tag), // ScriptTag
        offset (u16), // Script
    }
}

table! {
    @define
    #[doc = "A script record."]
    pub Record {
        default_language_offset (u16                ), // DefaultLangSys
        language_count          (u16                ), // LangSysCount
        language_headers        (Vec<LanguageHeader>), // LangSysRecord
        default_language        (Option<Language>   ),
        language_records        (Vec<Language>      ),
    }
}

table! {
    #[doc = "A language-system header."]
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
                raise!("found an unknown lookup order");
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
        let mut records: Vec<Record> = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            try!(tape.jump(position + headers[i].offset as u64));
            records.push(try!(tape.take()));
        }
        Ok(Scripts { count: count, headers: headers, records: records })
    }
}

impl Value for Record {
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
        Ok(Record {
            default_language_offset: default_language_offset,
            language_count: language_count,
            language_headers: language_headers,
            default_language: default_language,
            language_records: language_records,
        })
    }
}

macro_rules! implement {
    ($($tag:expr => $name:expr => $token:ident,)*) => (
        /// A script.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Script {
            $(#[doc = $name] $token,)*
        }

        impl Script {
            fn tag(&self) -> &'static str {
                use self::Script::*;
                match *self {
                    $($token => $tag,)*
                }
            }
        }

        impl Scripts {
            /// Return the record of a script if present.
            pub fn get(&self, script: Script) -> Option<&Record> {
                let tag = script.tag().as_bytes();
                for (i, header) in self.headers.iter().enumerate() {
                    if header.tag.0 == tag {
                        return Some(&self.records[i]);
                    }
                }
                None
            }
        }
    );
}

implement! {
    "adlm" => "Adlam" => Adlam,
    "ahom" => "Ahom" => Ahom,
    "hluw" => "Anatolian Hieroglyphs" => AnatolianHieroglyphs,
    "arab" => "Arabic" => Arabic,
    "armn" => "Armenian" => Armenian,
    "avst" => "Avestan" => Avestan,
    "bali" => "Balinese" => Balinese,
    "bamu" => "Bamum" => Bamum,
    "bass" => "Bassa Vah" => BassaVah,
    "batk" => "Batak" => Batak,
    "beng" => "Bengali" => Bengali,
    "bng2" => "Bengali v.2" => BengaliV2,
    "bhks" => "Bhaiksuki" => Bhaiksuki,
    "bopo" => "Bopomofo" => Bopomofo,
    "brah" => "Brahmi" => Brahmi,
    "brai" => "Braille" => Braille,
    "bugi" => "Buginese" => Buginese,
    "buhd" => "Buhid" => Buhid,
    "byzm" => "Byzantine Music" => ByzantineMusic,
    "cans" => "Canadian Syllabics" => CanadianSyllabics,
    "cari" => "Carian" => Carian,
    "aghb" => "Caucasian Albanian" => CaucasianAlbanian,
    "cakm" => "Chakma" => Chakma,
    "cham" => "Cham" => Cham,
    "cher" => "Cherokee" => Cherokee,
    "hani" => "CJK Ideographic" => CJKIdeographic,
    "copt" => "Coptic" => Coptic,
    "cprt" => "Cypriot Syllabary" => CypriotSyllabary,
    "cyrl" => "Cyrillic" => Cyrillic,
    "DFLT" => "Default" => Default,
    "dsrt" => "Deseret" => Deseret,
    "deva" => "Devanagari" => Devanagari,
    "dev2" => "Devanagari v.2" => DevanagariV2,
    "dupl" => "Duployan" => Duployan,
    "egyp" => "Egyptian Hieroglyphs" => EgyptianHieroglyphs,
    "elba" => "Elbasan" => Elbasan,
    "ethi" => "Ethiopic" => Ethiopic,
    "geor" => "Georgian" => Georgian,
    "glag" => "Glagolitic" => Glagolitic,
    "goth" => "Gothic" => Gothic,
    "gran" => "Grantha" => Grantha,
    "grek" => "Greek" => Greek,
    "gujr" => "Gujarati" => Gujarati,
    "gjr2" => "Gujarati v.2" => GujaratiV2,
    "guru" => "Gurmukhi" => Gurmukhi,
    "gur2" => "Gurmukhi v.2" => GurmukhiV2,
    "hang" => "Hangul" => Hangul,
    "jamo" => "Hangul Jamo" => HangulJamo,
    "hano" => "Hanunoo" => Hanunoo,
    "hatr" => "Hatran" => Hatran,
    "hebr" => "Hebrew" => Hebrew,
    "kana" => "Hiragana" => Hiragana,
    "armi" => "Imperial Aramaic" => ImperialAramaic,
    "phli" => "Inscriptional Pahlavi" => InscriptionalPahlavi,
    "prti" => "Inscriptional Parthian" => InscriptionalParthian,
    "java" => "Javanese" => Javanese,
    "kthi" => "Kaithi" => Kaithi,
    "knda" => "Kannada" => Kannada,
    "knd2" => "Kannada v.2" => KannadaV2,
    "kana" => "Katakana" => Katakana,
    "kali" => "Kayah Li" => KayahLi,
    "khar" => "Kharosthi" => Kharosthi,
    "khmr" => "Khmer" => Khmer,
    "khoj" => "Khojki" => Khojki,
    "sind" => "Khudawadi" => Khudawadi,
    "lao"  => "Lao" => Lao,
    "latn" => "Latin" => Latin,
    "lepc" => "Lepcha" => Lepcha,
    "limb" => "Limbu" => Limbu,
    "lina" => "Linear A" => LinearA,
    "linb" => "Linear B" => LinearB,
    "lisu" => "Lisu (Fraser)" => Lisu,
    "lyci" => "Lycian" => Lycian,
    "lydi" => "Lydian" => Lydian,
    "mahj" => "Mahajani" => Mahajani,
    "mlym" => "Malayalam" => Malayalam,
    "mlm2" => "Malayalam v.2" => MalayalamV2,
    "mand" => "Mandaic, Mandaean" => Mandaic,
    "mani" => "Manichaean" => Manichaean,
    "marc" => "Marchen" => Marchen,
    "math" => "Mathematical Alphanumeric Symbols" => MathematicalAlphanumericSymbols,
    "mtei" => "Meitei Mayek (Meithei, Meetei)" => MeiteiMayek,
    "mend" => "Mende Kikakui" => MendeKikakui,
    "merc" => "Meroitic Cursive" => MeroiticCursive,
    "mero" => "Meroitic Hieroglyphs" => MeroiticHieroglyphs,
    "plrd" => "Miao" => Miao,
    "modi" => "Modi" => Modi,
    "mong" => "Mongolian" => Mongolian,
    "mroo" => "Mro" => Mro,
    "mult" => "Multani" => Multani,
    "musc" => "Musical Symbols" => MusicalSymbols,
    "mymr" => "Myanmar" => Myanmar,
    "mym2" => "Myanmar v.2" => MyanmarV2,
    "nbat" => "Nabataean" => Nabataean,
    "newa" => "Newa" => Newa,
    "talu" => "New Tai Lue" => NewTaiLue,
    "nko"  => "N'Ko" => NKo,
    "orya" => "Odia (formerly Oriya)" => Odia,
    "ory2" => "Odia v.2 (formerly Oriya v.2)" => OdiaV2,
    "ogam" => "Ogham" => Ogham,
    "olck" => "Ol Chiki" => OlChiki,
    "ital" => "Old Italic" => OldItalic,
    "hung" => "Old Hungarian" => OldHungarian,
    "narb" => "Old North Arabian" => OldNorthArabian,
    "perm" => "Old Permic" => OldPermic,
    "xpeo" => "Old Persian Cuneiform" => OldPersianCuneiform,
    "sarb" => "Old South Arabian" => OldSouthArabian,
    "orkh" => "Old Turkic, Orkhon Runic" => OldTurkic,
    "osge" => "Osage" => Osage,
    "osma" => "Osmanya" => Osmanya,
    "hmng" => "Pahawh Hmong" => PahawhHmong,
    "palm" => "Palmyrene" => Palmyrene,
    "pauc" => "Pau Cin Hau" => PauCinHau,
    "phag" => "Phags-pa" => PhagsPa,
    "phnx" => "Phoenician" => Phoenician,
    "phlp" => "Psalter Pahlavi" => PsalterPahlavi,
    "rjng" => "Rejang" => Rejang,
    "runr" => "Runic" => Runic,
    "samr" => "Samaritan" => Samaritan,
    "saur" => "Saurashtra" => Saurashtra,
    "shrd" => "Sharada" => Sharada,
    "shaw" => "Shavian" => Shavian,
    "sidd" => "Siddham" => Siddham,
    "sgnw" => "Sign Writing" => SignWriting,
    "sinh" => "Sinhala" => Sinhala,
    "sora" => "Sora Sompeng" => SoraSompeng,
    "xsux" => "Sumero-Akkadian Cuneiform" => SumeroAkkadianCuneiform,
    "sund" => "Sundanese" => Sundanese,
    "sylo" => "Syloti Nagri" => SylotiNagri,
    "syrc" => "Syriac" => Syriac,
    "tglg" => "Tagalog" => Tagalog,
    "tagb" => "Tagbanwa" => Tagbanwa,
    "tale" => "Tai Le" => TaiLe,
    "lana" => "Tai Tham (Lanna)" => TaiTham,
    "tavt" => "Tai Viet" => TaiViet,
    "takr" => "Takri" => Takri,
    "taml" => "Tamil" => Tamil,
    "tml2" => "Tamil v.2" => TamilV2,
    "tang" => "Tangut" => Tangut,
    "telu" => "Telugu" => Telugu,
    "tel2" => "Telugu v.2" => TeluguV2,
    "thaa" => "Thaana" => Thaana,
    "thai" => "Thai" => Thai,
    "tibt" => "Tibetan" => Tibetan,
    "tfng" => "Tifinagh" => Tifinagh,
    "tirh" => "Tirhuta" => Tirhuta,
    "ugar" => "Ugaritic Cuneiform" => UgariticCuneiform,
    "vai"  => "Vai" => Vai,
    "wara" => "Warang Citi" => WarangCiti,
    "yi"   => "Yi" => Yi,
}
