use postscript::compact::FontSet;
use truetype::{CharMapping, FontHeader, HorizontalHeader, HorizontalMetrics};
use truetype::{OffsetTable, MaximumProfile, NamingTable, PostScriptInfo, WindowsMetrics};

/// A font.
#[derive(Default)]
pub struct Font {
    pub offset_table: OffsetTable,
    pub char_mapping: Option<CharMapping>,
    pub font_header: Option<FontHeader>,
    pub horizontal_header: Option<HorizontalHeader>,
    pub horizontal_metrics: Option<HorizontalMetrics>,
    pub maximum_profile: Option<MaximumProfile>,
    pub naming_table: Option<NamingTable>,
    pub postscript_info: Option<PostScriptInfo>,
    pub windows_metrics: Option<WindowsMetrics>,
    pub postscript_fontset: Option<FontSet>,
}
