use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{DrawTextW, GetDC, DT_CALCRECT},
};

use native_core::{Rect, Renderable};

use crate::core::to_Rect;

pub fn text_size(parent: HWND, text: String) -> Rect {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    let mut value: Vec<u16> = text.encode_utf16().collect();
    unsafe {
        DrawTextW(
            GetDC(parent),
            &mut value[..],
            &mut rect as *mut RECT,
            DT_CALCRECT,
        );
    }
    to_Rect(rect)
}

pub fn padding_rect(control: &impl Renderable, rect: &mut RECT) {
    // Top, right, bottom, left
    let ns = control.rect();
    let padding = control.get_styles().0.padding.calc(ns.width(), ns.height());

    rect.top += padding.0;
    rect.right -= padding.1;
    rect.bottom -= padding.2;
    rect.left += padding.3;
}
