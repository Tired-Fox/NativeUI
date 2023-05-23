extern crate skylight;

use skylight::{
    core::constants::HS,
    macros::{controls, layout},
    ui::{
        popup::{ButtonLayout, Icon, MessageBox, MessageReturn},
        Brush, HookType, Window,
    },
};

// use style::{Prop, BS};

fn main() {
    let window = Window::builder()
        .size(800, 400)
        .title("Native UI")
        .icon("NativeUi.ico")
        .background(Brush::hatch("B6996D".into(), HS::DCROSS))
        .hook(HookType::QUIT, |handle| {
            MessageBox::new(
                handle,
                "Quit Application",
                "Are you sure?",
                ButtonLayout::YesNo,
                Icon::Info,
            ) == MessageReturn::Yes
        })
        .layout(layout!(controls::text!(
            "Native UI Sample Window",
            "id-goes-here",
            ["text", "two"]
        )));

    match window.open() {
        Err(message) => {
            MessageBox::new(
                None,
                "NativeUI Exception",
                message.as_str(),
                ButtonLayout::Ok,
                Icon::Error,
            );
        }
        _ => (),
    }
}
