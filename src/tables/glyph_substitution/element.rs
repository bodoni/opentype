use truetype::GlyphID;

table! {
    #[doc = "Alternates."]
    pub Alternates { // AlternateSet
        glyph_count (u16), // glyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // alternateGlyphIDs
            tape.take_given(this.glyph_count as usize)
        },
    }
}

table! {
    #[doc = "A ligature."]
    pub Ligature { // Ligature
        glyph_id    (GlyphID), // ligatureGlyph
        glyph_count (u16    ), // componentCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // componentGlyphIDs
            if this.glyph_count == 0 {
                raise!("found a malformed ligature substitution");
            }
            tape.take_given(this.glyph_count as usize - 1)
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
        glyph_count (u16), // glyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // substituteGlyphIDs
            tape.take_given(this.glyph_count as usize)
        },
    }
}
