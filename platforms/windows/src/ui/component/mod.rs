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
mod scroll_bar;

pub use text::Text;
pub use scroll_bar::ScrollBar;

#[derive(Default)]
pub enum ProcResult {
    #[default]
    Default,
    Success,
    Fail,
}

pub trait Proc {
    fn proc(&mut self, hwnd: HWND, msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> ProcResult {
        ProcResult::Default
    }
}

pub extern "system" fn wndproc<'a, T>(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT
where
    T: Proc,
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
