mod button;
mod helpers;
mod text;

use std::fmt;

pub use button::Button;
use style::Stylesheet;
pub use text::Text;
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        DefWindowProcW, GetWindowLongPtrW, SetWindowLongPtrW, CREATESTRUCTA, GWLP_USERDATA,
        WM_CREATE,
    },
};

use crate::core::{ViewType, ProcResult, Renderable, Rect};

pub trait Control: fmt::Debug + Renderable {
    fn ns_rect(&self) -> &Rect;
    fn classes(&mut self, classes: Vec<&'static str>);
    fn create(&mut self, parent: ViewType, stylesheet: &Stylesheet) -> Result<(), String>;
    fn proc(&mut self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> ProcResult;
}

pub extern "system" fn wndproc<'a, T>(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT
where
    T: Control,
{
    unsafe {
        match message {
            WM_CREATE => {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut T;
                // (*this).handle = window;

                SetWindowLongPtrW(window, GWLP_USERDATA, this as _);
                // (*this).on_create().ok();
                return LRESULT(0);
            }
            _ => {
                let this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut T;

                if !this.is_null() {
                    return match (*this).proc(window, message, wparam, lparam) {
                        ProcResult::Success => LRESULT(0),
                        ProcResult::Fail => LRESULT(1),
                        ProcResult::Default => DefWindowProcW(window, message, wparam, lparam),
                    };
                } else {
                    DefWindowProcW(window, message, wparam, lparam)
                }
            }
        }
    }
}
