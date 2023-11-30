use windows::core::w;
use cypress_sys::event::close;
use cypress_sys::modal::{Dialog, Icon};
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

fn main() {
    let window = Window::builder()
        .title("Rust Window")
        .icon("../../assets/images/NativeUI.ico")
        .theme(Theme::Light)
        .background(Background::new(0xF3CB87, rgb!(192, 153, 84)))
        .show()
        .unwrap();

    let file_picker = Dialog::file()
        .title("Cypress")
        .multi_select()
        .filter("All Files", ["*"])
        .filter("Text Files", ["txt"])
        .filter_index(2)
        .filename("test")
        .default_extension("txt");

    let prompt = Dialog::prompt()
        .title("Cypress")
        .message("Are you sure?")
        .icon(Icon::Exclamation);

    run(move |id, event| -> bool {
        match event {
            Event::Close => {
                let files = file_picker.save();
                println!("{:?}", files);
                return prompt.run();
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
