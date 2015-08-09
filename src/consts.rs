//! Constants.

use types::*;

pub const CFF_FORMAT_TAG: &'static [u8; 4] = b"OTTO";

pub const CHAR_MAP_TAG: &'static [u8; 4] = b"cmap";
pub const CHAR_MAP_HEADER_VERSION_0_0: USHORT = 0;

pub const FONT_HEADER_TAG: &'static [u8; 4] = b"head";
pub const FONT_HEADER_VERSION_1_0: Fixed = Fixed(0x00010000);
pub const FONT_HEADER_MAGIC_NUMBER: ULONG = 0x5F0F3CF5;

pub const MAX_PROFILE_TAG: &'static [u8; 4] = b"maxp";
pub const MAX_PROFILE_VERSION_0_5: Fixed = Fixed(0x00005000);
