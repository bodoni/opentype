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
    #[doc = "A chain rule."]
    pub ChainRule {
        backward_glyph_count (u16), // BacktrackGlyphCount

        backward_glyph_ids (Vec<GlyphID>) |this, tape| { // Backtrack
            tape.take_given(this.backward_glyph_count as usize)
        },

        input_glyph_count (u16), // InputGlyphCount

        input_glyph_ids (Vec<GlyphID>) |this, tape| { // Input
            tape.take_given(this.input_glyph_count as usize)
        },

        forward_glyph_count (u16), // LookaheadGlyphCount

        forward_glyph_ids (Vec<GlyphID>) |this, tape| { // LookAhead
            tape.take_given(this.forward_glyph_count as usize)
        },

        substitution_count (u16), // SubstCount

        substitutions (Vec<Substitution>) |this, tape| { // SubstLookupRecord
            tape.take_given(this.substitution_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of chain rules."]
    pub ChainRuleSet {
        count (u16), // ChainSubRuleCount

        offsets (Vec<u16>) |this, tape, _| { // ChainSubRule
            tape.take_given(this.count as usize)
        },

        records (Vec<ChainRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A class rule."]
    pub ClassRule {
        glyph_count        (u16), // GlyphCount
        substitution_count (u16), // SubstCount

        class_ids (Vec<u16>) |this, tape| { // Class
            if this.glyph_count == 0 {
                raise!("found a malformed class rule");
            }
            tape.take_given(this.glyph_count as usize - 1)
        },

        substitutions (Vec<Substitution>) |this, tape| { // SubstLookupRecord
            tape.take_given(this.substitution_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of class rules."]
    pub ClassRuleSet {
        count (u16), // SubClassRuleCnt

        offsets (Vec<u16>) |this, tape, _| { // SubClassRule
            tape.take_given(this.count as usize)
        },

        records (Vec<ClassRule>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
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
    #[doc = "A rule."]
    pub Rule {
        glyph_count        (u16), // GlyphCount
        substitution_count (u16), // SubstCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // Input
            if this.glyph_count == 0 {
                raise!("found a malformed rule");
            }
            tape.take_given(this.glyph_count as usize - 1)
        },

        substitutions (Vec<Substitution>) |this, tape| { // SubstLookupRecord
            tape.take_given(this.substitution_count as usize)
        },
    }
}

table! {
    @position
    #[doc = "A set of rules."]
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
    pub Substitution {
        sequence_index (u16), // SequenceIndex
        lookup_index   (u16), // LookupListIndex
    }
}
