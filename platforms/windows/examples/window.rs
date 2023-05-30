extern crate skylight;

use native_core::STYLESHEET;
use skylight::{
    styles,
    core::constants::HS,
    prelude::{component, layout},
    ui::{
        popup::{ButtonLayout, Icon, MessageBox, MessageReturn},
        Brush, HookType, Window,
    },
};

fn main() {
    STYLESHEET.0.write().unwrap().dup(styles!(
        window {
            padding: 2px;
        }
        text {
            margin-block: 2px;
        }
    ));

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
        .layout(layout!(component::text!(
                "Native UI Sample Window",
                "id-goes-here",
                ["text", "two"]
            ),
            component::text!(
                "Second line of text",
                "second-line"
            )
        ));

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
