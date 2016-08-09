macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

#[doc(hidden)]
#[macro_export]
macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($($kind)+),)* } }
        table! { @implement pub $structure { $($field ($($kind)+) $(|$($argument),+| $body)*,)* } }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)* }
    );
    (@implement pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)*
    }) => (
        impl $crate::Value for $structure {
            #[allow(unused_variables)]
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let position = try!(tape.position());
                let mut table: $structure = unsafe { ::std::mem::uninitialized() };
                $({
                    let value = table!(@read $structure, tape, table, position, [$($kind)+]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read $structure:ident, $tape:ident, $table:ident, $position:ident, [$kind:ty]
     |$band:ident, $chair:ident, $location:ident| $body:block) => ({
        #[inline(always)]
        fn read<T: $crate::Tape>($band: &mut T, $chair: &$structure, $location: u64)
                                 -> $crate::Result<$kind> $body
        try!(read($tape, &$table, $position))
    });
    (@read $structure:ident, $tape:ident, $table:expr, $position:ident, [$kind:ty]) => (
        try!($tape.take())
    );
}
