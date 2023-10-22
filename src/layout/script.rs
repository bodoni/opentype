//! The script list.

use truetype::Tag;

use crate::layout::language;

table! {
    @position
    #[doc = "A script list."]
    pub Scripts { // ScriptList
        count (u16), // scriptCount

        headers (Vec<Header>) |this, tape, _| { // scriptRecords
            tape.take_given(this.count as usize)
        },

        records (Vec<Record>) |this, tape, position| {
            jump_take!(tape, position, this.count, i => this.headers[i].offset)
        },
    }
}

table! {
    #[doc = "A script header."]
    #[derive(Copy)]
    pub Header { // ScriptRecord
        tag    (Tag), // scriptTag
        offset (u16), // scriptOffset
    }
}

table! {
    @position
    #[doc = "A script record."]
    pub Record { // Script
        default_language_offset (u16), // defaultLangSysOffset
        language_count          (u16), // langSysCount

        language_headers (Vec<language::Header>) |this, tape, _| { // langSysRecords
            tape.take_given(this.language_count as usize)
        },

        default_language (Option<language::Record>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.default_language_offset)
        },

        language_records (Vec<language::Record>) |this, tape, position| {
            jump_take!(tape, position, this.language_count, i => this.language_headers[i].offset)
        },
    }
}

impl Scripts {
    /// Return the record of a script if present.
    pub fn get<T: Into<Tag>>(&self, tag: T) -> Option<&Record> {
        let tag = tag.into();
        self.headers
            .iter()
            .enumerate()
            .find(|(_, header)| header.tag == tag)
            .map(|(i, _)| &self.records[i])
    }
}

impl Record {
    /// Return the record of a language if present.
    pub fn get<T: Into<Tag>>(&self, tag: T) -> Option<&language::Record> {
        let tag = tag.into();
        self.language_headers
            .iter()
            .enumerate()
            .find(|(_, header)| header.tag == tag)
            .map(|(i, _)| &self.language_records[i])
    }
}

macro_rules! implement {
    ($($tag:literal => $name:literal => $variant:ident,)*) => (
        /// A script.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Script {
            $(#[doc = $name] $variant,)*
        }

        impl Script {
            /// Create an instance from a tag.
            #[allow(unreachable_patterns)]
            pub fn from_tag(tag: &Tag) -> Option<Self> {
                match &**tag {
                    $($tag => Some(Self::$variant),)*
                    _ => None,
                }
            }

            /// Return the name.
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }
        }

        impl From<Script> for Tag {
            fn from(script: Script) -> Self {
                match script {
                    $(Script::$variant => Tag(*$tag),)*
                }
            }
        }
    );
}

