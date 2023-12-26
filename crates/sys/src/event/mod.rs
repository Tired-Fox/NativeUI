use std::{cell::RefCell, sync::{Arc, RwLockWriteGuard}};

use keyboard::KeyboardEvent;
use mouse::MouseEvent;
use crate::event::mouse::MouseEventType;

pub mod keyboard;
pub mod mouse;

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

#[derive(Debug, Clone)]
pub struct PaintEvent {
    pub handle: isize,
}

#[derive(Debug, Clone)]
pub enum Event {
    Close,
    Repaint,
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
}

pub trait IntoEvent {
    fn into_event(self) -> Event;
}

pub fn close(id: isize) {
    #[cfg(target_os = "windows")]
    unsafe {
        use crate::windows::event::wnd_proc;
        use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
        use windows::Win32::UI::WindowsAndMessaging::{CallWindowProcW, WM_CLOSE};

        CallWindowProcW(Some(wnd_proc), HWND(id), WM_CLOSE, WPARAM(0), LPARAM(0));
    }
}

pub fn quit(code: i32) {
    #[cfg(target_os = "windows")]
    unsafe {
        ::windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
    }
    std::process::exit(code);
}

pub fn run<R, F>(callback: F) 
where
    R: IntoEventResult,
    F: Fn(isize, Event, ()) -> R + 'static + Sync + Send,
{
    #[cfg(target_os = "windows")]
    crate::windows::event::run((), callback);
}

pub fn run_with_state<R, F, T>(state: T, callback: F) 
where
    R: IntoEventResult,
    F: Fn(isize, Event, T) -> R + 'static + Sync + Send,
    T: Clone + Send + Sync + 'static
{
    #[cfg(target_os = "windows")]
    crate::windows::event::run(state, callback);
}
