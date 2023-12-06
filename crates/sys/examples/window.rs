extern crate cypress_sys;

use cypress_sys::style::{Background, Theme};
use cypress_sys::{prelude::*, Window};

fn main() {
    let _ = Window::builder()
        .title("Rust Window")
        .theme(Theme::Auto)
        .background(Background::new(0xA35FC1, 0x0B0B0B))
        .icon("../../assets/images/NativeUI.ico")
        .show()
        .unwrap();
}
