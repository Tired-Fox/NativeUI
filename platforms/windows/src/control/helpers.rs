use windows::Win32::{
    Foundation::{HWND, RECT, BOOL},
    Graphics::Gdi::{DrawTextW, GetDC, InvalidateRect, DT_CALCRECT},
    UI::WindowsAndMessaging::{SetWindowPos, SET_WINDOW_POS_FLAGS},
};

use native_core::Rect;

use crate::core::{to_Rect, to_RECT};

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
    to_Rect(rect)
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

        let rect: RECT = to_RECT(control.rect().clone());
        InvalidateRect(*control.handle(), Some(&rect as *const RECT), true);
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
