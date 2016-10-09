table! {
    #[doc = "A table of varaint glyphs to satisfy alternate measurements requirements."]
    #[derive(Copy)]
    pub Variants { // MathVariants
        min_connector_overlap (i16),
        vert_coverage_offset  (u16), // VertGlyphCoverage
        horz_coverage_offset  (u16), // HorizGlyphCoverage
        vert_count            (u16), // VertGlyphCount
        horz_count            (u16), // HorizGlyphCount
    }
}