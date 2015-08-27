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
        impl ::tape::Value for $structure {
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(
                    table.$field = read_field!($structure, tape, table, [$($kind)+]
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
    ($structure:ident, $tape:ident, $table:ident, [$kind:ty]
                                                  |$pipe:ident, $chair:ident| $body:block) => ({
        #[inline(always)]
        #[allow(unused_variables)]
        fn read<T: ::tape::Tape>($pipe: &mut T, $chair: &$structure) -> ::Result<$kind> $body
        try!(read($tape, &$table))
    });
    ($structure:ident, $tape:expr, $table:expr, [$kind:ty]) => ({
        try!(::tape::Value::read($tape))
    });
);

macro_rules! read_vector(
    ($tape:ident, $count:expr, Byte) => (unsafe {
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        values.set_len(count);
        if try!(::std::io::Read::read($tape, &mut values)) != count {
            return raise!("failed to read as much as needed");
        }
        Ok(values)
    });
    ($tape:ident, $count:expr, Char) => (unsafe {
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        values.set_len(count);
        if try!(::std::io::Read::read($tape, &mut values)) != count {
            return raise!("failed to read as much as needed");
        }
        Ok(::std::mem::transmute(values))
    });
    ($tape:ident, $count:expr) => ({
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(try!(::tape::Value::read($tape)));
        }
        Ok(values)
    });
);

mod horizontal_header;
mod horizontal_metrics;
mod naming_table;
mod postscript;
mod windows_metrics;

pub use self::horizontal_header::HorizontalHeader;
pub use self::horizontal_metrics::{HorizontalMetrics, LongHorizontalMetric};
pub use self::naming_table::{NamingTable, NamingTable0, NamingTable1};
pub use self::naming_table::{NameRecord, LanguageTagRecord};
pub use self::postscript::{PostScript, PostScript10, PostScript30};
pub use self::windows_metrics::{WindowsMetrics, WindowsMetrics3, WindowsMetrics5};
