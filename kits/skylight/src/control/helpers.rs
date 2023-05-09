use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{DrawTextW, GetDC, DT_CALCRECT}, UI::WindowsAndMessaging::{SET_WINDOW_POS_FLAGS, SetWindowPos},
};

use crate::core::Rect;

use super::Control;

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
    rect.into()
}

pub fn update_pos(control: &mut impl Control) {
    let rect = control.rect();
    let handle = control.handle();
    unsafe {
        SetWindowPos(
            *handle,
            None,
            rect.left,
            rect.top,
            rect.width(),
            rect.height(),
            SET_WINDOW_POS_FLAGS::default(),
        );
    }
}

pub fn padding_rect(control: &impl Control, rect: &mut RECT) {
    // Top, right, bottom, left
    let ns = control.ns_rect();
    let padding = control.style().0.padding.calc(ns.width(), ns.height());

    rect.top += padding.0;
    rect.right -= padding.1;
    rect.bottom -= padding.2;
    rect.left += padding.3;
}
