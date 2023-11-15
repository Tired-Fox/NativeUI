pub mod keyboard;
pub mod mouse;

use keyboard::KeyboardEvent;
use mouse::MouseEvent;

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

impl From<(u32, usize, isize)> for InputEvent {
    fn from(v: (u32, usize, isize)) -> Self {
        match v.0 {
            _ if keyboard::KeyboardEvent::message(v.0) => InputEvent::Keyboard(KeyboardEvent::from(v)),
            _ if mouse::MouseEvent::message(v.0) => InputEvent::Mouse(MouseEvent::from(v)),
            _ => panic!("Unknown keyboard event message: {}", v.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaintEvent{ pub handle: isize }

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
