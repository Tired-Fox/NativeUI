use windows::{
    core::{IntoParam, HSTRING, PCWSTR},
    Win32::{Foundation::HWND, UI::WindowsAndMessaging::*},
};

use super::types::{ButtonLayout, Icon, MessageReturn};

pub fn new<'a, P0>(
    handle: P0,
    title: &'a str,
    body: &'a str,
    button_layout: ButtonLayout,
    icon: Icon,
) -> MessageReturn
where
    P0: IntoParam<HWND>,
{
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
        )
        .into()
    }
}
