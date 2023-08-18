//! The [common layout tables][1].
//!
//! [1]: https://learn.microsoft.com/en-gb/typography/opentype/spec/chapter2

pub mod context;
pub mod coverage;
pub mod feature;
pub mod language;
pub mod lookup;
pub mod script;

mod class;
mod correction;
mod directory;

pub use class::{Class, Class1, Class2, ClassRange};
pub use context::{ChainedContext, Context};
pub use correction::{Correction, Device, Variation};
pub use coverage::Coverage;
pub use directory::Directory;
pub use feature::{Feature, Features};
pub use language::Language;
pub use lookup::Lookups;
pub use script::{Script, Scripts};
