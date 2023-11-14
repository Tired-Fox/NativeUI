use std::mem::transmute;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, DispatchMessageW, GetMessageW, MSG, PostQuitMessage, WM_DESTROY, WM_PAINT, WM_CREATE, CREATESTRUCTW, SetWindowLongPtrW, GWLP_USERDATA, GetWindowLongPtrW, TranslateMessage};
use crate::windows::window::{Handler, WindowOptions};
use ::windows::Win32::UI::WindowsAndMessaging::{WM_CHAR, WM_KEYDOWN, WM_KEYUP};
use crate::event::{Event, InputEvent, PaintEvent};
use crate::event::keyboard::KeyboardEvent;

pub fn run<F: Fn(Event) + 'static>(callback: F) {
    unsafe {
        let mut message = MSG::default();
        while GetMessageW(&mut message, None, 0, 0).into() {
            DispatchMessageW(&message);
            match message.message {
                _ if InputEvent::message(message.message) => {
                    callback(Event::Input{ id: message.hwnd.0, value: InputEvent::from((message.message, message.wParam.0, message.lParam.0)) });
                }
                WM_PAINT => {
                    callback(Event::Paint{ id: message.hwnd.0 });
                }
                _ => {}
            }
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
            },
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT::default()
            },
            _ => {
                let user_data = unsafe { GetWindowLongPtrW(window, GWLP_USERDATA)};
                let options = std::ptr::NonNull::<WindowOptions>::new(user_data as _);
                let mut result = options.map_or(false, |options| {
                    if options.as_ref().proc.is_some() {
                        (options.as_ref().proc.clone().unwrap())(window, message, wparam, lparam)
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