implement! {
    b"adlm" => "Adlam" => Adlam,
    b"ahom" => "Ahom" => Ahom,
    b"hluw" => "Anatolian Hieroglyphs" => AnatolianHieroglyphs,
    b"arab" => "Arabic" => Arabic,
    b"armn" => "Armenian" => Armenian,
    b"avst" => "Avestan" => Avestan,
    b"bali" => "Balinese" => Balinese,
    b"bamu" => "Bamum" => Bamum,
    b"bass" => "Bassa Vah" => BassaVah,
    b"batk" => "Batak" => Batak,
    b"beng" => "Bengali" => Bengali,
    b"bng2" => "Bengali" => BengaliV2,
    b"bhks" => "Bhaiksuki" => Bhaiksuki,
    b"bopo" => "Bopomofo" => Bopomofo,
    b"brah" => "Brahmi" => Brahmi,
    b"brai" => "Braille" => Braille,
    b"bugi" => "Buginese" => Buginese,
    b"buhd" => "Buhid" => Buhid,
    b"byzm" => "Byzantine Music" => ByzantineMusic,
    b"cans" => "Canadian Syllabics" => CanadianSyllabics,
    b"cari" => "Carian" => Carian,
    b"aghb" => "Caucasian Albanian" => CaucasianAlbanian,
    b"cakm" => "Chakma" => Chakma,
    b"cham" => "Cham" => Cham,
    b"cher" => "Cherokee" => Cherokee,
    b"hani" => "CJK Ideographic" => CJKIdeographic,
    b"copt" => "Coptic" => Coptic,
    b"cprt" => "Cypriot Syllabary" => CypriotSyllabary,
    b"cyrl" => "Cyrillic" => Cyrillic,
    b"DFLT" => "Default" => Default,
    b"dsrt" => "Deseret" => Deseret,
    b"deva" => "Devanagari" => Devanagari,
    b"dev2" => "Devanagari" => DevanagariV2,
    b"dupl" => "Duployan" => Duployan,
    b"egyp" => "Egyptian Hieroglyphs" => EgyptianHieroglyphs,
    b"elba" => "Elbasan" => Elbasan,
    b"ethi" => "Ethiopic" => Ethiopic,
    b"geor" => "Georgian" => Georgian,
    b"glag" => "Glagolitic" => Glagolitic,
    b"goth" => "Gothic" => Gothic,
    b"gran" => "Grantha" => Grantha,
    b"grek" => "Greek" => Greek,
    b"gujr" => "Gujarati" => Gujarati,
    b"gjr2" => "Gujarati" => GujaratiV2,
    b"guru" => "Gurmukhi" => Gurmukhi,
    b"gur2" => "Gurmukhi" => GurmukhiV2,
    b"hang" => "Hangul" => Hangul,
    b"jamo" => "Hangul Jamo" => HangulJamo,
    b"hano" => "Hanunoo" => Hanunoo,
    b"hatr" => "Hatran" => Hatran,
    b"hebr" => "Hebrew" => Hebrew,
    b"kana" => "Hiragana" => Hiragana,
    b"armi" => "Imperial Aramaic" => ImperialAramaic,
    b"phli" => "Inscriptional Pahlavi" => InscriptionalPahlavi,
    b"prti" => "Inscriptional Parthian" => InscriptionalParthian,
    b"java" => "Javanese" => Javanese,
    b"kthi" => "Kaithi" => Kaithi,
    b"knda" => "Kannada" => Kannada,
    b"knd2" => "Kannada" => KannadaV2,
    b"kana" => "Katakana" => Katakana,
    b"kali" => "Kayah Li" => KayahLi,
    b"khar" => "Kharosthi" => Kharosthi,
    b"khmr" => "Khmer" => Khmer,
    b"khoj" => "Khojki" => Khojki,
    b"sind" => "Khudawadi" => Khudawadi,
    b"lao " => "Lao" => Lao,
    b"latn" => "Latin" => Latin,
    b"lepc" => "Lepcha" => Lepcha,
    b"limb" => "Limbu" => Limbu,
    b"lina" => "Linear A" => LinearA,
    b"linb" => "Linear B" => LinearB,
    b"lisu" => "Lisu (Fraser)" => Lisu,
    b"lyci" => "Lycian" => Lycian,
    b"lydi" => "Lydian" => Lydian,
    b"mahj" => "Mahajani" => Mahajani,
    b"mlym" => "Malayalam" => Malayalam,
    b"mlm2" => "Malayalam" => MalayalamV2,
    b"mand" => "Mandaic, Mandaean" => Mandaic,
    b"mani" => "Manichaean" => Manichaean,
    b"marc" => "Marchen" => Marchen,
    b"math" => "Mathematical Alphanumeric Symbols" => MathematicalAlphanumericSymbols,
    b"mtei" => "Meitei Mayek (Meithei, Meetei)" => MeiteiMayek,
    b"mend" => "Mende Kikakui" => MendeKikakui,
    b"merc" => "Meroitic Cursive" => MeroiticCursive,
    b"mero" => "Meroitic Hieroglyphs" => MeroiticHieroglyphs,
    b"plrd" => "Miao" => Miao,
    b"modi" => "Modi" => Modi,
    b"mong" => "Mongolian" => Mongolian,
    b"mroo" => "Mro" => Mro,
    b"mult" => "Multani" => Multani,
    b"musc" => "Musical Symbols" => MusicalSymbols,
    b"mymr" => "Myanmar" => Myanmar,
    b"mym2" => "Myanmar" => MyanmarV2,
    b"nbat" => "Nabataean" => Nabataean,
    b"newa" => "Newa" => Newa,
    b"talu" => "New Tai Lue" => NewTaiLue,
    b"nko " => "N’Ko" => NKo,
    b"orya" => "Odia" => Odia,
    b"ory2" => "Odia" => OdiaV2,
    b"ogam" => "Ogham" => Ogham,
    b"olck" => "Ol Chiki" => OlChiki,
    b"ital" => "Old Italic" => OldItalic,
    b"hung" => "Old Hungarian" => OldHungarian,
    b"narb" => "Old North Arabian" => OldNorthArabian,
    b"perm" => "Old Permic" => OldPermic,
    b"xpeo" => "Old Persian Cuneiform" => OldPersianCuneiform,
    b"sarb" => "Old South Arabian" => OldSouthArabian,
    b"orkh" => "Old Turkic, Orkhon Runic" => OldTurkic,
    b"osge" => "Osage" => Osage,
    b"osma" => "Osmanya" => Osmanya,
    b"hmng" => "Pahawh Hmong" => PahawhHmong,
    b"palm" => "Palmyrene" => Palmyrene,
    b"pauc" => "Pau Cin Hau" => PauCinHau,
    b"phag" => "Phags-pa" => Phagspa,
    b"phnx" => "Phoenician" => Phoenician,
    b"phlp" => "Psalter Pahlavi" => PsalterPahlavi,
    b"rjng" => "Rejang" => Rejang,
    b"runr" => "Runic" => Runic,
    b"samr" => "Samaritan" => Samaritan,
    b"saur" => "Saurashtra" => Saurashtra,
    b"shrd" => "Sharada" => Sharada,
    b"shaw" => "Shavian" => Shavian,
    b"sidd" => "Siddham" => Siddham,
    b"sgnw" => "Sign Writing" => SignWriting,
    b"sinh" => "Sinhala" => Sinhala,
    b"sora" => "Sora Sompeng" => SoraSompeng,
    b"xsux" => "Sumero-Akkadian Cuneiform" => SumeroAkkadianCuneiform,
    b"sund" => "Sundanese" => Sundanese,
    b"sylo" => "Syloti Nagri" => SylotiNagri,
    b"syrc" => "Syriac" => Syriac,
    b"tglg" => "Tagalog" => Tagalog,
    b"tagb" => "Tagbanwa" => Tagbanwa,
    b"tale" => "Tai Le" => TaiLe,
    b"lana" => "Tai Tham (Lanna)" => TaiTham,
    b"tavt" => "Tai Viet" => TaiViet,
    b"takr" => "Takri" => Takri,
    b"taml" => "Tamil" => Tamil,
    b"tml2" => "Tamil" => TamilV2,
    b"tang" => "Tangut" => Tangut,
    b"telu" => "Telugu" => Telugu,
    b"tel2" => "Telugu" => TeluguV2,
    b"thaa" => "Thaana" => Thaana,
    b"thai" => "Thai" => Thai,
    b"tibt" => "Tibetan" => Tibetan,
    b"tfng" => "Tifinagh" => Tifinagh,
    b"tirh" => "Tirhuta" => Tirhuta,
    b"ugar" => "Ugaritic Cuneiform" => UgariticCuneiform,
    b"vai " => "Vai" => Vai,
    b"wara" => "Warang Citi" => WarangCiti,
    b"yi  " => "Yi" => Yi,
}
