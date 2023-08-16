//! The feature list.

use truetype::Tag;

table! {
    @position
    #[doc = "A feature list."]
    pub Features { // FeatureList
        count (u16), // featureCount

        headers (Vec<Header>) |this, tape, _| { // featureRecords
            tape.take_given(this.count as usize)
        },

        records (Vec<Record>) |this, tape, position| {
            jump_take!(tape, position, this.count, i => this.headers[i].offset)
        },
    }
}

table! {
    #[doc = "A feature header."]
    #[derive(Copy)]
    pub Header { // FeatureRecord
        tag    (Tag), // featureTag
        offset (u16), // featureOffset
    }
}

table! {
    @position
    #[doc = "A feature record."]
    pub Record { // Feature
        parameter_offset   (u16), // featureParamsOffset
        lookup_index_count (u16), // lookupIndexCount

        lookup_indices (Vec<u16>) |this, tape, _| { // lookupListIndices
            tape.take_given(this.lookup_index_count as usize)
        },

        parameters (Option<Vec<u8>>) |this, tape, position| {
            if this.parameter_offset != 0 {
                tape.jump(position + this.parameter_offset as u64)?;
                Ok(Some(tape.take_bytes(0)?))
            } else {
                Ok(None)
            }
        },
    }
}

table! {
    #[doc = "Feature variations."]
    #[derive(Copy)]
    pub Variations { // FeatureVariations
        major_version (u16) = { 1 }, // MajorVersion
        minor_version (u16) = { 0 }, // MinorVersion
        count         (u32), // FeatureVariationRecordsCount
    }
}

impl Features {
    /// Return the record of a feature if present.
    pub fn get<T: Into<Tag>>(&self, tag: T) -> Option<&Record> {
        let tag = tag.into();
        self.headers
            .iter()
            .enumerate()
            .find(|(_, header)| header.tag == tag)
            .map(|(i, _)| &self.records[i])
    }
}

