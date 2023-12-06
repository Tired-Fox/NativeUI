extern crate cypress_sys;

use cypress_sys::event::{
    close,
    keyboard::{Key, KeyboardEvent, VirtualKey},
    quit, run, Event, InputEvent,
};
use cypress_sys::modal::{Button, Buttons, Dialog};
use cypress_sys::style::{Background, Theme};
use cypress_sys::{prelude::*, Window};

fn main() {
    let _ = Window::builder()
        .title("Rust Window")
        .theme(Theme::Auto)
        .background(Background::new(0xA35FC1, 0x0B0B0B))
        .icon("../../assets/images/NativeUI.ico")
        .show()
        .unwrap();

    run(|id, event| match event {
        Event::Input(InputEvent::Keyboard(KeyboardEvent::KeyDown(key))) => match key {
            Key::Virtual(VirtualKey::Escape) => {
                close(id);
            }
            _ => {}
        },
        Event::Close => {
            if Dialog::prompt()
                .title("Exit Application")
                .message("Are you sure?")
                .buttons(Buttons::OkCancel)
                .show()
                == Button::Ok
            {
                quit(0)
            }
        }
        _ => {}
    })
}
