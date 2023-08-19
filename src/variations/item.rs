//! The item variation store.

table! {
    @position
    #[doc = "An item variation store."]
    pub Store { // ItemVariationStore
        format        (u16) = { 1 }, // format
        region_offset (u32), // offsetToVariationRegionList
        count         (u16), // itemVariationDataCount

        offsets (Vec<u32>) |this, tape, _| { // itemVariationDataOffsets
            tape.take_given(this.count as usize)
        },

        records (Vec<Record>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A record of an item variation store."]
    pub Record { // ItemVariationData
        item_count        (u16), // itemCount
        short_delta_count (u16), // shortDeltaCount
        region_count      (u16), // regionCount

        region_indices (Vec<u16>) |this, tape| { // regionIndices
            tape.take_given(this.region_count as usize)
        },

        deltas (Vec<u8>) |this, tape| { // deltaSets
            let per_item_count = this.short_delta_count + this.region_count;
            tape.take_given(this.item_count as usize * per_item_count as usize)
        },
    }
}
