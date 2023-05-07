extern crate native_ui;
// use native_ui::skylight::{
//     core::{constants::HS, Brush},
//     macros::controls,
//     popup::{ButtonLayout, Icon, MessageBox, MessageReturn},
//     HookType, Window,
// };
use native_ui::styles;

fn main() {
    // let mut window = Window::new()
    //     .size(800, 400)
    //     .title("Native UI")
    //     .icon("NativeUi.ico")
    //     .background(Brush::hatch("B6996D".into(), HS::DCROSS))
    //     .layout(vec![controls::text!("Native UI Test Window")])
    // // .style(vec![
    // //     ("width", Prop::PX(800)),
    // //     ("height", Prop::PX(400)),
    // //     (
    // //         "background",
    // //         Prop::Background("B6996D".into(), Some(BS::DCROSS)),
    // //     ),
    // // ]);
    //     .open();
    let stylesheet = styles! {
        div {
            width: 10px;
            height: 50%;
        }
    };

    println!("{:?}", stylesheet);
}
