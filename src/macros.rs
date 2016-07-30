macro_rules! itemize(($($chunk:item)*) => ($($chunk)*));

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! read_field(
    ($structure:ident, $tape:ident, $table:ident,
     [$kind:ty] |$band:ident, $chair:ident| $body:block) => ({
        #[allow(unused_variables)]
        #[inline(always)]
        fn read<T: ::Tape>($band: &mut T, $chair: &$structure) -> ::Result<$kind> $body
        try!(read($tape, &$table))
    });
    ($structure:ident, $tape:ident, $table:expr, [$kind:ty]) => (read_value!($tape));
);

macro_rules! read_value(
    ($tape:expr) => (try!(::Value::read($tape)));
    ($tape:expr, $kind:ty) => (try!(<$kind as ::Value>::read($tape)));
);

macro_rules! read_walue(
    ($tape:expr, $parameter:expr) => (try!(::Walue::read($tape, $parameter)));
    ($tape:expr, $parameter:expr, $kind:ty) => ({
        try!(<$kind as ::Walue<_>>::read($tape, $parameter))
    });
);

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($($kind)+),)* } }
        table! { @implement pub $structure { $($field ($($kind)+) $(|$($argument),+| $body)*,)* } }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)*
    }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)* }
    });
    (@implement pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)*
    }) => (
        impl ::Value for $structure {
            fn read<T: ::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table: $structure = unsafe { ::std::mem::uninitialized() };
                $({
                    let value = read_field!($structure, tape, table,
                                            [$($kind)+] $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
}
