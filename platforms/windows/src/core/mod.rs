pub mod constants;
pub mod error;
pub mod image;
pub mod scroll;

use std::fmt::Debug;

use native_core::Rect;

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
    Graphics::Gdi::{GetDC, GetTextMetricsW, TEXTMETRICW},
    UI::WindowsAndMessaging::{
        DefWindowProcW, GetWindowLongPtrW, SetWindowLongPtrW, CREATESTRUCTW, GWLP_USERDATA,
    },
};

use self::constants::WM;

#[inline(always)]
pub const fn loword(x: u32) -> u16 {
    (x & 0xFFFF) as u16
}

#[inline(always)]
pub const fn hiword(x: u32) -> u16 {
    ((x >> 16) & 0xFFFF) as u16
}

pub struct CharInfo {
    pub width: i32,
    pub upper: i32,
    pub height: i32,
}

impl CharInfo {
    pub fn new(handle: HWND) -> Self {
        let mut tm: TEXTMETRICW = TEXTMETRICW::default();
        unsafe {
            GetTextMetricsW(GetDC(handle), &mut tm as *mut TEXTMETRICW);
        }
        let width = tm.tmAveCharWidth;
        let upper = (width / 2)
            * match tm.tmPitchAndFamily.0 & 1 {
                1 => 3,
                _ => 2,
            };

        CharInfo {
            width,
            upper,
            height: tm.tmHeight + tm.tmExternalLeading,
        }
    }
}

#[derive(Default)]
pub enum ProcResult {
    #[default]
    Default,
    Success,
    Fail,
}

pub trait Proc {
    fn proc(&mut self, _hwnd: HWND, _msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> ProcResult {
        ProcResult::Default
    }
}

pub extern "system" fn wndproc<'a, T>(
    handle: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT
where
    T: Proc + Debug,
{
    unsafe {
        match message {
            WM::CREATE => {
                let cs = lparam.0 as *const CREATESTRUCTW;
                let this = (*cs).lpCreateParams as *mut T;
                
                SetWindowLongPtrW(handle, GWLP_USERDATA, this as _);
                LRESULT(0)
            }
            _ => {
                let this = GetWindowLongPtrW(handle, GWLP_USERDATA) as *mut T;

                if !this.is_null() {
                    match (*this).proc(handle, message, wparam, lparam) {
                        ProcResult::Success => LRESULT(0),
                        ProcResult::Fail => LRESULT(1),
                        ProcResult::Default => DefWindowProcW(handle, message, wparam, lparam),
                    }
                } else {
                    DefWindowProcW(handle, message, wparam, lparam)
                }
            }
        }
    }
}

pub fn to_RECT(rect: Rect) -> RECT {
    RECT {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
    }
}

pub fn to_Rect(rect: RECT) -> Rect {
    Rect {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
    }
}
