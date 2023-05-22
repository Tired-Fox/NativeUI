use windows::Win32::{
    Foundation::COLORREF,
    Graphics::Gdi::{CreateHatchBrush, CreateSolidBrush, HATCH_BRUSH_STYLE, HBRUSH},
};

pub struct Brush;

use style::color::Color;

impl Brush {
    pub fn solid(color: Color) -> HBRUSH {
        unsafe { CreateSolidBrush(COLORREF(color.into())) }
    }

    pub fn hatch(color: Color, pattern: HATCH_BRUSH_STYLE) -> HBRUSH {
        unsafe { CreateHatchBrush(pattern, COLORREF(color.into())) }
    }
}
