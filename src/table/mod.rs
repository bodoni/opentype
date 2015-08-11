//! Tables making up a font file.

#![allow(non_camel_case_types, non_snake_case)]

macro_rules! table(
    ($structure:ident { $($field:ident ($($kind:tt)+) $(|$this:ident| $body:block)*,)+ }) => (
        declare!($structure { $($field ($($kind)+),)+ });
        implement!($structure { $($field ($($kind)+) $(|$this| $body)*,)+ });
    );
);

macro_rules! declare(
    ($structure:ident { $($field:ident ($kind:ty),)+ }) => (
        itemize! {
            #[derive(Clone, Debug, Default, Eq, PartialEq)]
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
        #[allow(unused_variables)]
        fn count($that: &$structure) -> ::Result<usize> $body
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

mod char_mapping;
mod font_header;
mod horizontal_header;
mod horizontal_metrics;
mod maximum_profile;
mod offset_table;
mod windows_metrics;

pub use self::char_mapping::{CharMapping, CharMappingHeader, CharMappingRecord};
pub use self::char_mapping::{CharMappingEncoding, CharMappingEncoding4, CharMappingEncoding6};
pub use self::font_header::FontHeader;
pub use self::horizontal_header::HorizontalHeader;
pub use self::horizontal_metrics::HorizontalMetrics;
pub use self::maximum_profile::{MaximumProfile, MaximumProfile05, MaximumProfile10};
pub use self::offset_table::{OffsetTable, OffsetTableHeader, OffsetTableRecord};
pub use self::windows_metrics::{WindowsMetrics, WindowsMetrics3, WindowsMetrics5};
