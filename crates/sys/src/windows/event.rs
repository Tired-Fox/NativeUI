use std::mem::transmute;

use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, FillRect, HDC};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW, GetWindowLongPtrW,
    PostQuitMessage, SetWindowLongPtrW, CREATESTRUCTW, GWLP_USERDATA, MSG, WM_CREATE, WM_DESTROY,
    WM_ERASEBKGND, WM_PAINT,
};

use crate::event::{Event, InputEvent};
use crate::windows::window::WindowOptions;
use crate::windows::Background;

pub fn run<F: Fn(isize, Event) + 'static>(callback: F) {
    let mut message = MSG::default();
    while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
        unsafe { DispatchMessageW(&message) };
        match message.message {
            _ if InputEvent::message(message.message) => {
                callback(
                    message.hwnd.0,
                    Event::Input(InputEvent::from((
                        message.message,
                        message.wParam.0,
                        message.lParam.0,
                    ))),
                );
            }
            WM_PAINT => {
                callback(message.hwnd.0, Event::Repaint);
            }
            _ => {}
        }
    }
}

pub extern "system" fn wnd_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_CREATE => {
            let create_struct: &CREATESTRUCTW = unsafe { transmute(lparam) };
            unsafe { SetWindowLongPtrW(window, GWLP_USERDATA, create_struct.lpCreateParams as _) };
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        WM_ERASEBKGND => {
            // Auto fill background with window theme color
            let user_data = unsafe { GetWindowLongPtrW(window, GWLP_USERDATA) };
            let sample = std::ptr::NonNull::<WindowOptions>::new(user_data as _);
            let background =
                sample.map_or(Background::default(), |s| unsafe { s.as_ref() }.background.clone());

            let mut rect = RECT::default();
            unsafe { GetClientRect(window, &mut rect).unwrap() };

            let brush = unsafe { CreateSolidBrush(COLORREF(background.color())) };
            unsafe { FillRect(HDC(wparam.0 as isize), &rect, brush) };
            LRESULT(0)
        },
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}
