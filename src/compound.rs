use Result;
use band::Band;

pub trait Compound {
    fn read<T: Band>(&mut self, band: &mut T) -> Result<()>;
}

macro_rules! itemize(
    ($code:item) => ($code);
);

macro_rules! compound(
    ($structure:ident { $($field:ident [$($kind:tt)+] $(|$this:ident| $body:block)*,)+ }) => (
        declare_compound!($structure { $($field $($kind)+,)+ });
        implement_compound!($structure { $($field [$($kind)+] $(|$this| $body)*,)+ });
    );
);

macro_rules! declare_compound(
    ($structure:ident { $($field:ident $kind:ty,)+ }) => (
        itemize! {
            #[derive(Default)]
            pub struct $structure { $(pub $field: $kind,)+ }
        }
    );
);

macro_rules! implement_compound(
    ($structure:ident { $($field:ident [$($kind:tt)+] $(|$this:ident| $body:block)*,)+ }) => (
        impl ::compound::Compound for $structure {
            fn read<T: ::band::Band>(&mut self, band: &mut T) -> ::Result<()> {
                $(self.$field = read_field!($structure, self, band, $($kind)+ $(|$this| $body)*);)+
                Ok(())
            }
        }
    );
);

macro_rules! read_field(
    ($structure:ident, $this:ident, $band:ident, USHORT) => ({
        try!($band.read_u16())
    });
    ($structure:ident, $this:ident, $band:ident, SHORT) => ({
        try!($band.read_i16())
    });
    ($structure:ident, $this:ident, $band:ident, ULONG) => ({
        try!($band.read_u32())
    });
    ($structure:ident, $this:ident, $band:ident, Fixed) => ({
        Fixed(try!($band.read_u32()))
    });
    ($structure:ident, $this:ident, $band:ident, LONGDATETIME) => ({
        try!($band.read_i64())
    });
    ($structure:ident, $this:ident, $band:ident, Vec<USHORT> |$that:ident| $body:block) => ({
        #[allow(unused_variables)]
        fn count($that: &$structure) -> usize $body
        let _ = count($this);
        vec![]
    });
    ($structure:ident, $this:ident, $band:ident, Vec<SHORT> |$that:ident| $body:block) => ({
        #[allow(unused_variables)]
        fn count($that: &$structure) -> usize $body
        let _ = count($this);
        vec![]
    });
);
