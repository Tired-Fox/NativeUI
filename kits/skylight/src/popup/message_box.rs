use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::*,
    },
};

use super::types::{ButtonLayout, Icon, MessageReturn};

pub fn new(
    handle: HWND,
    title: &'static str,
    body: &'static str,
    button_layout: ButtonLayout,
    icon: Icon,
) -> MessageReturn {
    let mut style: MESSAGEBOX_STYLE = button_layout.into();

    if icon != Icon::None {
        style |= icon.into();
    }

    unsafe {
        MessageBoxW(
            handle,
            PCWSTR::from_raw(HSTRING::from(body).as_ptr()),
            PCWSTR::from_raw(HSTRING::from(title).as_ptr()),
            style,
        ).into()
    }
}
