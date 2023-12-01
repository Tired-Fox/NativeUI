use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP,
    WM_XBUTTONDBLCLK, WM_XBUTTONDOWN, WM_XBUTTONUP,
    WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_MOUSEHWHEEL, WM_CAPTURECHANGED,
};
use windows::Win32::UI::Controls::{WM_MOUSEHOVER, WM_MOUSELEAVE};
use crate::windows::{get_wheel_delta_wparam, hiword, loword};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton {
    Control = 0x08,
    Shift = 0x04,
    Left = 0x01,
    Right = 0x02,
    Middle = 0x10,
    X1 = 0x20,
    X2 = 0x40,
}

impl MouseButton {
    pub fn buttons_down(v: usize) -> Vec<Self> {
        let mut buttons = Vec::new();
        if v & 0x01 == 0x01 {
            buttons.push(MouseButton::Left);
        }
        if v & 0x02 == 0x02 {
            buttons.push(MouseButton::Right);
        }
        if v & 0x10 == 0x10 {
            buttons.push(MouseButton::Middle);
        }
        if v & 0x20 == 0x20 {
            buttons.push(MouseButton::X1);
        }
        if v & 0x40 == 0x40 {
            buttons.push(MouseButton::X2);
        }
        if v & 0x08 == 0x08 {
            buttons.push(MouseButton::Control);
        }
        if v & 0x04 == 0x04 {
            buttons.push(MouseButton::Shift);
        }
        buttons
    }

    pub fn xbutton(v: usize) -> Self {
        if (v >> 16) == 1 {
            MouseButton::X1
        } else {
            MouseButton::X2
        }
    }
}

#[derive(Debug, Clone)]
pub enum MouseEventType {
    Ignore,
    Down(MouseButton),
    Up(MouseButton),
    Double(MouseButton),
    Move,
    Hover,
    Scroll(i16),
    HScroll(i16),
}

impl From<(u32, usize)> for MouseEventType {
    fn from(value: (u32, usize)) -> Self {
        match value.0 {
            WM_LBUTTONDBLCLK => MouseEventType::Double(MouseButton::Left),
            WM_MBUTTONDBLCLK => MouseEventType::Double(MouseButton::Middle),
            WM_RBUTTONDBLCLK => MouseEventType::Double(MouseButton::Right),
            WM_XBUTTONDBLCLK => MouseEventType::Double(MouseButton::xbutton(value.1)),

            WM_LBUTTONDOWN => MouseEventType::Down(MouseButton::Left),
            WM_MBUTTONDOWN => MouseEventType::Down(MouseButton::Middle),
            WM_RBUTTONDOWN => MouseEventType::Down(MouseButton::Right),
            WM_XBUTTONDOWN => MouseEventType::Down(MouseButton::xbutton(value.1)),

            WM_LBUTTONUP => MouseEventType::Up(MouseButton::Left),
            WM_MBUTTONUP => MouseEventType::Up(MouseButton::Middle),
            WM_RBUTTONUP => MouseEventType::Up(MouseButton::Right),
            WM_XBUTTONUP => MouseEventType::Up(MouseButton::xbutton(value.1)),

            WM_MOUSEWHEEL => MouseEventType::Scroll(get_wheel_delta_wparam(value.1) / 120),
            WM_MOUSEHWHEEL => MouseEventType::HScroll(get_wheel_delta_wparam(value.1) / 120),

            WM_MOUSEMOVE => MouseEventType::Move,
            WM_MOUSEHOVER => MouseEventType::Hover,
            533 => { MouseEventType::Ignore }
            _ => {
                eprintln!("Unknown mouse event message: {}", value.0);
                MouseEventType::Ignore
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: u16,
    pub y: u16,
    pub etype: MouseEventType,
}

impl MouseEvent {
    pub fn message(m: u32) -> bool {
        match m {
            WM_LBUTTONDBLCLK | WM_LBUTTONDOWN | WM_LBUTTONUP |
            WM_MBUTTONDBLCLK | WM_MBUTTONDOWN | WM_MBUTTONUP |
            WM_RBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONUP |
            WM_XBUTTONDBLCLK | WM_XBUTTONDOWN | WM_XBUTTONUP |
            WM_MOUSEMOVE | WM_MOUSEWHEEL | WM_MOUSEHOVER |
            WM_MOUSELEAVE | WM_MOUSEHWHEEL | WM_CAPTURECHANGED => true,
            _ => false
        }
    }
}

impl From<(u32, WPARAM, LPARAM)> for MouseEvent {
    fn from(value: (u32, WPARAM, LPARAM)) -> Self {
        MouseEvent {
            x: loword(value.2.0 as usize),
            y: hiword(value.2.0 as usize),
            etype: MouseEventType::from((value.0, value.1.0)),
        }
    }
}
