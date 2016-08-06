//! The [layout][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/chapter2.htm

use truetype::GlyphID;

mod class;
mod coverage;
mod directory;

pub mod feature;
pub mod lookup;
pub mod script;

pub use self::class::{Class, Class1, Class2};
pub use self::coverage::{Coverage, Coverage1, Coverage2};
pub use self::directory::{Directory, Header};
pub use self::feature::Features;
pub use self::lookup::Lookups;
pub use self::script::Scripts;

table! {
    #[doc = "A glyph range."]
    #[derive(Copy)]
    pub Range { // RangeRecord or ClassRangeRecord
        start (GlyphID), // Start
        end   (GlyphID), // End
        index (u16    ), // StartCoverageIndex or Class
    }
}
