use std::cell::{Cell, RefCell};
use std::mem::transmute;
use std::ops::Deref;
use std::sync::Arc;

use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, FillRect, HDC};
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW, GetWindowLongPtrW, PostQuitMessage, SetWindowLongPtrW, CREATESTRUCTW, GWLP_USERDATA, MSG, WM_CREATE, WM_DESTROY, WM_ERASEBKGND, WM_PAINT, WM_CLOSE, DestroyWindow, CallWindowProcW};

use crate::event::{Event, InputEvent};
use crate::window::WindowOptions;
use crate::style::Background;
use crate::windows::is_dark_mode;

#[derive(Default)]
struct Handler {
    handler: Option<Arc<dyn Fn(HWND, u32, WPARAM, LPARAM) -> bool + Sync + Send + 'static>>
}

impl Handler {
    pub fn set_handler<F: Fn(HWND, u32, WPARAM, LPARAM) -> bool + Sync + Send + 'static>(&mut self, handler: F) {
        self.handler = Some(Arc::new(handler));
    }

    pub fn handle(&self, hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> bool {
        if let Some(handler) = &self.handler {
            handler(hwnd, message, wparam, lparam)
        } else {
            false
        }
    }
}

thread_local! {
    static HANDLER: RefCell<Handler> = RefCell::new(Handler::default());
}

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

pub fn run<R: IntoEventResult, F: (Fn(isize, Event) -> R) + 'static + Sync + Send>(callback: F) {
    let mut message = MSG::default();
    HANDLER.with(|handler| {
        handler.borrow_mut().set_handler(
            move |hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM| {
                match message {
                    _ if InputEvent::message(message) => {
                        callback(
                            hwnd.0,
                            Event::Input(InputEvent::from((
                                message,
                                wparam,
                                lparam,
                            ))),
                        );
                    }
                    WM_CLOSE => {
                        let result = callback(hwnd.0, Event::Close).into_event_result();
                        if result {
                            unsafe { DestroyWindow(hwnd) };
                        }
                    }
                    WM_PAINT => {
                        callback(hwnd.0, Event::Repaint);
                    }
                    _ => { return false }
                }
                true
            }
        );
    });

    while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
        unsafe { DispatchMessageW(&message) };
    }
}

pub extern "system" fn wnd_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    HANDLER.with(|handler| {
        let handler = handler.borrow();
        if !handler.handle(window, message, wparam, lparam) {
            match message {
                WM_CREATE => {
                    let create_struct: &CREATESTRUCTW = unsafe { transmute(lparam) };
                    unsafe { SetWindowLongPtrW(window, GWLP_USERDATA, create_struct.lpCreateParams as _) };
                    LRESULT(0)
                },
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
        } else {
            LRESULT(0)
        }
    })
}
