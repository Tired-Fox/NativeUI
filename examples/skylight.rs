extern crate native_ui;

use native_ui::skylight::{
    core::{constants::{HS, SBS}, Brush, ChildType},
    macros::controls,
    control::ScrollBar,
    ui::{popup::{ButtonLayout, Icon, MessageBox, MessageReturn}, Window, HookType},
};

use native_ui::styles;
use style::Stylesheet;

fn main() {
    let mut window = Window::new()
        .title("Native UI")
        .icon("NativeUi.ico")
        .background(Brush::hatch("B6996D".into(), HS::DCROSS))
        .layout(vec![
            controls::text!("Native UI Test Window", ".h1"),
            controls::text!("Second Line", ".h2"),
        ])
        // .stylesheet(styles! {
        //     root {
        //         width: 75%;
        //         height: 75%;
        //         padding-right: 10px;
        //         background-color: #ff6b6b;
        //         left: 10px;
        //     }

        //     .h1 {
        //         font-style: italic;
        //         padding: 10px 10px 5px 50%;
        //     }
        // })
        .stylesheet(Stylesheet::file("styles.css"))
        .open();
}
