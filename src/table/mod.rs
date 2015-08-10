//! Tables making up a font file.

#![allow(non_snake_case)]

macro_rules! table(
    ($structure:ident { $($field:ident ($($kind:tt)+) $(|$this:ident| $body:block)*,)+ }) => (
        declare!($structure { $($field $($kind)+,)+ });
        implement!($structure { $($field ($($kind)+) $(|$this| $body)*,)+ });
    );
);

macro_rules! declare(
    ($structure:ident { $($field:ident $kind:ty,)+ }) => (
        itemize! {
            #[derive(Debug, Default)]
            pub struct $structure { $(pub $field: $kind,)+ }
        }
    );
);

macro_rules! implement(
    ($structure:ident { $($field:ident ($($kind:tt)+) $(|$this:ident| $body:block)*,)+ }) => (
        impl ::band::Value for $structure {
            fn read<T: ::band::Band>(band: &mut T) -> ::Result<Self> {
                let mut value = $structure::default();
                $(value.$field = read!($structure, value, band, [$($kind)+] $(|$this| $body)*);)+
                Ok(value)
            }
        }
    );
);

macro_rules! itemize(
    ($code:item) => ($code);
);

macro_rules! read(
    ($structure:ident, $this:ident, $band:ident, [$kind:ty] |$that:ident| $body:block) => ({
        #[inline(always)]
        fn count($that: &$structure) -> Result<usize> $body
        let count = try!(count(&$this));
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(try!(::band::Value::read($band)));
        }
        values
    });
    ($structure:ident, $this:ident, $band:ident, [$kind:ty]) => ({
        try!(::band::Value::read($band))
    });
);

mod header;
mod mapping;
mod prelude;
mod profile;

pub use self::header::FontHeader;
pub use self::mapping::{CharMapping, CharMapping4, CharMapping6};
pub use self::mapping::{CharMappingHeader, EncodingRecord};
pub use self::prelude::{OffsetTable, TableRecord};
pub use self::profile::{MaxProfile, MaxProfile05, MaxProfile10};
