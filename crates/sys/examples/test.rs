use cypress_sys::event::close;
use cypress_sys::modal::{Button, ColorDialog, Dialog, Icon};
use cypress_sys::style::Theme;
use cypress_sys::{
    event::{
        keyboard::{Key, KeyboardEvent, VirtualKey},
        run, Event, InputEvent,
    },
    prelude::*,
    style::{rgb, Background},
    window::Window,
};
use std::sync::Arc;

fn main() {
    let _window = Window::builder()
        .title("Rust Window")
        .icon("../../assets/images/NativeUI.ico")
        .theme(Theme::Light)
        .background(Background::new(0xF3CB87, rgb!(192, 153, 84)))
        .show()
        .unwrap();

    let file_picker = Dialog::file()
        .title("Cypress")
        .filters(2, &[
            ("All Files", &["*"]),
            ("Text Files", &["*.txt"]),
        ])
        .multiple()
        .filename("test", "txt");

    let prompt = Dialog::prompt()
        .title("Cypress")
        .message("Are you sure?")
        .icon(Icon::Exclamation);

    // TODO: Add cross platform assignment of dialog to parent window
    let mut color_picker = Dialog::color().initial(0x00FF00);

    run(move |id, event| -> bool {
        match event {
            Event::Close => {
                return prompt.show() == Button::Ok;
            }
            // Keydown events
            Event::Input(InputEvent::Keyboard(KeyboardEvent::KeyDown(key))) => match key {
                Key::Virtual(VirtualKey::Escape) => {
                    close(id);
                }
                Key::Char('1') => {
                    let files = file_picker.open_file();
                    println!("{:?}", files);
                }
                Key::Char('2') => {
                    let file = file_picker.save_file();
                    println!("{:?}", file);
                }
                Key::Char('3') => {
                    let files = file_picker.open_folder();
                    println!("{:?}", files);
                }
                Key::Char('4') => {
                    let result = color_picker.show();
                    println!("Color: {:?}", result);
                    println!(
                        "  - customs: [{}]",
                        ColorDialog::get_custom_colors()
                            .iter()
                            .map(|x| format!("{:x?}", x))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
                _ => {}
            },
            _ => {}
        }
        true
    });
}
