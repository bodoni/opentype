//! The [common layout tables][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/chapter2.htm

pub mod feature;
pub mod lookup;
pub mod script;

mod class;
mod correction;
mod coverage;
mod directory;

pub use class::{Class, Class1, Class2, ClassRange};
pub use correction::{Correction, Device, Variation};
pub use coverage::{Coverage, Coverage1, Coverage2, CoverageRange};
pub use directory::Directory;
pub use feature::Features;
pub use lookup::Lookups;
pub use script::Scripts;
