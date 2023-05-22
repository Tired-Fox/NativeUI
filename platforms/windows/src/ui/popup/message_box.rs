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

/// 0 == fail
/// 1 == success
/// 2 == dismiss
pub fn is_success(result: MessageReturn) -> u8 {
    match result {
        MessageReturn::Ok => 1,
        MessageReturn::Yes => 1,
        MessageReturn::Continue => 1,
        MessageReturn::No => 0,
        MessageReturn::Cancel => 2,
        MessageReturn::Abort => 0,
        MessageReturn::Ignore => 2,
        MessageReturn::Retry => 1,
        MessageReturn::TryAgain => 1,
        MessageReturn::Help => 1,
        MessageReturn::Close => 2,
        MessageReturn::Async => 1,
        MessageReturn::Timeout => 2,
    }
}
