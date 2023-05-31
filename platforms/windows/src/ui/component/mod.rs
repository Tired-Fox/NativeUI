use native_core::Component;
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        DefWindowProcW, GetWindowLongPtrW, SetWindowLongPtrW, CREATESTRUCTA, GWLP_USERDATA,
        WM_CREATE,
    },
};

mod helpers;
mod text;

pub use text::Text;
