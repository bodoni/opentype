//! The [layout][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/chapter2.htm

pub mod feature;
pub mod lookup;
pub mod script;

pub use self::feature::Features;
pub use self::lookup::Lookups;
pub use self::script::Scripts;
