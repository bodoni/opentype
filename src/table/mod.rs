//! Tables making up a font file.

#![allow(non_camel_case_types, non_snake_case)]

macro_rules! spec {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)+
    }) => (
        declare! {
            $(#[$attribute])* pub $structure {
                $($field ($($kind)+),)+
            }
        }
        implement! {
            pub $structure {
                $($field ($($kind)+) $(|$($argument),+| $body)*,)+
            }
        }
    );
}

macro_rules! declare {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)+
    }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure {
            $(pub $field: $kind,)+
        }
    });
}

macro_rules! implement {
    (pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)+
    }) => (
        impl ::band::Value for $structure {
            fn read<T: ::band::Band>(band: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(
                    table.$field = read_field!($structure, band, table, [$($kind)+]
                                               $(|$($argument),+| $body)*);
                )+
                Ok(table)
            }
        }
    );
}

macro_rules! itemize(
    ($code:item) => ($code);
);

macro_rules! read_field(
    ($structure:ident, $band:ident, $table:ident, [$kind:ty]
                                                  |$gang:ident, $chair:ident| $body:block) => ({
        #[inline(always)]
        #[allow(unused_variables)]
        fn read<T: ::band::Band>($gang: &mut T, $chair: &$structure) -> ::Result<$kind> $body
        try!(read($band, &$table))
    });
    ($structure:ident, $band:expr, $table:expr, [$kind:ty]) => ({
        try!(::band::Value::read($band))
    });
);

macro_rules! read_vector(
    ($band:ident, $count:expr) => ({
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(try!(::band::Value::read($band)));
        }
        Ok(values)
    });
);

mod char_mapping;
mod font_header;
mod horizontal_header;
mod horizontal_metrics;
mod maximum_profile;
mod naming_table;
mod offset_table;
mod postscript;
mod windows_metrics;

pub use self::char_mapping::{CharMapping, CharMappingHeader, CharMappingRecord};
pub use self::char_mapping::{CharMappingEncoding, CharMappingEncoding4, CharMappingEncoding6};
pub use self::font_header::FontHeader;
pub use self::horizontal_header::HorizontalHeader;
pub use self::horizontal_metrics::HorizontalMetrics;
pub use self::maximum_profile::{MaximumProfile, MaximumProfile05, MaximumProfile10};
pub use self::naming_table::{NamingTable, NamingTable0, NamingTable1};
pub use self::naming_table::{NameRecord, LanguageTagRecord};
pub use self::offset_table::{OffsetTable, OffsetTableHeader, OffsetTableRecord};
pub use self::postscript::{PostScript, PostScript10, PostScript30};
pub use self::windows_metrics::{WindowsMetrics, WindowsMetrics3, WindowsMetrics5};
