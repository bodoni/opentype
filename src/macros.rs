macro_rules! jump_take(
    (@unwrap $tape:ident, $position:ident, $offset:expr) => ({
        try!($tape.jump($position + $offset as u64));
        try!($tape.take())
    });
    (@unwrap $tape:ident, $position:ident, $count:expr, $offsets:expr) => (
        jump_take!(@unwrap $tape, $position, $count, i => $offsets[i])
    );
    (@unwrap $tape:ident, $position:ident, $count:expr, $i:ident => $iterator:expr) => ({
        let mut values = Vec::with_capacity($count as usize);
        for $i in 0..($count as usize) {
            try!($tape.jump($position + $iterator as u64));
            values.push(try!($tape.take()));
        }
        values
    });
    ($tape:ident, $position:ident, $offset:expr) => (
        Ok(jump_take!(@unwrap $tape, $position, $offset))
    );
    ($tape:ident, $position:ident, $count:expr, $offsets:expr) => (
        Ok(jump_take!(@unwrap $tape, $position, $count, i => $offsets[i]))
    );
    ($tape:ident, $position:ident, $count:expr, $i:ident => $iterator:expr) => (
        Ok(jump_take!(@unwrap $tape, $position, $count, $i => $iterator))
    );
);

macro_rules! jump_take_given(
    (@unwrap $tape:ident, $position:ident, $count:expr, $offsets:expr, $parameter:expr) => (
        jump_take_given!(@unwrap $tape, $position, $count, i => $offsets[i], $parameter)
    );
    (@unwrap $tape:ident, $position:ident, $count:expr, $i:ident => $iterator:expr,
     $parameter:expr) => ({
        let mut values = Vec::with_capacity($count as usize);
        for $i in 0..($count as usize) {
            try!($tape.jump($position + $iterator as u64));
            values.push(try!($tape.take_given($parameter)));
        }
        values
    });
    ($tape:ident, $position:ident, $count:expr, $offsets:expr, $parameter:expr) => (
        Ok(jump_take_given!(@unwrap $tape, $position, $count, i => $offsets[i], $parameter))
    );
);

macro_rules! jump_take_maybe(
    (@unwrap $tape:ident, $position:ident, $offset:expr) => (
        if $offset > 0 {
            try!($tape.jump($position + $offset as u64));
            Some(try!($tape.take()))
        } else {
            None
        }
    );
    (@unwrap $tape:ident, $position:ident, $count:expr, $i:ident => $iterator:expr) => ({
        let mut values = Vec::with_capacity($count as usize);
        for $i in 0..($count as usize) {
            if $iterator > 0 {
                try!($tape.jump($position + $iterator as u64));
                values.push(Some(try!($tape.take())));
            } else {
                values.push(None);
            }
        }
        values
    });
    ($tape:ident, $position:ident, $offset:expr) => (
        Ok(jump_take_maybe!(@unwrap $tape, $position, $offset))
    );
    ($tape:ident, $position:ident, $count:expr, $offsets:expr) => (
        Ok(jump_take_maybe!(@unwrap $tape, $position, $count, i => $offsets[i]))
    );
);

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! table {
    ($(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($kind:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $name { $($field ($($kind)+),)* } }
        table! {
            @implement
            pub $name { $($field ($($kind)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
    );
    (@position $(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($kind:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $name { $($field ($($kind)+),)* } }
        table! {
            @implement @position
            pub $name { $($field ($($kind)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
    );
    (@define $(#[$attribute:meta])* pub $name:ident { $($field:ident ($kind:ty),)* }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug)]
        pub struct $name { $(pub $field: $kind,)* }
    );
    (@implement pub $name:ident {
        $($field:ident ($($kind:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl $crate::Value for $name {
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let mut table: $name = unsafe { ::std::mem::zeroed() };
                $({
                    let value = table!(@read $name, table, tape [] [$($kind)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@implement @position pub $name:ident {
        $($field:ident ($($kind:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl $crate::Value for $name {
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let position = try!(tape.position());
                let mut table: $name = unsafe { ::std::mem::zeroed() };
                $({
                    let value = table!(@read $name, table, tape [position] [$($kind)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read $name:ident, $this:ident, $tape:ident [$($position:tt)*] [$kind:ty] []) => (
        try!($tape.take())
    );
    (@read $name:ident, $this:ident, $tape:ident [$($position:tt)*] [$kind:ty]
     [$value:block]) => ({
        let value = try!($tape.take());
        if value != $value {
            raise!("found a malformed or unsupported table");
        }
        value
    });
    (@read $name:ident, $this:ident, $tape:ident [] [$kind:ty] []
     |$this_:pat, $tape_:pat| $body:block) => ({
        #[inline(always)]
        fn read<T: $crate::Tape>($this_: &$name, $tape_: &mut T) -> $crate::Result<$kind> $body
        try!(read(&$this, $tape))
    });
    (@read $name:ident, $this:ident, $tape:ident [$position:ident] [$kind:ty] []
     |$this_:pat, $tape_:pat, $position_:pat| $body:block) => ({
        #[inline(always)]
        fn read<T: $crate::Tape>($this_: &$name, $tape_: &mut T, $position_: u64)
                                 -> $crate::Result<$kind> $body
        try!(read(&$this, $tape, $position))
    });
}
