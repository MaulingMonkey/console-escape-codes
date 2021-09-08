#[cfg(doc)] use crate::vt100::{self, *};



/// A 24-bit red/green/blue color tuple struct.  See [vt100::sgr_foreground_rgb] | [sgr_background_rgb] | [set_screen_color]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {
    pub const fn new(rgb: u32) -> Self { Self((rgb >> 16) as u8, (rgb >> 8) as u8, (rgb >> 0) as u8) }
    pub fn to_rgb(self) -> u32 { (self.0 as u32) << 16 | (self.1 as u32) << 8 | (self.2 as u32) << 0 }
}
