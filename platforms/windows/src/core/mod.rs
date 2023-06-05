pub mod constants;
pub mod error;
pub mod image;
pub mod scroll;

use std::fmt::Debug;

use native_core::Rect;

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
    Graphics::Gdi::{GetDC, GetTextMetricsW, TEXTMETRICW},
    UI::{
        Controls::STATE_SYSTEM_INVISIBLE,
        WindowsAndMessaging::{
            DefWindowProcW, GetWindowLongPtrW, GetWindowLongW, SetWindowLongPtrW, CREATESTRUCTW,
            GWLP_USERDATA, GWL_STYLE,
        },
    },
};

use crate::core::constants::WS;

use self::{
    constants::{MK, SB, WM},
    scroll::{hscroll, mouse_scroll, vscroll},
};

#[inline(always)]
pub const fn loword(x: u32) -> u32 {
    x & 0xFFFF
}

#[inline(always)]
pub const fn hiword(x: u32) -> u32 {
    (x >> 16) & 0xFFFF
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
    fn scroll(&mut self, hwnd: HWND, msg: u32, wparam: WPARAM) -> bool {
        let mut hs= false;
        let mut vs= false;
        unsafe {
            let style = GetWindowLongW(hwnd, GWL_STYLE);
            hs = (style & WS::HSCROLL.0 as i32) == WS::HSCROLL.0 as i32;
            vs = (style & WS::VSCROLL.0 as i32) == WS::VSCROLL.0 as i32;
        }
        match msg {
            WM::VSCROLL if vs => {
                vscroll(hwnd, wparam);
            }
            WM::HSCROLL if hs => {
                hscroll(hwnd, wparam);
            }
            WM::MOUSEHWHEEL if hs => {
                let distance = hiword(wparam.0 as u32);

                mouse_scroll(
                    hwnd,
                    SB::HORZ,
                    match distance {
                        _ if distance == 120 => 1,
                        _ => -1,
                    },
                );
            }
            WM::MOUSEWHEEL if vs || hs => {
                let distance = hiword(wparam.0 as u32);
                let modifier = MK::MODIFIERKEY(loword(wparam.0 as u32) as u32);

                if modifier == MK::SHIFT {
                    mouse_scroll(
                        hwnd,
                        SB::HORZ,
                        match distance {
                            _ if distance == 120 => -1,
                            _ => 1,
                        },
                    );
                } else {
                    mouse_scroll(
                        hwnd,
                        SB::VERT,
                        match distance {
                            _ if distance == 120 => -1,
                            _ => 1,
                        },
                    );
                }
            }
            _ => return false,
        }
        true        
    }

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
