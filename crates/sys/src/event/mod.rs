use keyboard::KeyboardEvent;
use mouse::MouseEvent;

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
pub enum InputEvent {
    // Bit 30 == 1 when repeating key / held down
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
}

impl InputEvent {
    pub fn message(m: u32) -> bool {
        keyboard::KeyboardEvent::message(m) || mouse::MouseEvent::message(m)
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
    Input(InputEvent),
}

impl Event {
    pub fn keyboard(&self) -> Option<&KeyboardEvent> {
        match self {
            Event::Input(InputEvent::Keyboard(ke)) => Some(ke),
            _ => None,
        }
    }

    pub fn input(&self) -> Option<&InputEvent> {
        match self {
            Event::Input(ie) => Some(ie),
            _ => None,
        }
    }

    pub fn mouse(&self) -> Option<&MouseEvent> {
        match self {
            Event::Input(InputEvent::Mouse(me)) => Some(me),
            _ => None,
        }
    }
}

pub trait IntoEvent {
    fn into_event(self) -> Event;
}

pub fn close(id: isize) {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
        use crate::windows::event::{wnd_proc};
        use windows::Win32::UI::WindowsAndMessaging::{CallWindowProcW, WM_CLOSE};

        CallWindowProcW(
            Some(wnd_proc),
            HWND(id),
            WM_CLOSE,
            WPARAM(0),
            LPARAM(0),
        );
    }
}

pub fn quit() {
    #[cfg(target_os = "windows")]
    unsafe {
        ::windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
    }
}

pub fn run<R: IntoEventResult, F: Fn(isize, Event) -> R + 'static + Sync + Send>(callback: F) {
    #[cfg(target_os = "windows")]
    crate::windows::event::run(callback);
}
