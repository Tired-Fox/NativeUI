extern crate native_ui;
use std::process::exit;

use native_ui::{App, ui::{popup::{message, Buttons, Icon}, Window}, prelude::{layout, component}, styles};

fn main() {
    if message("Quit", "Close Application?", Buttons::YesNo, Icon::Info).into() {
        exit(2)
    }

    let app = App::builder()
        .add_window(
            Window::builder()
                .title("Rust Window")
                .size(800, 200)
                .class("window1")
                .layout(layout![
                    component::text!("Some text here", "dog")
                ])
                .build()
        )
        .add_window(
            Window::builder()
                .title("Rust Window 2")
                .class("window2")
                .layout(layout![
                    component::text!("Some text here", "dog")
                ])
                .build()
        )
        .set_style(styles!(
            .window1 {
                background-color: green; 
            }
            .window2 {
                background-color: red; 
            }
        ))
        .build();

    app.run().unwrap();

    // Window::builder()
    //     .id("main")
    //     .title("Rust Window")
    //     .class(vec!["material", "shadow"])
    //     .stylesheet(styles!(
    //         #main {
    //             margin: 0;
    //             padding: 0;
    //         }

    //         window {
    //             icon: url("icon.ico");
    //         }
    //         .material { border-radius: 50% }
    //         .shadow { border: 1px solid green }
    //     ))
    //     .size(800, 300)
    //     .icon(Icon::new("icon.ico", 300))
    //     .run();

    // // or
    // let window: Window = Window::builder()
    //     .id("main")
    //     .title("Rust Window")
    //     .class(vec!["material", "shadow"])
    //     .size(800, 300)
    //     .icon(Icon::new("icon.ico", 300))
    //     .build();

    // window.run();
}
