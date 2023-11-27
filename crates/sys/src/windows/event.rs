use std::mem::transmute;

use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, FillRect, HDC};
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW, GetWindowLongPtrW, PostQuitMessage, SetWindowLongPtrW, CREATESTRUCTW, GWLP_USERDATA, MSG, WM_CREATE, WM_DESTROY, WM_ERASEBKGND, WM_PAINT, WM_CLOSE, DestroyWindow, CallWindowProcW};

use crate::event::{Event, InputEvent};
use crate::window::WindowOptions;
use crate::style::Background;
use crate::windows::is_dark_mode;

pub trait IntoEventResult {
    fn into_event_result(self) -> bool;
}

impl IntoEventResult for () {
    fn into_event_result(self) -> bool {
        true
    }
}

impl IntoEventResult for bool {
    fn into_event_result(self) -> bool {
        self
    }
}

pub fn run<R: IntoEventResult, F: Fn(isize, Event) -> R + 'static>(callback: F) {
    let mut message = MSG::default();
    while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
        println!("Message Close [{}:{}]: {}", message.message, WM_CLOSE, message.message == WM_CLOSE);
        match message.message {
            _ if InputEvent::message(message.message) => {
                callback(
                    message.hwnd.0,
                    Event::Input(InputEvent::from((
                        message.message,
                        message.wParam,
                        message.lParam,
                    ))),
                );
            }
            WM_CLOSE => {
                println!("CLOSE");
                let result = callback(message.hwnd.0, Event::Close).into_event_result();
                println!("Close: {}", result);
                if result {
                    unsafe { DestroyWindow(message.hwnd) };
                }
            }
            WM_PAINT => {
                callback(message.hwnd.0, Event::Repaint);
            }
            _ => unsafe {
                DispatchMessageW(&message);
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
    match message {
        WM_CREATE => {
            let create_struct: &CREATESTRUCTW = unsafe { transmute(lparam) };
            unsafe { SetWindowLongPtrW(window, GWLP_USERDATA, create_struct.lpCreateParams as _) };
            LRESULT(0)
        },
        WM_CLOSE => {
            // TODO: Get event loop to access wnd_proc only events
            unsafe { DestroyWindow(window) };
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

            let brush = unsafe { CreateSolidBrush(COLORREF(background.color(is_dark_mode().into()))) };
            unsafe { FillRect(HDC(wparam.0 as isize), &rect, brush) };
            LRESULT(0)
        },
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}
