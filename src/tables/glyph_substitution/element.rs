use truetype::GlyphID;

table! {
    #[doc = "Alternates."]
    pub Alternates { // AlternateSet
        count (u16), // glyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // alternateGlyphIDs
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A ligature."]
    pub Ligature { // Ligature
        glyph_id        (GlyphID), // ligatureGlyph
        component_count (u16    ), // componentCount

        component_ids (Vec<GlyphID>) |this, tape| { // componentGlyphIDs
            if this.component_count == 0 {
                raise!("found a malformed ligature substitution");
            }
            tape.take_given(this.component_count as usize - 1)
        },
    }
}

table! {
    @position
    #[doc = "Ligatures."]
    pub Ligatures { // LigatureSet
        count (u16), // ligatureCount

        offsets (Vec<u16>) |this, tape, _| { // ligatureOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<Ligature>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A sequence."]
    pub Sequence { // Sequence
        count (u16), // glyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // substituteGlyphIDs
            tape.take_given(this.count as usize)
        },
    }
}
