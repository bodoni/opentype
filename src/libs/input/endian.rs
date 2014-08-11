#[inline = "always"]
pub fn reverse_u16(value: u16) -> u16 {
    ((value >> 8) & 0x00FF) |
    ((value << 8) & 0xFF00)
}

#[inline = "always"]
pub fn reverse_u32(value: u32) -> u32 {
    ((value >> 24) & 0x000000FF) |
    ((value >>  8) & 0x0000FF00) |
    ((value <<  8) & 0x00FF0000) |
    ((value << 24) & 0xFF000000)
}

pub trait Endian {
    #[cfg(target_endian = "little")]
    fn with_big_endian(self) -> Self;

    #[cfg(target_endian = "big")]
    fn with_little_endian(self) -> Self;

    #[inline = "always"]
    #[cfg(target_endian = "little")]
    fn with_little_endian(self) -> Self { self }

    #[inline = "always"]
    #[cfg(target_endian = "big")]
    fn with_big_endian(self) -> Self { self }
}

#[macro_export]
macro_rules! impl_endian(
    ($subject:ty, $($field:ident as $size:ident),*) => (
        impl Endian for $subject {
            #[cfg(target_endian = "little")]
            fn with_big_endian(mut self) -> $subject {
                $(
                    self.$field = concat_idents!(reverse_, $size)(self.$field);
                )*
                self
            }

            #[cfg(target_endian = "big")]
            fn with_little_endian(mut self) -> $subject {
                $(
                    self.$field = concat_idents!(reverse_, $size)(self.$field);
                )*
                self
            }
        }
    )
)
