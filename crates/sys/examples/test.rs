use cypress_sys::windows::event::run;
use cypress_sys::windows::window::{Theme, Window};
use windows::core::w;
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, DestroyWindow, MB_OKCANCEL, MessageBoxW, PostQuitMessage};
use windows::Win32::UI::WindowsAndMessaging::{WM_PAINT, WM_DESTROY, WM_CLOSE};

fn main() {
    let window = Window::builder()
        .title("Window from Rust")
        .theme(Theme::Dark)
        .proc(|handle, message, lparam, wparam| {
            match message {
                WM_CLOSE => unsafe {
                    if MessageBoxW(None, w!("Ansi"), w!("Cypress"), MB_OKCANCEL).0 == 1 {
                        DestroyWindow(handle);
                    }
                    true
                },
                WM_DESTROY => unsafe {
                    PostQuitMessage(0);
                    true
                },
                WM_PAINT => unsafe {
                    println!("User def pain");
                    ValidateRect(handle, None);
                    true
                }
                _ => false,
            }
        })
        .create()
        .unwrap();
    window.show();
    run();
}
