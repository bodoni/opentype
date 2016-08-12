use truetype::GlyphID;

table! {
    #[doc = "A set of alternate glyphs."]
    pub AlternateSet {
        count (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // Alternate
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A ligature."]
    pub Ligature {
        glyph_id        (GlyphID), // LigGlyph
        component_count (u16    ), // CompCount

        component_ids (Vec<GlyphID>) |this, tape| { // Component
            if this.component_count == 0 {
                raise!("found a malformed ligature record");
            }
            tape.take_given(this.component_count as usize - 1)
        },
    }
}

table! {
    @position
    #[doc = "A set of ligatures."]
    pub LigatureSet {
        count (u16), // LigatureCount

        offsets (Vec<u16>) |this, tape, _| { // Ligature
            tape.take_given(this.count as usize)
        },

        records (Vec<Ligature>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A substitution rule."]
    pub Rule {
        glyph_count        (u16), // GlyphCount
        substitution_count (u16), // SubstCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // Input
            tape.take_given(this.glyph_count as usize)
        },

        substitutions (Vec<Substibution>) |this, tape| { // SubstLookupRecord
            tape.take_given(this.substitution_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of substitution rules."]
    pub RuleSet {
        count (u16), // SubRuleCount

        offsets (Vec<u16>) |this, tape, _| { // SubRule
            tape.take_given(this.count as usize)
        },

        records (Vec<Rule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A sequence of glyphs."]
    pub Sequence {
        count (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // Substitute
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A substitution."]
    #[derive(Copy)]
    pub Substibution {
        index        (u16), // SequenceIndex
        lookup_index (u16), // LookupListIndex
    }
}
