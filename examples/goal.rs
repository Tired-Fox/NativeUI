extern crate native_ui;
use std::process::exit;

use native_ui::ui::{
    popup::{message, Buttons, Icon},
    // Window,
};

fn main() {
    if message("Quit", "Close Application?", Buttons::YesNo, Icon::Info).into() {
        exit(2)
    }

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
