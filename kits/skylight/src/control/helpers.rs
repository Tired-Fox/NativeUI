use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{DrawTextW, GetDC, DT_CALCRECT}, UI::WindowsAndMessaging::{GetWindowLongPtrW, GWLP_USERDATA},
};

use crate::{core::Rect, Window};

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

pub fn get_window<'a>(window: HWND) -> Result<&'a Window, String> {
    unsafe {
        let this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Window;
        if !this.is_null() {
            Ok(&*this)
        } else {
            Err("No window assigned to handle".to_owned())
        }
    }
}
