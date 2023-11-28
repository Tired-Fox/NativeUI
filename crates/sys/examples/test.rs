use windows::core::w;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, IDOK, MB_DEFAULT_DESKTOP_ONLY, MB_ICONEXCLAMATION, MB_OKCANCEL,
};

use cypress_sys::event::close;
use cypress_sys::modal::{Icon, Dialog};
use cypress_sys::{
    event::{
        keyboard::{Key, KeyboardEvent, VirtualKey},
        run, Event, InputEvent,
    },
    prelude::*,
    style::{rgb, Background},
    window::Window,
};

fn main() {
    let window = Window::builder()
        .title("Rust Window")
        .icon("../../assets/images/NativeUI.ico")
        .background(Background::new(0xF3CB87, rgb!(192, 153, 84)))
        .show()
        .unwrap();

    println!("{}", window.id());

    run(|id, event| -> bool {
        match event {
            Event::Close => unsafe {
                let files = Dialog::open_file()
                    .title("Cypress")
                    .multiple(true)
                    .build();

                return Dialog::prompt()
                    .title("Cypress")
                    .message("Are you sure?")
                    .icon(Icon::Exclamation)
                    .build();
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
        }
        true
    });
}
