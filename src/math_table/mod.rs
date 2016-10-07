//! The [glyph-definition table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/GDEF.htm

pub mod constants;

use self::constants::Constants;

table! {
    @position
    #[doc = "The math table."] 
    pub Math {
        header(MathHeader),

        constants (Constants) |this, tape, positition| {
            jump_take!(tape, positition, this.header.constants_offset)
        },
    }
}

table! {
    #[doc = "A math header table."]
    #[derive(Copy)]
    pub MathHeader {
        major_version     (u16) = { 1 },  // MajorVersion
        minor_version     (u16) = { 0 },  // MinorVersion
        constants_offset  (u16), // ConstantsTable offset
        glyph_info_offset (u16), // GlyphInfo offset
        variants_offset   (u16), // GlyphInfo offset
    }
}