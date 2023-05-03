extern crate skylight;

use skylight::{
    popup::{ButtonLayout, Icon, MessageBox, MessageReturn},
    HookType, Window,
};

use style::{Prop, BS};

fn main() {
    let window = Window::new()
        .title("Native UI")
        .icon("NativeUi.ico")
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
            ("width", Prop::PX(1000)),
            ("height", Prop::PX(800)),
            ("background", Prop::Background("B6996D".into(), Some(BS::CROSS))),
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
        _ => ()
    }
}
