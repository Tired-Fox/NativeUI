extern crate native_ui;
use native_ui::skylight::{
    core::{constants::HS, Brush},
    macros::controls,
    popup::{ButtonLayout, Icon, MessageBox, MessageReturn},
    HookType, Window,
};
use native_ui::styles;

fn main() {
    let mut window = Window::new()
        .size(800, 400)
        .title("Native UI")
        .icon("NativeUi.ico")
        .background(Brush::hatch("B6996D".into(), HS::DCROSS))
        .layout(vec![controls::text!("Native UI Test Window", ".h1")])
        .stylesheet(styles! {
            root {
                width: 50%;
                height: 25%;
            }

            .h1 {
                font-style: italic;
            }
        })
        .open();
    // let stylesheet: Stylesheet = styles! {
    //     div {
    //         width: 10px;
    //         height: 50%;
    //     }

    //     view {
    //         font-style: italic;
    //     }
    // };

    // println!("{:?}", stylesheet);
    // stylesheet.get_style(vec!["div", "view"]);
}
