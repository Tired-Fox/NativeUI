extern crate native_ui;
use std::process::exit;

use native_ui::{
    prelude::{component, layout, styles},
    ui::{
        popup::{message, Buttons, Icon},
        Window,
    },
    App,
};

fn main() {
    if message("Quit", "Close Application?", Buttons::YesNo, Icon::Info).into() {
        exit(2)
    }

    let app = App::builder()
        .add_windows(vec![
            Window::builder()
                .title("Rust Window")
                .size(400, 200)
                .class("window1")
                .class("window1")
                .layout(layout![component::text!("Some text here", "dog"), component::text!("Second Line")])
                .build(),
            // Window::builder()
            //     .title("Rust Window 2")
            //     .id("window2")
            //     .layout(layout![component::text!("Some text here", "dog")])
            //     .build(),
        ])
        .set_style(styles!(
            window {
                overflow: scroll;
            }
            .window1 {
                background-color: green;
            }
            #window2 {
                background-color: red;
            }
            text {
                margin-block: 20px;
                height: 200px;
                width: 600px;
            }
            :root {
                --color: #f32;
            }
        ))
        .build();

    app.run().unwrap();
}
