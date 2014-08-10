macro_rules! reverse_u16(
    ($field:expr) => (
        $field =
            (($field >> 8) & 0x00FF) |
            (($field << 8) & 0xFF00);
    )
)

macro_rules! reverse_u32(
    ($field:expr) => (
        $field =
            (($field >> 24) & 0x000000FF) |
            (($field >>  8) & 0x0000FF00) |
            (($field <<  8) & 0x00FF0000) |
            (($field << 24) & 0xFF000000);
    )
)

macro_rules! implement_endian(
    ($subject:ty, $this:ident, $body:expr) => (
        impl Endian for $subject {
            #[cfg(target_endian="little")]
            fn with_big_endian(&mut self) -> &mut $subject {
                fn convert($this: &mut $subject) { $body }
                convert(self);
                self
            }

            #[cfg(target_endian="big")]
            fn with_little_endian(&mut self) -> &mut $subject {
                fn convert($this: &mut $subject) { $body }
                convert(self);
                self
            }
        }
    )
)

pub trait Endian {
    #[cfg(target_endian="little")]
    fn with_big_endian(&mut self) -> &mut Self;

    #[cfg(target_endian="big")]
    fn with_little_endian(&mut self) -> &mut Self;

    #[cfg(target_endian="little")]
    fn with_little_endian(&mut self) -> &mut Self { self }

    #[cfg(target_endian="big")]
    fn with_big_endian(&mut self) -> &mut Self { self }
}

pub static CFF_TAG: u32 = 0x4F54544F;

pub struct OffsetTable {
    pub tag: u32,
    pub table_count: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

implement_endian!(OffsetTable, this, {
    reverse_u32!(this.tag);
    reverse_u16!(this.table_count);
    reverse_u16!(this.search_range);
    reverse_u16!(this.entry_selector);
    reverse_u16!(this.range_shift);
})
