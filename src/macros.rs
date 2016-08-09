macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($($kind)+),)* } }
        table! { @implement pub $structure { $($field ($($kind)+) $(|$($argument),+| $body)*,)* } }
    );
    (@position $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($($kind)+),)* } }
        table! {
            @implement @position
            pub $structure { $($field ($($kind)+) $(|$($argument),+| $body)*,)* }
        }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)* }
    );
    (@implement pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl $crate::Value for $structure {
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let mut table: $structure = unsafe { ::std::mem::uninitialized() };
                $({
                    let value = table!(@read $structure, tape, table, [], [$($kind)+]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@implement @position pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl $crate::Value for $structure {
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let position = try!(tape.position());
                let mut table: $structure = unsafe { ::std::mem::uninitialized() };
                $({
                    let value = table!(@read $structure, tape, table, [position], [$($kind)+]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read $structure:ident, $tape:ident, $table:ident, [], [$kind:ty]
     |$chair:tt, $band:tt| $body:block) => ({
        #[inline(always)]
        fn read<T: $crate::Tape>($chair: &$structure, $band: &mut T) -> $crate::Result<$kind> $body
        try!(read(&$table, $tape))
    });
    (@read $structure:ident, $tape:ident, $table:ident, [$position:ident], [$kind:ty]
     |$chair:tt, $band:tt, $location:tt| $body:block) => ({
        #[inline(always)]
        fn read<T: $crate::Tape>($chair: &$structure, $band: &mut T, $location: u64)
                                 -> $crate::Result<$kind> $body
        try!(read(&$table, $tape, $position))
    });
    (@read $structure:ident, $tape:ident, $table:expr, [$($position:tt)*], [$kind:ty]) => (
        try!($tape.take())
    );
}
