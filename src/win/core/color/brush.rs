use windows::Win32::{
    Foundation::COLORREF,
    Graphics::Gdi::{CreateHatchBrush, CreateSolidBrush, HATCH_BRUSH_STYLE, HBRUSH},
};

use crate::win::core::style::hs;

pub struct Brush;

impl Brush {
    pub fn solid(color: u32) -> HBRUSH {
        unsafe { CreateSolidBrush(COLORREF(color)) }
    }

    pub fn hatch(color: u32, pattern: HATCH_BRUSH_STYLE) -> HBRUSH {
        unsafe { CreateHatchBrush(pattern, COLORREF(color)) }
    }
}
