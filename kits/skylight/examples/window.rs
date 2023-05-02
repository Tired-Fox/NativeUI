extern crate skylight;

use skylight::{
    core::{style::hs, Brush},
    popup::{ButtonLayout, Icon, MessageBox, MessageReturn},
    HookType, Window,
};

fn main() {
    match Window::new()
        .size(800, 400)
        .title("Native UI")
        .icon("NativeUi.ico")
        .background(Brush::hatch("#B6996D".into(), hs::DIAGNOL))
        .hook(HookType::QUIT, |handle| {
            MessageBox::new(
                Some(handle),
                "Quit Application",
                "Are you sure?",
                ButtonLayout::YesNo,
                Icon::Info,
            ) == MessageReturn::Yes
        })
        .open()
    {
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
