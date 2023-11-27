use std::thread::sleep;
use std::time::Duration;
use cypress_sys::{
    prelude::*,
    event::{Event, InputEvent, keyboard::{Key, KeyboardEvent, VirtualKey}, quit, run},
    style::{Background, rgb},
    window::Window
};
use cypress_sys::event::close;

fn main() {
    let window = Window::builder()
        .title("Rust Window")
        .icon("../../assets/images/NativeUI.ico")
        .background(Background::new(0xF3CB87, rgb!(192, 153, 84)))
        .show()
        .unwrap();

    println!("{}", window.id());

    run(move |id, event| match event {
        Event::Close => {
            println!("Close");
        },
        Event::Input(InputEvent::Keyboard(ke)) => match ke {
            KeyboardEvent::KeyDown(v) => {
                if v == Key::Virtual(VirtualKey::Escape) {
                    close(id);
                }
            }
            _ => {}
        },
        _ => {}
    });
}