macro_rules! implement {
    ($($tag:literal => $name:literal => $variant:ident,)*) => (
        /// A feature.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Feature {
            $(#[doc = $name] $variant,)*
        }

        impl Feature {
            /// Create an instance from a tag.
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

        impl From<Feature> for Tag {
            fn from(feature: Feature) -> Self {
                match feature {
                    $(Feature::$variant => Tag(*$tag),)*
                }
            }
        }
    );
}

implement! {
    b"aalt" => "Access All Alternates" => AccessAllAlternates,
    b"abvf" => "Above-base Forms" => AboveBaseForms,
    b"abvm" => "Above-base Mark Positioning" => AboveBaseMarkPositioning,
    b"abvs" => "Above-base Substitutions" => AboveBaseSubstitutions,
    b"afrc" => "Alternative Fractions" => AlternativeFractions,
    b"akhn" => "Akhand" => Akhand,
    b"blwf" => "Below-base Forms" => BelowBaseForms,
    b"blwm" => "Below-base Mark Positioning" => BelowBaseMarkPositioning,
    b"blws" => "Below-base Substitutions" => BelowBaseSubstitutions,
    b"calt" => "Contextual Alternates" => ContextualAlternates,
    b"case" => "Case-sensitive Forms" => CaseSensitiveForms,
    b"ccmp" => "Glyph Composition and Decomposition" => GlyphCompositionDecomposition,
    b"cfar" => "Conjunct Form After Ro" => ConjunctFormAfterRo,
    b"chws" => "Contextual Half-width Spacing" => ContextualHalfWidthSpacing,
    b"cjct" => "Conjunct Forms" => ConjunctForms,
    b"clig" => "Contextual Ligatures" => ContextualLigatures,
    b"cpct" => "Centered CJK Punctuation" => CenteredCJKPunctuation,
    b"cpsp" => "Capital Spacing" => CapitalSpacing,
    b"cswh" => "Contextual Swash" => ContextualSwash,
    b"curs" => "Cursive Positioning" => CursivePositioning,
    b"cv01" => "Character Variants 1" => CharacterVariants1,
    b"cv02" => "Character Variants 2" => CharacterVariants2,
    b"cv03" => "Character Variants 3" => CharacterVariants3,
    b"cv04" => "Character Variants 4" => CharacterVariants4,
    b"cv05" => "Character Variants 5" => CharacterVariants5,
    b"cv06" => "Character Variants 6" => CharacterVariants6,
    b"cv07" => "Character Variants 7" => CharacterVariants7,
    b"cv08" => "Character Variants 8" => CharacterVariants8,
    b"cv09" => "Character Variants 9" => CharacterVariants9,
    b"cv10" => "Character Variants 10" => CharacterVariants10,
    b"cv11" => "Character Variants 11" => CharacterVariants11,
    b"cv12" => "Character Variants 12" => CharacterVariants12,
    b"cv13" => "Character Variants 13" => CharacterVariants13,
    b"cv14" => "Character Variants 14" => CharacterVariants14,
    b"cv15" => "Character Variants 15" => CharacterVariants15,
    b"cv16" => "Character Variants 16" => CharacterVariants16,
    b"cv17" => "Character Variants 17" => CharacterVariants17,
    b"cv18" => "Character Variants 18" => CharacterVariants18,
    b"cv19" => "Character Variants 19" => CharacterVariants19,
    b"cv20" => "Character Variants 20" => CharacterVariants20,
    b"cv21" => "Character Variants 21" => CharacterVariants21,
    b"cv22" => "Character Variants 22" => CharacterVariants22,
    b"cv23" => "Character Variants 23" => CharacterVariants23,
    b"cv24" => "Character Variants 24" => CharacterVariants24,
    b"cv25" => "Character Variants 25" => CharacterVariants25,
    b"cv26" => "Character Variants 26" => CharacterVariants26,
    b"cv27" => "Character Variants 27" => CharacterVariants27,
    b"cv28" => "Character Variants 28" => CharacterVariants28,
    b"cv29" => "Character Variants 29" => CharacterVariants29,
    b"cv30" => "Character Variants 30" => CharacterVariants30,
    b"cv31" => "Character Variants 31" => CharacterVariants31,
    b"cv32" => "Character Variants 32" => CharacterVariants32,
    b"cv33" => "Character Variants 33" => CharacterVariants33,
    b"cv34" => "Character Variants 34" => CharacterVariants34,
    b"cv35" => "Character Variants 35" => CharacterVariants35,
    b"cv36" => "Character Variants 36" => CharacterVariants36,
    b"cv37" => "Character Variants 37" => CharacterVariants37,
    b"cv38" => "Character Variants 38" => CharacterVariants38,
    b"cv39" => "Character Variants 39" => CharacterVariants39,
    b"cv40" => "Character Variants 40" => CharacterVariants40,
    b"cv41" => "Character Variants 41" => CharacterVariants41,
    b"cv42" => "Character Variants 42" => CharacterVariants42,
    b"cv43" => "Character Variants 43" => CharacterVariants43,
    b"cv44" => "Character Variants 44" => CharacterVariants44,
    b"cv45" => "Character Variants 45" => CharacterVariants45,
    b"cv46" => "Character Variants 46" => CharacterVariants46,
    b"cv47" => "Character Variants 47" => CharacterVariants47,
    b"cv48" => "Character Variants 48" => CharacterVariants48,
    b"cv49" => "Character Variants 49" => CharacterVariants49,
    b"cv50" => "Character Variants 50" => CharacterVariants50,
    b"cv51" => "Character Variants 51" => CharacterVariants51,
    b"cv52" => "Character Variants 52" => CharacterVariants52,
    b"cv53" => "Character Variants 53" => CharacterVariants53,
    b"cv54" => "Character Variants 54" => CharacterVariants54,
    b"cv55" => "Character Variants 55" => CharacterVariants55,
    b"cv56" => "Character Variants 56" => CharacterVariants56,
    b"cv57" => "Character Variants 57" => CharacterVariants57,
    b"cv58" => "Character Variants 58" => CharacterVariants58,
    b"cv59" => "Character Variants 59" => CharacterVariants59,
    b"cv60" => "Character Variants 60" => CharacterVariants60,
    b"cv61" => "Character Variants 61" => CharacterVariants61,
    b"cv62" => "Character Variants 62" => CharacterVariants62,
    b"cv63" => "Character Variants 63" => CharacterVariants63,
    b"cv64" => "Character Variants 64" => CharacterVariants64,
    b"cv65" => "Character Variants 65" => CharacterVariants65,
    b"cv66" => "Character Variants 66" => CharacterVariants66,
    b"cv67" => "Character Variants 67" => CharacterVariants67,
    b"cv68" => "Character Variants 68" => CharacterVariants68,
    b"cv69" => "Character Variants 69" => CharacterVariants69,
    b"cv70" => "Character Variants 70" => CharacterVariants70,
    b"cv71" => "Character Variants 71" => CharacterVariants71,
    b"cv72" => "Character Variants 72" => CharacterVariants72,
    b"cv73" => "Character Variants 73" => CharacterVariants73,
    b"cv74" => "Character Variants 74" => CharacterVariants74,
    b"cv75" => "Character Variants 75" => CharacterVariants75,
    b"cv76" => "Character Variants 76" => CharacterVariants76,
    b"cv77" => "Character Variants 77" => CharacterVariants77,
    b"cv78" => "Character Variants 78" => CharacterVariants78,
    b"cv79" => "Character Variants 79" => CharacterVariants79,
    b"cv80" => "Character Variants 80" => CharacterVariants80,
    b"cv81" => "Character Variants 81" => CharacterVariants81,
    b"cv82" => "Character Variants 82" => CharacterVariants82,
    b"cv83" => "Character Variants 83" => CharacterVariants83,
    b"cv84" => "Character Variants 84" => CharacterVariants84,
    b"cv85" => "Character Variants 85" => CharacterVariants85,
    b"cv86" => "Character Variants 86" => CharacterVariants86,
    b"cv87" => "Character Variants 87" => CharacterVariants87,
    b"cv88" => "Character Variants 88" => CharacterVariants88,
    b"cv89" => "Character Variants 89" => CharacterVariants89,
    b"cv90" => "Character Variants 90" => CharacterVariants90,
    b"cv91" => "Character Variants 91" => CharacterVariants91,
    b"cv92" => "Character Variants 92" => CharacterVariants92,
    b"cv93" => "Character Variants 93" => CharacterVariants93,
    b"cv94" => "Character Variants 94" => CharacterVariants94,
    b"cv95" => "Character Variants 95" => CharacterVariants95,
    b"cv96" => "Character Variants 96" => CharacterVariants96,
    b"cv97" => "Character Variants 97" => CharacterVariants97,
    b"cv98" => "Character Variants 98" => CharacterVariants98,
    b"cv99" => "Character Variants 99" => CharacterVariants99,
    b"c2pc" => "Petite Capitals from Capitals" => PetiteCapitalsFromCapitals,
    b"c2sc" => "Small Capitals from Capitals" => SmallCapitalsFromCapitals,
    b"dist" => "Distances" => Distances,
    b"dlig" => "Discretionary Ligatures" => DiscretionaryLigatures,
    b"dnom" => "Denominators" => Denominators,
    b"dtls" => "Dotless Forms" => DotlessForms,
    b"expt" => "Expert Forms" => ExpertForms,
    b"falt" => "Final Glyph on Line Alternates" => FinalGlyphonLineAlternates,
    b"fin2" => "Terminal Forms 2" => TerminalForms2,
    b"fin3" => "Terminal Forms 3" => TerminalForms3,
    b"fina" => "Terminal Forms 1" => TerminalForms1,
    b"flac" => "Flattened Accent Forms" => FlattenedAccentForms,
    b"frac" => "Fractions" => Fractions,
    b"fwid" => "Full Widths" => FullWidths,
    b"half" => "Half Forms" => HalfForms,
    b"haln" => "Halant Forms" => HalantForms,
    b"halt" => "Alternate Half Widths" => AlternateHalfWidths,
    b"hist" => "Historical Forms" => HistoricalForms,
    b"hkna" => "Horizontal Kana Alternates" => HorizontalKanaAlternates,
    b"hlig" => "Historical Ligatures" => HistoricalLigatures,
    b"hngl" => "Hangul" => Hangul,
    b"hojo" => "Hojo Kanji Forms" => HojoKanjiForms,
    b"hwid" => "Half Widths" => HalfWidths,
    b"init" => "Initial Forms" => InitialForms,
    b"isol" => "Isolated Forms" => IsolatedForms,
    b"ital" => "Italics" => Italics,
    b"jalt" => "Justification Alternates" => JustificationAlternates,
    b"jp78" => "JIS78 Forms" => JIS78Forms,
    b"jp83" => "JIS83 Forms" => JIS83Forms,
    b"jp90" => "JIS90 Forms" => JIS90Forms,
    b"jp04" => "JIS2004 Forms" => JIS2004Forms,
    b"kern" => "Kerning" => Kerning,
    b"lfbd" => "Left Bounds" => LeftBounds,
    b"liga" => "Standard Ligatures" => StandardLigatures,
    b"ljmo" => "Leading Jamo Forms" => LeadingJamoForms,
    b"lnum" => "Lining Figures" => LiningFigures,
    b"locl" => "Localized Forms" => LocalizedForms,
    b"ltra" => "Left-to-right Alternates" => LeftToRightAlternates,
    b"ltrm" => "Left-to-right Mirrored Forms" => LeftToRightMirroredForms,
    b"mark" => "Mark Positioning" => MarkPositioning,
    b"med2" => "Medial Forms 2" => MedialForms2,
    b"medi" => "Medial Forms 1" => MedialForms1,
    b"mgrk" => "Mathematical Greek" => MathematicalGreek,
    b"mkmk" => "Mark-to-mark Positioning" => MarkToMarkPositioning,
    b"mset" => "Mark Positioning via Substitution" => MarkPositioningViaSubstitution,
    b"nalt" => "Alternate Annotation Forms" => AlternateAnnotationForms,
    b"nlck" => "NLC Kanji Forms" => NLCKanjiForms,
    b"nukt" => "Nukta Forms" => NuktaForms,
    b"numr" => "Numerators" => Numerators,
    b"onum" => "Oldstyle Figures" => OldstyleFigures,
    b"opbd" => "Optical Bounds" => OpticalBounds,
    b"ordn" => "Ordinals" => Ordinals,
    b"ornm" => "Ornaments" => Ornaments,
    b"palt" => "Proportional Alternate Widths" => ProportionalAlternateWidths,
    b"pcap" => "Petite Capitals" => PetiteCapitals,
    b"pkna" => "Proportional Kana" => ProportionalKana,
    b"pnum" => "Proportional Figures" => ProportionalFigures,
    b"pref" => "Pre-base Forms" => PreBaseForms,
    b"pres" => "Pre-base Substitutions" => PreBaseSubstitutions,
    b"pstf" => "Post-base Forms" => PostBaseForms,
    b"psts" => "Post-base Substitutions" => PostBaseSubstitutions,
    b"pwid" => "Proportional Widths" => ProportionalWidths,
    b"qwid" => "Quarter Widths" => QuarterWidths,
    b"rand" => "Randomize" => Randomize,
    b"rclt" => "Required Contextual Alternates" => RequiredContextualAlternates,
    b"rkrf" => "Rakar Forms" => RakarForms,
    b"rlig" => "Required Ligatures" => RequiredLigatures,
    b"rphf" => "Reph Forms" => RephForms,
    b"rtbd" => "Right Bounds" => RightBounds,
    b"rtla" => "Right-to-left Alternates" => RightToLeftAlternates,
    b"rtlm" => "Right-to-left Mirrored Forms" => RightToLeftMirroredForms,
    b"ruby" => "Ruby Notation Forms" => RubyNotationForms,
    b"rvrn" => "Required Variation Alternates" => RequiredVariationAlternates,
    b"salt" => "Stylistic Alternates" => StylisticAlternates,
    b"sinf" => "Scientific Inferiors" => ScientificInferiors,
    b"size" => "Optical Size" => OpticalSize,
    b"smcp" => "Small Capitals" => SmallCapitals,
    b"smpl" => "Simplified Forms" => SimplifiedForms,
    b"ss01" => "Stylistic Set 1" => StylisticSet1,
    b"ss02" => "Stylistic Set 2" => StylisticSet2,
    b"ss03" => "Stylistic Set 3" => StylisticSet3,
    b"ss04" => "Stylistic Set 4" => StylisticSet4,
    b"ss05" => "Stylistic Set 5" => StylisticSet5,
    b"ss06" => "Stylistic Set 6" => StylisticSet6,
    b"ss07" => "Stylistic Set 7" => StylisticSet7,
    b"ss08" => "Stylistic Set 8" => StylisticSet8,
    b"ss09" => "Stylistic Set 9" => StylisticSet9,
    b"ss10" => "Stylistic Set 10" => StylisticSet10,
    b"ss11" => "Stylistic Set 11" => StylisticSet11,
    b"ss12" => "Stylistic Set 12" => StylisticSet12,
    b"ss13" => "Stylistic Set 13" => StylisticSet13,
    b"ss14" => "Stylistic Set 14" => StylisticSet14,
    b"ss15" => "Stylistic Set 15" => StylisticSet15,
    b"ss16" => "Stylistic Set 16" => StylisticSet16,
    b"ss17" => "Stylistic Set 17" => StylisticSet17,
    b"ss18" => "Stylistic Set 18" => StylisticSet18,
    b"ss19" => "Stylistic Set 19" => StylisticSet19,
    b"ss20" => "Stylistic Set 20" => StylisticSet20,
    b"ssty" => "Math Script Style Alternates" => MathScriptStyleAlternates,
    b"stch" => "Stretching Glyph Decomposition" => StretchingGlyphDecomposition,
    b"subs" => "Subscript" => Subscript,
    b"sups" => "Superscript" => Superscript,
    b"swsh" => "Swash" => Swash,
    b"titl" => "Titling" => Titling,
    b"tjmo" => "Trailing Jamo Forms" => TrailingJamoForms,
    b"tnam" => "Traditional Name Forms" => TraditionalNameForms,
    b"tnum" => "Tabular Figures" => TabularFigures,
    b"trad" => "Traditional Forms" => TraditionalForms,
    b"twid" => "Third Widths" => ThirdWidths,
    b"unic" => "Unicase" => Unicase,
    b"valt" => "Alternate Vertical Metrics" => AlternateVerticalMetrics,
    b"vatu" => "Vattu Variants" => VattuVariants,
    b"vchw" => "Vertical Contextual Half-width Spacing" => VerticalContextualHalfWidthSpacing,
    b"vert" => "Vertical Writing" => VerticalWriting,
    b"vhal" => "Alternate Vertical Half Metrics" => AlternateVerticalHalfMetrics,
    b"vjmo" => "Vowel Jamo Forms" => VowelJamoForms,
    b"vkna" => "Vertical Kana Alternates" => VerticalKanaAlternates,
    b"vkrn" => "Vertical Kerning" => VerticalKerning,
    b"vpal" => "Proportional Alternate Vertical Metrics" => ProportionalAlternateVerticalMetrics,
    b"vrt2" => "Vertical Alternates and Rotation" => VerticalAlternatesAndRotation,
    b"vrtr" => "Vertical Alternates for Rotation" => VerticalAlternatesForRotation,
    b"zero" => "Slashed Zero" => SlashedZero,
}
