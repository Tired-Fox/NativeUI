use std::mem::transmute;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, DispatchMessageW, GetMessageW, MSG, PostQuitMessage, WM_DESTROY, WM_PAINT, WM_CREATE, CREATESTRUCTW, SetWindowLongPtrW, GWLP_USERDATA, GetWindowLongPtrW};
use crate::windows::window::{Handler, WindowOptions};

pub fn run() {
    unsafe {
        let mut message = MSG::default();
        while GetMessageW(&mut message, None, 0, 0).into() {
            DispatchMessageW(&message);
        }
    }
}

pub extern "system" fn wnd_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => unsafe {
                let create_struct: &CREATESTRUCTW = transmute(lparam);
                SetWindowLongPtrW(window, GWLP_USERDATA, create_struct.lpCreateParams as _);
                LRESULT::default()
            }
            _ => {
                let user_data = unsafe { GetWindowLongPtrW(window, GWLP_USERDATA)};
                let options = std::ptr::NonNull::<WindowOptions>::new(user_data as _);
                let mut result = options.map_or(false, |options| {
                    if options.as_ref().proc.is_some() {
                        options.as_ref().proc.clone().unwrap()(window, message, wparam, lparam)
                    } else {
                        false
                    }
                });
                if !result {
                    result = DefWindowProcW(window, message, wparam, lparam).0 == 0;
                }
                LRESULT(!result as _)
            },
        }
    }
}
