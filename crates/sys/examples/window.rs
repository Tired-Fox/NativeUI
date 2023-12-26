extern crate cypress_sys;

use cypress_sys::event::run_with_state;
use cypress_sys::style::{Background, Theme};
use cypress_sys::{
    event::{
        close,
        keyboard::{Key, KeyboardEvent},
        run, Event,
    },
    prelude::*,
    Window,
};
use std::fmt::Debug;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
struct State<T>(Arc<RwLock<T>>);
impl<T> State<T> {
    pub fn new(state: T) -> Self {
        Self(Arc::new(RwLock::new(state)))
    }

    pub fn as_ref(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().unwrap()
    }

    fn as_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap()
    }
}

impl<T: Debug> Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// Controls the state of the modifier keys
#[derive(Debug, Default, Clone, Copy)]
struct KeyState {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub caps: bool,
}

fn main() {
    let _ = Window::builder()
        .title("Rust Window")
        // Try changing this to `Light` and `Dark` and see what happens
        .theme(Theme::Auto)
        .background(Background::new(0xA35FC1, 0x0B0B0B))
        .icon("../../assets/images/NativeUI.ico")
        .show()
        .unwrap();

    let state = State::new(KeyState::default());
    run_with_state(state, move |id, event, state| match event {
        Event::Keyboard(KeyboardEvent::KeyDown(key)) => match key {
            Key::Control => state.as_mut().ctrl = true,
            Key::Alt => state.as_mut().alt = true,
            Key::Shift => state.as_mut().shift = true,
            Key::Capital => state.as_mut().caps = true,
            Key::Escape => close(id),
            key => {
                // Print key with current modifiers
                println!("{:?}: {:?}", state.as_ref(), key);
            }
        },
        Event::Keyboard(KeyboardEvent::KeyUp(key)) => match key {
            Key::Control => state.as_mut().ctrl = false,
            Key::Alt => state.as_mut().alt = false,
            Key::Shift => state.as_mut().shift = false,
            Key::Capital => state.as_mut().caps = false,
            _ => {}
        },
        _ => {}
    })
}
