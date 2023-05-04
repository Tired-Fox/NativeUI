extern crate skylight;

use skylight::{
    control::*,
    // core::{ChildType, ControlType},
    text,
    HookType, Window,
    popup::{MessageBox, MessageReturn, ButtonLayout, Icon}
};

use style::{Prop, BS};

fn main() {
    let mut window = Window::new()
        // .size(800, 400)
        .title("Native UI")
        .icon("NativeUi.ico")
        // .background(Brush::hatch("B6996D".into(), HS::DCROSS))
        .hook(HookType::QUIT, |handle| {
            MessageBox::new(
                handle,
                "Quit Application",
                "Are you sure?",
                ButtonLayout::YesNo,
                Icon::Info,
            ) == MessageReturn::Yes
        })
        .style(vec![
            ("width", Prop::PX(800)),
            ("height", Prop::PX(400)),
            (
                "background",
                Prop::Background("B6996D".into(), Some(BS::DCROSS)),
            ),
        ])
        .layout(&mut vec![
            text!("Native UI Test Window")
        ]);

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
