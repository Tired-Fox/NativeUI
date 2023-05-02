extern crate native_ui;

use skylight::{
    core::{style::hs, Brush},
    popup::{ButtonLayout, Icon, MessageBox, MessageReturn},
    run, EventKey, Window,
};

fn main() {
    let mut window = Window::new()
        .size(800, 400)
        .title("Native UI")
        .icon("NativeUi.ico")
        .background(Brush::hatch("#B6996D".into(), hs::DIAGNOL))
        .hook(EventKey::QUIT, |handle| {
            MessageBox::new(
                handle,
                "Quit Application",
                "Are you sure?",
                ButtonLayout::YesNo,
                Icon::Info,
            ) == MessageReturn::Yes
        });

    let mut window2 = Window::new()
        .size(400, 300)
        .title("Rust Window")
        .background(Brush::solid("#F0F".into()));

    run(vec![&mut window, &mut window2]);
}
