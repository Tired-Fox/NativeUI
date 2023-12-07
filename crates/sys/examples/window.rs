extern crate cypress_sys;

use cypress_sys::style::{Background, Theme};
use cypress_sys::{event::run, prelude::*, Window};

fn main() {
    let _ = Window::builder()
        .title("Rust Window")
        // Try changing this to `Light` and `Dark` and see what happens
        .theme(Theme::Auto)
        .background(Background::new(0xA35FC1, 0x0B0B0B))
        .icon("../../assets/images/NativeUI.ico")
        .show()
        .unwrap();

    run(|_id, _event| {})
}
