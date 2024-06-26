//! The language list.

use truetype::Tag;

table! {
    /// A language-system header.
    pub Header { // LangSysRecord
        tag    (Tag), // langSysTag
        offset (u16), // langSysOffset
    }
}

table! {
    /// A language-system record.
    pub Record { // LangSys
        lookup_order_offset    (u16) = { 0 }, // lookupOrderOffset
        required_feature_index (u16), // requiredFeatureIndex
        feature_index_count    (u16), // featureIndexCount

        feature_indices (Vec<u16>) |this, tape| { // featureIndices
            tape.take_given(this.feature_index_count as usize)
        },
    }
}

macro_rules! implement {
    ($(
        $(#[$attribute:meta])*
        $tag:literal => $name:literal => $variant:ident => $codes:literal,
    )*) => (
        /// A language system.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Language {
            $($(#[$attribute])* #[doc = $name] $variant,)*
            Other(Tag),
        }

        impl Language {
            /// Create an instance from a tag.
            pub fn from_tag(tag: &Tag) -> Self {
                match &**tag {
                    $($(#[$attribute])* $tag => Self::$variant,)*
                    _ => Self::Other(tag.clone()),
                }
            }

            /// Return ISO 639 codes.
            pub fn codes(&self) -> impl Iterator<Item = &'static str> {
                let filter = |code: &&str| !code.is_empty();
                let value = match self {
                    $($(#[$attribute])* Language::$variant => $codes,)*
                    _ => "",
                };
                value.split(", ").filter(filter)
            }

            /// Return the name.
            pub fn name(&self) -> Option<&'static str> {
                match self {
                    $($(#[$attribute])* Self::$variant => Some($name),)*
                    _ => None,
                }
            }
        }

        impl From<Language> for Tag {
            fn from(language: Language) -> Self {
                match language {
                    $($(#[$attribute])* Language::$variant => Tag(*$tag),)*
                    Language::Other(tag) => tag,
                }
            }
        }
    );
}

implement! {
    b"ABA " => "Abaza" => Abaza => "abq",
    b"ABK " => "Abkhazian" => Abkhazian => "abk",
    b"ACH " => "Acholi" => Acholi => "ach",
    b"ACR " => "Achi" => Achi => "acr",
    b"ADY " => "Adyghe" => Adyghe => "ady",
    b"AFK " => "Afrikaans" => Afrikaans => "afr",
    b"AFR " => "Afar" => Afar => "aar",
    b"AGW " => "Agaw" => Agaw => "ahg",
    b"AIO " => "Aiton" => Aiton => "aio",
    b"AKA " => "Akan" => Akan => "aka",
    b"ALS " => "Alsatian" => Alsatian => "gsw",
    b"ALT " => "Altai" => Altai => "atv, alt",
    b"AMH " => "Amharic" => Amharic => "amh",
    b"ANG " => "Anglo-Saxon" => AngloSaxon => "ang",
    b"APPH" => "Phonetic Transcription, Americanist" => AmericanistPhoneticNotation => "",
    b"ARA " => "Arabic" => Arabic => "ara",
    b"ARG " => "Aragonese" => Aragonese => "arg",
    b"ARI " => "Aari" => Aari => "aiw",
    b"ARK " => "Rakhine" => Rakhine => "mhv, rmz, rki",
    b"ASM " => "Assamese" => Assamese => "asm",
    b"AST " => "Asturian" => Asturian => "ast",
    b"ATH " => "Athapaskan" => Athapaskan => "apk, apj, apl, apm, apw, nav, bea, sek, bcr, caf, \
                                              crx, clc, gwi, haa, chp, dgr, scs, xsl, srs, ing, \
                                              hoi, koy, hup, ktw, mvb, wlk, coq, ctc, gce, tol, \
                                              tuu, kkz, tgx, tht, aht, tfn, taa, tau, tcb, kuu, \
                                              tce, ttm, txc",
    b"AVR " => "Avar" => Avar => "ava",
    b"AWA " => "Awadhi" => Awadhi => "awa",
    b"AYM " => "Aymara" => Aymara => "aym",
    b"AZB " => "Torki" => Torki => "azb",
    b"AZE " => "Azerbaijani" => Azerbaijani => "aze",
    b"BAD " => "Badaga" => Badaga => "bfq",
    b"BAD0" => "Banda" => Banda => "bad",
    b"BAG " => "Baghelkhandi" => Baghelkhandi => "bfy",
    b"BAL " => "Balkar" => Balkar => "krc",
    b"BAN " => "Balinese" => Balinese => "ban",
    b"BAR " => "Bavarian" => Bavarian => "bar",
    b"BAU " => "Baulé" => Baule => "bci",
    b"BBC " => "Batak Toba" => BatakToba => "bbc",
    b"BBR " => "Berber" => Berber => "",
    b"BCH " => "Bench" => Bench => "bcq",
    b"BCR " => "Bible Cree" => BibleCree => "",
    b"BDY " => "Bandjalang" => Bandjalang => "bdy",
    b"BEL " => "Belarussian" => Belarussian => "bel",
    b"BEM " => "Bemba" => Bemba => "bem",
    b"BEN " => "Bengali" => Bengali => "ben",
    b"BGC " => "Haryanvi" => Haryanvi => "bgc",
    b"BGQ " => "Bagri" => Bagri => "bgq",
    b"BGR " => "Bulgarian" => Bulgarian => "bul",
    b"BHI " => "Bhili" => Bhili => "bhi, bhb",
    b"BHO " => "Bhojpuri" => Bhojpuri => "bho",
    b"BIK " => "Bikol" => Bikol => "bik, bhk, bcl, bto, cts, bln",
    b"BIL " => "Bilen" => Bilen => "byn",
    b"BIS " => "Bislama" => Bislama => "bis",
    b"BJJ " => "Kanauji" => Kanauji => "bjj",
    b"BKF " => "Blackfoot" => Blackfoot => "bla",
    b"BLI " => "Baluchi" => Baluchi => "bal",
    b"BLK " => "Pa’o Karen" => PaoKaren => "blk",
    b"BLN " => "Balante" => Balante => "bjt, ble",
    b"BLT " => "Balti" => Balti => "bft",
    b"BMB " => "Bambara (Bamanankan)" => Bambara => "bam",
    b"BML " => "Bamileke" => Bamileke => "",
    b"BOS " => "Bosnian" => Bosnian => "bos",
    b"BPY " => "Bishnupriya Manipuri" => BishnupriyaManipuri => "bpy",
    b"BRE " => "Breton" => Breton => "bre",
    b"BRH " => "Brahui" => Brahui => "brh",
    b"BRI " => "Braj Bhasha" => BrajBhasha => "bra",
    b"BRM " => "Burmese" => Burmese => "mya",
    b"BRX " => "Bodo" => Bodo => "brx",
    b"BSH " => "Bashkir" => Bashkir => "bak",
    b"BSK " => "Burushaski" => Burushaski => "bsk",
    b"BTI " => "Beti" => Beti => "btb",
    b"BTS " => "Batak Simalungun" => BatakSimalungun => "bts",
    b"BUG " => "Bugis" => Bugis => "bug",
    b"BYV " => "Medumba" => Medumba => "byv",
    b"CAK " => "Kaqchikel" => Kaqchikel => "cak",
    b"CAT " => "Catalan" => Catalan => "cat",
    b"CBK " => "Zamboanga Chavacano" => ZamboangaChavacano => "cbk",
    b"CEB " => "Cebuano" => Cebuano => "ceb",
    b"CHE " => "Chechen" => Chechen => "che",
    b"CHG " => "Chaha Gurage" => ChahaGurage => "sgw",
    b"CHH " => "Chattisgarhi" => Chattisgarhi => "hne",
    b"CHI " => "Chichewa (Chewa, Nyanja)" => Chichewa => "nya",
    b"CHK " => "Chukchi" => Chukchi => "ckt",
    b"CHK0" => "Chuukese" => Chuukese => "chk",
    b"CHO " => "Choctaw" => Choctaw => "cho",
    b"CHP " => "Chipewyan" => Chipewyan => "chp",
    b"CHR " => "Cherokee" => Cherokee => "chr",
    b"CHA " => "Chamorro" => Chamorro => "cha",
    b"CHU " => "Chuvash" => Chuvash => "chv",
    b"CHY " => "Cheyenne" => Cheyenne => "chy",
    b"CGG " => "Chiga" => Chiga => "cgg",
    b"CMR " => "Comorian" => Comorian => "swb, wlc, wni, zdj",
    b"COP " => "Coptic" => Coptic => "cop",
    b"COR " => "Cornish" => Cornish => "cor",
    b"COS " => "Corsican" => Corsican => "cos",
    b"CPP " => "Creoles" => Creoles => "cpp",
    b"CRE " => "Cree" => Cree => "cre",
    b"CRR " => "Carrier" => Carrier => "crx, caf",
    b"CRT " => "Crimean Tatar" => CrimeanTatar => "crh",
    b"CSB " => "Kashubian" => Kashubian => "csb",
    b"CSL " => "Church Slavonic" => ChurchSlavonic => "chu",
    b"CSY " => "Czech" => Czech => "ces",
    b"CTG " => "Chittagonian" => Chittagonian => "ctg",
    b"CUK " => "San Blas Kuna" => SanBlasKuna => "cuk",
    b"DAN " => "Danish" => Danish => "dan",
    b"DAR " => "Dargwa" => Dargwa => "dar",
    b"DAX " => "Dayi" => Dayi => "dax",
    b"DCR " => "Woods Cree" => WoodsCree => "cwd",
    #[cfg(feature = "default-language")]
    b"DFLT" => "Default" => Default => "",
    b"DEU " => "German" => German => "deu",
    b"DGO " => "Dogri" => Dogri => "dgo",
    b"DGR " => "Dogri" => DogriMacrolanguage => "doi",
    b"DHG " => "Dhangu" => Dhangu => "dhg",
    b"DHV " => "Divehi (Dhivehi, Maldivian)" => DivehiDeprecated => "div",
    b"DIQ " => "Dimli" => Dimli => "diq",
    b"DIV " => "Divehi (Dhivehi, Maldivian)" => Divehi => "div",
    b"DJR " => "Zarma" => Zarma => "dje",
    b"DJR0" => "Djambarrpuyngu" => Djambarrpuyngu => "djr",
    b"DNG " => "Dangme" => Dangme => "ada",
    b"DNJ " => "Dan" => Dan => "dnj",
    b"DNK " => "Dinka" => Dinka => "din",
    b"DRI " => "Dari" => Dari => "prs",
    b"DUJ " => "Dhuwal" => Dhuwal => "duj",
    b"DUN " => "Dungan" => Dungan => "dng",
    b"DZN " => "Dzongkha" => Dzongkha => "dzo",
    b"EBI " => "Ebira" => Ebira => "igb",
    b"ECR " => "Eastern Cree" => EasternCree => "crj, crl",
    b"EDO " => "Edo" => Edo => "bin",
    b"EFI " => "Efik" => Efik => "efi",
    b"ELL " => "Greek" => Greek => "ell",
    b"EMK " => "Eastern Maninkakan" => EasternManinkakan => "emk",
    b"ENG " => "English" => English => "eng",
    b"ERZ " => "Erzya" => Erzya => "myv",
    b"ESP " => "Spanish" => Spanish => "spa",
    b"ESU " => "Central Yupik" => CentralYupik => "esu",
    b"ETI " => "Estonian" => Estonian => "est",
    b"EUQ " => "Basque" => Basque => "eus",
    b"EVK " => "Evenki" => Evenki => "evn",
    b"EVN " => "Even" => Even => "eve",
    b"EWE " => "Ewe" => Ewe => "ewe",
    b"FAN " => "French Antillean" => FrenchAntillean => "acf",
    b"FAN0" => "Fang" => Fang => "fan",
    b"FAR " => "Persian" => Persian => "fas",
    b"FAT " => "Fanti" => Fanti => "fat",
    b"FIN " => "Finnish" => Finnish => "fin",
    b"FJI " => "Fijian" => Fijian => "fij",
    b"FLE " => "Dutch (Flemish)" => DutchFlemish => "vls",
    b"FMP " => "Fe’fe’" => Fefe => "fmp",
    b"FNE " => "Forest Nenets" => ForestNenets => "enf",
    b"FON " => "Fon" => Fon => "fon",
    b"FOS " => "Faroese" => Faroese => "fao",
    b"FRA " => "French" => French => "fra",
    b"FRC " => "Cajun French" => CajunFrench => "frc",
    b"FRI " => "Frisian" => Frisian => "fry",
    b"FRL " => "Friulian" => Friulian => "fur",
    b"FRP " => "Arpitan" => Arpitan => "frp",
    b"FTA " => "Futa" => Futa => "fuf",
    b"FUL " => "Fulah" => Fulah => "ful",
    b"FUV " => "Nigerian Fulfulde" => NigerianFulfulde => "fuv",
    b"GAD " => "Ga" => Ga => "gaa",
    b"GAE " => "Scottish Gaelic (Gaelic)" => ScottishGaelic => "gla",
    b"GAG " => "Gagauz" => Gagauz => "gag",
    b"GAL " => "Galician" => Galician => "glg",
    b"GAR " => "Garshuni" => Garshuni => "",
    b"GAW " => "Garhwali" => Garhwali => "gbm",
    b"GEZ " => "Ge’ez" => Geez => "gez",
    b"GIH " => "Githabul" => Githabul => "gih",
    b"GIL " => "Gilyak" => Gilyak => "niv",
    b"GIL0" => "Kiribati (Gilbertese)" => Kiribati => "gil",
    b"GKP " => "Kpelle (Guinea)" => KpelleGuinea => "gkp",
    b"GLK " => "Gilaki" => Gilaki => "glk",
    b"GMZ " => "Gumuz" => Gumuz => "guk",
    b"GNN " => "Gumatj" => Gumatj => "gnn",
    b"GOG " => "Gogo" => Gogo => "gog",
    b"GON " => "Gondi" => Gondi => "gon",
    b"GRN " => "Greenlandic" => Greenlandic => "kal",
    b"GRO " => "Garo" => Garo => "grt",
    b"GUA " => "Guarani" => Guarani => "grn",
    b"GUC " => "Wayuu" => Wayuu => "guc",
    b"GUF " => "Gupapuyngu" => Gupapuyngu => "guf",
    b"GUJ " => "Gujarati" => Gujarati => "guj",
    b"GUZ " => "Gusii" => Gusii => "guz",
    b"HAI " => "Haitian (Haitian Creole)" => Haitian => "hat",
    b"HAL " => "Halam" => Halam => "flm",
    b"HAR " => "Harauti" => Harauti => "hoj",
    b"HAU " => "Hausa" => Hausa => "hau",
    b"HAW " => "Hawaiian" => Hawaiian => "haw",
    b"HAY " => "Haya" => Haya => "hay",
    b"HAZ " => "Hazaragi" => Hazaragi => "haz",
    b"HBN " => "Hammer-Banna" => HammerBanna => "amf",
    b"HER " => "Herero" => Herero => "her",
    b"HIL " => "Hiligaynon" => Hiligaynon => "hil",
    b"HIN " => "Hindi" => Hindi => "hin",
    b"HMA " => "High Mari" => HighMari => "mrj",
    b"HMN " => "Hmong" => Hmong => "hmn",
    b"HMO " => "Hiri Motu" => HiriMotu => "hmo",
    b"HND " => "Hindko" => Hindko => "hno, hnd",
    b"HO  " => "Ho" => Ho => "hoc",
    b"HRI " => "Harari" => Harari => "har",
    b"HRV " => "Croatian" => Croatian => "hrv",
    b"HUN " => "Hungarian" => Hungarian => "hun",
    b"HYE " => "Armenian" => Armenian => "hye",
    b"HYE0" => "Armenian East" => ArmenianEast => "hye",
    b"IBA " => "Iban" => Iban => "iba",
    b"IBB " => "Ibibio" => Ibibio => "ibb",
    b"IBO " => "Igbo" => Igbo => "ibo",
    b"IJO " => "Ijo languages" => Ijolanguages => "ijc",
    b"IDO " => "Ido" => Ido => "ido",
    b"ILE " => "Interlingue" => Interlingue => "ile",
    b"ILO " => "Ilokano" => Ilokano => "ilo",
    b"INA " => "Interlingua" => Interlingua => "ina",
    b"IND " => "Indonesian" => Indonesian => "ind",
    b"ING " => "Ingush" => Ingush => "inh",
    b"INU " => "Inuktitut" => Inuktitut => "iku",
    b"IPK " => "Inupiat" => Inupiat => "ipk",
    b"IPPH" => "Phonetic transcription, IPA" => InternationalPhoneticAlphabet => "",
    b"IRI " => "Irish" => Irish => "gle",
    b"IRT " => "Irish Traditional" => IrishTraditional => "gle",
    b"ISL " => "Icelandic" => Icelandic => "isl",
    b"ISM " => "Inari Sami" => InariSami => "smn",
    b"ITA " => "Italian" => Italian => "ita",
    b"IWR " => "Hebrew" => Hebrew => "heb",
    b"JAM " => "Jamaican Creole" => JamaicanCreole => "jam",
    b"JAN " => "Japanese" => Japanese => "jpn",
    b"JAV " => "Javanese" => Javanese => "jav",
    b"JBO " => "Lojban" => Lojban => "jbo",
    b"JCT " => "Krymchak" => Krymchak => "jct",
    b"JII " => "Yiddish" => Yiddish => "yid",
    b"JUD " => "Ladino" => Ladino => "lad",
    b"JUL " => "Jula" => Jula => "dyu",
    b"KAB " => "Kabardian" => Kabardian => "kbd",
    b"KAB0" => "Kabyle" => Kabyle => "kab",
    b"KAC " => "Kachchi" => Kachchi => "kfr",
    b"KAL " => "Kalenjin" => Kalenjin => "kln",
    b"KAN " => "Kannada" => Kannada => "kan",
    b"KAR " => "Karachay" => Karachay => "krc",
    b"KAT " => "Georgian" => Georgian => "kat",
    b"KAZ " => "Kazakh" => Kazakh => "kaz",
    b"KDE " => "Makonde" => Makonde => "kde",
    b"KEA " => "Kabuverdianu (Crioulo)" => Kabuverdianu => "kea",
    b"KEB " => "Kebena" => Kebena => "ktb",
    b"KEK " => "Kekchi" => Kekchi => "kek",
    b"KGE " => "Khutsuri Georgian" => KhutsuriGeorgian => "kat",
    b"KHA " => "Khakass" => Khakass => "kjh",
    b"KHK " => "Khanty-Kazim" => KhantyKazim => "kca",
    b"KHM " => "Khmer" => Khmer => "khm",
    b"KHS " => "Khanty-Shurishkar" => KhantyShurishkar => "kca",
    b"KHT " => "Khamti Shan" => KhamtiShan => "kht",
    b"KHV " => "Khanty-Vakhi" => KhantyVakhi => "kca",
    b"KHW " => "Khowar" => Khowar => "khw",
    b"KIK " => "Kikuyu (Gikuyu)" => Kikuyu => "kik",
    b"KIR " => "Kirghiz (Kyrgyz)" => Kirghiz => "kir",
    b"KIS " => "Kisii" => Kisii => "kqs, kss",
    b"KIU " => "Kirmanjki" => Kirmanjki => "kiu",
    b"KJD " => "Southern Kiwai" => SouthernKiwai => "kjd",
    b"KJP " => "Eastern Pwo Karen" => EasternPwoKaren => "kjp",
    b"KKN " => "Kokni" => Kokni => "kex",
    b"KLM " => "Kalmyk" => Kalmyk => "xal",
    b"KMB " => "Kamba" => Kamba => "kam",
    b"KMN " => "Kumaoni" => Kumaoni => "kfy",
    b"KMO " => "Komo" => Komo => "kmw",
    b"KMS " => "Komso" => Komso => "kxc",
    b"KMZ " => "Khorasani Turkic" => KhorasaniTurkic => "kmz",
    b"KNR " => "Kanuri" => Kanuri => "kau",
    b"KOD " => "Kodagu" => Kodagu => "kfa",
    b"KOH " => "Korean Old Hangul" => KoreanOldHangul => "okm",
    b"KOK " => "Konkani" => Konkani => "kok",
    b"KON " => "Kikongo" => Kikongo => "ktu",
    b"KOM " => "Komi" => Komi => "kom",
    b"KON0" => "Kongo" => Kongo => "kon",
    b"KOP " => "Komi-Permyak" => KomiPermyak => "koi",
    b"KOR " => "Korean" => Korean => "kor",
    b"KOS " => "Kosraean" => Kosraean => "kos",
    b"KOZ " => "Komi-Zyrian" => KomiZyrian => "kpv",
    b"KPL " => "Kpelle" => Kpelle => "kpe",
    b"KRI " => "Krio" => Krio => "kri",
    b"KRK " => "Karakalpak" => Karakalpak => "kaa",
    b"KRL " => "Karelian" => Karelian => "krl",
    b"KRM " => "Karaim" => Karaim => "kdr",
    b"KRN " => "Karen" => Karen => "kar",
    b"KRT " => "Koorete" => Koorete => "kqy",
    b"KSH " => "Kashmiri" => Kashmiri => "kas",
    b"KSH0" => "Ripuarian" => Ripuarian => "ksh",
    b"KSI " => "Khasi" => Khasi => "kha",
    b"KSM " => "Kildin Sami" => KildinSami => "sjd",
    b"KSW " => "S’gaw Karen" => SgawKaren => "ksw",
    b"KUA " => "Kuanyama" => Kuanyama => "kua",
    b"KUI " => "Kui" => Kui => "kxu",
    b"KUL " => "Kulvi" => Kulvi => "kfx",
    b"KUM " => "Kumyk" => Kumyk => "kum",
    b"KUR " => "Kurdish" => Kurdish => "kur",
    b"KUU " => "Kurukh" => Kurukh => "kru",
    b"KUY " => "Kuy" => Kuy => "kdt",
    b"KYK " => "Koryak" => Koryak => "kpy",
    b"KYU " => "Western Kayah" => WesternKayah => "kyu",
    b"LAD " => "Ladin" => Ladin => "lld",
    b"LAH " => "Lahuli" => Lahuli => "bfu",
    b"LAK " => "Lak" => Lak => "lbe",
    b"LAM " => "Lambani" => Lambani => "lmn",
    b"LAO " => "Lao" => Lao => "lao",
    b"LAT " => "Latin" => Latin => "lat",
    b"LAZ " => "Laz" => Laz => "lzz",
    b"LCR " => "L-Cree" => LCree => "crm",
    b"LDK " => "Ladakhi" => Ladakhi => "lbj",
    b"LEZ " => "Lezgi" => Lezgi => "lez",
    b"LIJ " => "Ligurian" => Ligurian => "lij",
    b"LIM " => "Limburgish" => Limburgish => "lim",
    b"LIN " => "Lingala" => Lingala => "lin",
    b"LIS " => "Lisu" => Lisu => "lis",
    b"LJP " => "Lampung" => Lampung => "ljp",
    b"LKI " => "Laki" => Laki => "lki",
    b"LMA " => "Low Mari" => LowMari => "mhr",
    b"LMB " => "Limbu" => Limbu => "lif",
    b"LMO " => "Lombard" => Lombard => "lmo",
    b"LMW " => "Lomwe" => Lomwe => "ngl",
    b"LOM " => "Loma" => Loma => "lom",
    b"LRC " => "Luri" => Luri => "lrc, luz, bqi, zum",
    b"LSB " => "Lower Sorbian" => LowerSorbian => "dsb",
    b"LSM " => "Lule Sami" => LuleSami => "smj",
    b"LTH " => "Lithuanian" => Lithuanian => "lit",
    b"LTZ " => "Luxembourgish" => Luxembourgish => "ltz",
    b"LUA " => "Luba-Lulua" => LubaLulua => "lua",
    b"LUB " => "Luba-Katanga" => LubaKatanga => "lub",
    b"LUG " => "Ganda" => Ganda => "lug",
    b"LUH " => "Luyia" => Luyia => "luy",
    b"LUO " => "Luo" => Luo => "luo",
    b"LVI " => "Latvian" => Latvian => "lav",
    b"MAD " => "Madura" => Madura => "mad",
    b"MAG " => "Magahi" => Magahi => "mag",
    b"MAH " => "Marshallese" => Marshallese => "mah",
    b"MAJ " => "Majang" => Majang => "mpe",
    b"MAK " => "Makhuwa" => Makhuwa => "vmw",
    b"MAL " => "Malayalam" => Malayalam => "mal",
    b"MAM " => "Mam" => Mam => "mam",
    b"MAN " => "Mansi" => Mansi => "mns",
    b"MAP " => "Mapudungun" => Mapudungun => "arn",
    b"MAR " => "Marathi" => Marathi => "mar",
    b"MAW " => "Marwari" => Marwari => "mwr, dhd, rwr, mve, wry, mtr, swv",
    b"MBN " => "Mbundu" => Mbundu => "kmb",
    b"MBO " => "Mbo" => Mbo => "mbo",
    b"MCH " => "Manchu" => Manchu => "mnc",
    b"MCR " => "Moose Cree" => MooseCree => "crm",
    b"MDE " => "Mende" => Mende => "men",
    b"MDR " => "Mandar" => Mandar => "mdr",
    b"MEN " => "Me’en" => Meen => "mym",
    b"MER " => "Meru" => Meru => "mer",
    b"MFE " => "Morisyen" => Morisyen => "mfe",
    b"MIN " => "Minangkabau" => Minangkabau => "min",
    b"MIZ " => "Mizo" => Mizo => "lus",
    b"MKD " => "Macedonian" => Macedonian => "mkd",
    b"MKR " => "Makasar" => Makasar => "mak",
    b"MKW " => "Kituba" => Kituba => "mkw",
    b"MLE " => "Male" => Male => "mdy",
    b"MLG " => "Malagasy" => Malagasy => "mlg",
    b"MLN " => "Malinke" => Malinke => "mlq",
    b"MLR " => "Malayalam Reformed" => MalayalamReformed => "mal",
    b"MLY " => "Malay" => Malay => "msa",
    b"MND " => "Mandinka" => Mandinka => "mnk",
    b"MNG " => "Mongolian" => Mongolian => "mon",
    b"MNI " => "Manipuri" => Manipuri => "mni",
    b"MNK " => "Maninka" => Maninka => "man, mnk, myq, mku, msc, emk, mwk, mlq",
    b"MNX " => "Manx" => Manx => "glv",
    b"MOH " => "Mohawk" => Mohawk => "moh",
    b"MOK " => "Moksha" => Moksha => "mdf",
    b"MOL " => "Moldavian" => Moldavian => "mol",
    b"MON " => "Mon" => Mon => "mnw",
    b"MOR " => "Moroccan" => Moroccan => "",
    b"MOS " => "Mossi" => Mossi => "mos",
    b"MRI " => "Maori" => Maori => "mri",
    b"MTH " => "Maithili" => Maithili => "mai",
    b"MTS " => "Maltese" => Maltese => "mlt",
    b"MUN " => "Mundari" => Mundari => "unr",
    b"MUS " => "Muscogee" => Muscogee => "mus",
    b"MWL " => "Mirandese" => Mirandese => "mwl",
    b"MWW " => "Hmong Daw" => HmongDaw => "mww",
    b"MYN " => "Mayan" => Mayan => "myn",
    b"MZN " => "Mazanderani" => Mazanderani => "mzn",
    b"NAG " => "Naga-Assamese" => NagaAssamese => "nag",
    b"NAH " => "Nahuatl" => Nahuatl => "nah",
    b"NAN " => "Nanai" => Nanai => "gld",
    b"NAP " => "Neapolitan" => Neapolitan => "nap",
    b"NAS " => "Naskapi" => Naskapi => "nsk",
    b"NAU " => "Nauruan" => Nauruan => "nau",
    b"NAV " => "Navajo" => Navajo => "nav",
    b"NCR " => "N-Cree" => NCree => "csw",
    b"NDB " => "Ndebele" => Ndebele => "nbl, nde",
    b"NDC " => "Ndau" => Ndau => "ndc",
    b"NDG " => "Ndonga" => Ndonga => "ndo",
    b"NDS " => "Low Saxon" => LowSaxon => "nds",
    b"NEP " => "Nepali" => Nepali => "nep",
    b"NEW " => "Newari" => Newari => "new",
    b"NGA " => "Ngbaka" => Ngbaka => "nga",
    b"NGR " => "Nagari" => Nagari => "",
    b"NHC " => "Norway House Cree" => NorwayHouseCree => "csw",
    b"NIS " => "Nisi" => Nisi => "dap",
    b"NIU " => "Niuean" => Niuean => "niu",
    b"NKL " => "Nyankole" => Nyankole => "nyn",
    b"NKO " => "N’Ko" => NKo => "nqo",
    b"NLD " => "Dutch" => Dutch => "nld",
    b"NOE " => "Nimadi" => Nimadi => "noe",
    b"NOG " => "Nogai" => Nogai => "nog",
    b"NOR " => "Norwegian" => Norwegian => "nob",
    b"NOV " => "Novial" => Novial => "nov",
    b"NSM " => "Northern Sami" => NorthernSami => "sme",
    b"NSO " => "Sotho, Northern" => NorthernSotho => "nso",
    b"NTA " => "Northern Tai" => NorthernTai => "nod",
    b"NTO " => "Esperanto" => Esperanto => "epo",
    b"NYM " => "Nyamwezi" => Nyamwezi => "nym",
    b"NYN " => "Norwegian Nynorsk (Nynorsk, Norwegian)" => NorwegianNynorsk => "nno",
    b"NZA " => "Mbembe Tigon" => MbembeTigon => "nza",
    b"OCI " => "Occitan" => Occitan => "oci",
    b"OCR " => "Oji-Cree" => OjiCree => "ojs",
    b"OJB " => "Ojibway" => Ojibway => "oji",
    b"ORI " => "Odia" => Odia => "ori",
    b"ORO " => "Oromo" => Oromo => "orm",
    b"OSS " => "Ossetian" => Ossetian => "oss",
    b"PAA " => "Palestinian Aramaic" => PalestinianAramaic => "sam",
    b"PAG " => "Pangasinan" => Pangasinan => "pag",
    b"PAL " => "Pali" => Pali => "pli",
    b"PAM " => "Pampangan" => Pampangan => "pam",
    b"PAN " => "Punjabi" => Punjabi => "pan",
    b"PAP " => "Palpa" => Palpa => "plp",
    b"PAP0" => "Papiamentu" => Papiamentu => "pap",
    b"PAS " => "Pashto" => Pashto => "pus",
    b"PAU " => "Palauan" => Palauan => "pau",
    b"PCC " => "Bouyei" => Bouyei => "pcc",
    b"PCD " => "Picard" => Picard => "pcd",
    b"PDC " => "Pennsylvania German" => PennsylvaniaGerman => "pdc",
    b"PGR " => "Polytonic Greek" => PolytonicGreek => "ell",
    b"PHK " => "Phake" => Phake => "phk",
    b"PIH " => "Norfolk" => Norfolk => "pih",
    b"PIL " => "Filipino" => Filipino => "fil",
    b"PLG " => "Palaung" => Palaung => "pce, rbb, pll",
    b"PLK " => "Polish" => Polish => "pol",
    b"PMS " => "Piemontese" => Piemontese => "pms",
    b"PNB " => "Western Panjabi" => WesternPanjabi => "pnb",
    b"POH " => "Pocomchi" => Pocomchi => "poh",
    b"PON " => "Pohnpeian" => Pohnpeian => "pon",
    b"PRO " => "Provencal" => Provencal => "pro",
    b"PTG " => "Portuguese" => Portuguese => "por",
    b"PWO " => "Western Pwo Karen" => WesternPwoKaren => "pwo",
    b"QIN " => "Chin" => Chin => "bgr, cnh, cnw, czt, sez, tcp, csy, ctd, flm, pck, tcz, zom, \
                                  cmr, dao, hlt, cka, cnk, mrh, mwg, cbl, cnb, csh",
    b"QUC " => "K’iche’" => Kiche => "quc",
    b"QUH " => "Quechua (Bolivia)" => QuechuaBolivia => "quh",
    b"QUZ " => "Quechua" => Quechua => "quz",
    b"QVI " => "Quechua (Ecuador)" => QuechuaEcuador => "qvi",
    b"QWH " => "Quechua (Peru)" => QuechuaPeru => "qwh",
    b"RAJ " => "Rajasthani" => Rajasthani => "raj",
    b"RAR " => "Rarotongan" => Rarotongan => "rar",
    b"RBU " => "Russian Buriat" => RussianBuriat => "bxr",
    b"RCR " => "R-Cree" => RCree => "atj",
    b"REJ " => "Rejang" => Rejang => "rej",
    b"RIA " => "Riang" => Riang => "ria",
    b"RIF " => "Tarifit" => Tarifit => "rif",
    b"RIT " => "Ritarungo" => Ritarungo => "rit",
    b"RKW " => "Arakwal" => Arakwal => "rkw",
    b"RMS " => "Romansh" => Romansh => "roh",
    b"RMY " => "Vlax Romani" => VlaxRomani => "rmy",
    b"ROM " => "Romanian" => Romanian => "ron",
    b"ROY " => "Romany" => Romany => "rom",
    b"RSY " => "Rusyn" => Rusyn => "rue",
    b"RTM " => "Rotuman" => Rotuman => "rtm",
    b"RUA " => "Kinyarwanda" => Kinyarwanda => "kin",
    b"RUN " => "Rundi" => Rundi => "run",
    b"RUP " => "Aromanian" => Aromanian => "rup",
    b"RUS " => "Russian" => Russian => "rus",
    b"SAD " => "Sadri" => Sadri => "sck",
    b"SAN " => "Sanskrit" => Sanskrit => "san",
    b"SAS " => "Sasak" => Sasak => "sas",
    b"SAT " => "Santali" => Santali => "sat",
    b"SAY " => "Sayisi" => Sayisi => "chp",
    b"SCN " => "Sicilian" => Sicilian => "scn",
    b"SCO " => "Scots" => Scots => "sco",
    b"SEK " => "Sekota" => Sekota => "xan",
    b"SEL " => "Selkup" => Selkup => "sel",
    b"SGA " => "Old Irish" => OldIrish => "sga",
    b"SGO " => "Sango" => Sango => "sag",
    b"SGS " => "Samogitian" => Samogitian => "sgs",
    b"SHI " => "Tachelhit" => Tachelhit => "shi",
    b"SHN " => "Shan" => Shan => "shn",
    b"SIB " => "Sibe" => Sibe => "sjo",
    b"SID " => "Sidamo" => Sidamo => "sid",
    b"SIG " => "Silte Gurage" => SilteGurage => "xst",
    b"SKS " => "Skolt Sami" => SkoltSami => "sms",
    b"SKY " => "Slovak" => Slovak => "slk",
    b"SCS " => "North Slavey" => NorthSlavey => "scs",
    b"SLA " => "Slavey" => Slavey => "scs, xsl",
    b"SLV " => "Slovenian" => Slovenian => "slv",
    b"SML " => "Somali" => Somali => "som",
    b"SMO " => "Samoan" => Samoan => "smo",
    b"SNA " => "Sena" => Sena => "seh",
    b"SNA0" => "Shona" => Shona => "sna",
    b"SND " => "Sindhi" => Sindhi => "snd",
    b"SNH " => "Sinhala (Sinhalese)" => Sinhala => "sin",
    b"SNK " => "Soninke" => Soninke => "snk",
    b"SOG " => "Sodo Gurage" => SodoGurage => "gru",
    b"SOP " => "Songe" => Songe => "sop",
    b"SOT " => "Sotho, Southern" => SouthernSotho => "sot",
    b"SQI " => "Albanian" => Albanian => "sqi",
    b"SRB " => "Serbian" => Serbian => "srp",
    b"SRD " => "Sardinian" => Sardinian => "srd",
    b"SRK " => "Saraiki" => Saraiki => "skr",
    b"SRR " => "Serer" => Serer => "srr",
    b"SSL " => "South Slavey" => SouthSlavey => "xsl",
    b"SSM " => "Southern Sami" => SouthernSami => "sma",
    b"STQ " => "Saterland Frisian" => SaterlandFrisian => "stq",
    b"SUK " => "Sukuma" => Sukuma => "suk",
    b"SUN " => "Sundanese" => Sundanese => "sun",
    b"SUR " => "Suri" => Suri => "suq",
    b"SVA " => "Svan" => Svan => "sva",
    b"SVE " => "Swedish" => Swedish => "swe",
    b"SWA " => "Swadaya Aramaic" => SwadayaAramaic => "aii",
    b"SWK " => "Swahili" => Swahili => "swa",
    b"SWZ " => "Swati" => Swati => "ssw",
    b"SXT " => "Sutu" => Sutu => "ngo",
    b"SXU " => "Upper Saxon" => UpperSaxon => "sxu",
    b"SYL " => "Sylheti" => Sylheti => "syl",
    b"SYR " => "Syriac" => Syriac => "syr",
    b"SZL " => "Silesian" => Silesian => "szl",
    b"TAB " => "Tabasaran" => Tabasaran => "tab",
    b"TAJ " => "Tajiki" => Tajiki => "tgk",
    b"TAM " => "Tamil" => Tamil => "tam",
    b"TAT " => "Tatar" => Tatar => "tat",
    b"TCR " => "TH-Cree" => THCree => "cwd",
    b"TDD " => "Dehong Dai" => DehongDai => "tdd",
    b"TEL " => "Telugu" => Telugu => "tel",
    b"TET " => "Tetum" => Tetum => "tet",
    b"TGL " => "Tagalog" => Tagalog => "tgl",
    b"TGN " => "Tongan" => Tongan => "ton",
    b"TGR " => "Tigre" => Tigre => "tig",
    b"TGY " => "Tigrinya" => Tigrinya => "tir",
    b"THA " => "Thai" => Thai => "tha",
    b"THT " => "Tahitian" => Tahitian => "tah",
    b"TIB " => "Tibetan" => Tibetan => "bod",
    b"TIV " => "Tiv" => Tiv => "tiv",
    b"TKM " => "Turkmen" => Turkmen => "tuk",
    b"TMH " => "Tamashek" => Tamashek => "tmh",
    b"TMN " => "Temne" => Temne => "tem",
    b"TNA " => "Tswana" => Tswana => "tsn",
    b"TNE " => "Tundra Nenets" => TundraNenets => "enh",
    b"TNG " => "Tonga" => Tonga => "toi",
    b"TOD " => "Todo" => Todo => "xal",
    b"TOD0" => "Toma" => Toma => "tod",
    b"TPI " => "Tok Pisin" => TokPisin => "tpi",
    b"TRK " => "Turkish" => Turkish => "tur",
    b"TSG " => "Tsonga" => Tsonga => "tso",
    b"TUA " => "Turoyo Aramaic" => TuroyoAramaic => "tru",
    b"TUM " => "Tulu" => Tulu => "tum",
    b"TUL " => "Tumbuka" => Tumbuka => "tcy",
    b"TUV " => "Tuvin" => Tuvin => "tyv",
    b"TVL " => "Tuvalu" => Tuvalu => "tvl",
    b"TWI " => "Twi" => Twi => "aka",
    b"TYZ " => "Tày" => Tay => "tyz",
    b"TZM " => "Tamazight" => Tamazight => "tzm",
    b"TZO " => "Tzotzil" => Tzotzil => "tzo",
    b"UDM " => "Udmurt" => Udmurt => "udm",
    b"UKR " => "Ukrainian" => Ukrainian => "ukr",
    b"UMB " => "Umbundu" => Umbundu => "umb",
    b"URD " => "Urdu" => Urdu => "urd",
    b"USB " => "Upper Sorbian" => UpperSorbian => "hsb",
    b"UYG " => "Uyghur" => Uyghur => "uig",
    b"UZB " => "Uzbek" => Uzbek => "uzb",
    b"VEC " => "Venetian" => Venetian => "vec",
    b"VEN " => "Venda" => Venda => "ven",
    b"VIT " => "Vietnamese" => Vietnamese => "vie",
    b"VOL " => "Volapük" => Volapuk => "vol",
    b"VRO " => "Võro" => Voro => "vro",
    b"WA  " => "Wa" => Wa => "wbm",
    b"WAG " => "Wagdi" => Wagdi => "wbr",
    b"WAR " => "Waray-Waray" => WarayWaray => "war",
    b"WCR " => "West-Cree" => WestCree => "crk",
    b"WEL " => "Welsh" => Welsh => "cym",
    b"WLN " => "Walloon" => Walloon => "wln",
    b"WLF " => "Wolof" => Wolof => "wol",
    b"WTM " => "Mewati" => Mewati => "wtm",
    b"XBD " => "Lü" => Lu => "khb",
    b"XHS " => "Xhosa" => Xhosa => "xho",
    b"XJB " => "Minjangbal" => Minjangbal => "xjb",
    b"XOG " => "Soga" => Soga => "xog",
    b"XPE " => "Kpelle (Liberia)" => KpelleLiberia => "xpe",
    b"YAK " => "Sakha" => Sakha => "sah",
    b"YAO " => "Yao" => Yao => "yao",
    b"YAP " => "Yapese" => Yapese => "yap",
    b"YBA " => "Yoruba" => Yoruba => "yor",
    b"YCR " => "Y-Cree" => YCree => "cre",
    b"YIC " => "Yi Classic" => YiClassic => "",
    b"YIM " => "Yi Modern" => YiModern => "iii",
    b"ZEA " => "Zealandic" => Zealandic => "zea",
    b"ZGH " => "Standard Morrocan Tamazigh" => StandardMorrocanTamazigh => "zgh",
    b"ZHA " => "Zhuang" => Zhuang => "zha",
    b"ZHH " => "Chinese, Hong Kong SAR" => Chinese => "zho",
    b"ZHP " => "Chinese Phonetic" => ChinesePhonetic => "zho",
    b"ZHS " => "Chinese Simplified" => ChineseSimplified => "zho",
    b"ZHT " => "Chinese Traditional" => ChineseTraditional => "zho",
    b"ZND " => "Zande" => Zande => "zne",
    b"ZUL " => "Zulu" => Zulu => "zul",
    b"ZZA " => "Zazaki" => Zazaki => "zza",
}

#[cfg(test)]
mod tests {
    use truetype::Tag;

    use super::Language;

    #[test]
    fn codes() {
        assert_eq!(Language::from_tag(&Tag(*b"IPPH")).codes().count(), 0);
        assert_eq!(Language::from_tag(&Tag(*b"ATH ")).codes().count(), 43);
    }
}
