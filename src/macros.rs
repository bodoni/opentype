macro_rules! itemize(($($chunk:item)*) => ($($chunk)*));

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! read_value(
    ($tape:expr) => (try!(::Value::read($tape)));
    ($tape:expr, $kind:ty) => (try!(<$kind as ::Value>::read($tape)));
);

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)+
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($kind),)+ } }
        table! { @implement pub $structure { $($field,)+ } }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)+
    }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
    (@implement pub $structure:ident {
        $($field:ident,)+
    }) => (
        impl ::Value for $structure {
            fn read<T: ::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table: $structure = unsafe { ::std::mem::uninitialized() };
                $(::std::mem::forget(::std::mem::replace(&mut table.$field, read_value!(tape)));)+
                Ok(table)
            }
        }
    );
}
