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
