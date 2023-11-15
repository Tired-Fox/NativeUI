use cypress_sys::event::keyboard::{Key, KeyboardEvent, VirtualKey};
use cypress_sys::event::{Event, InputEvent};
use cypress_sys::windows::Background;
use cypress_sys::windows::event::run;
use cypress_sys::windows::window::Window;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::UI::WindowsAndMessaging::PostQuitMessage;

/*
 */
fn main() {
    Window::builder()
        .title("Rust Window")
        .icon("../../assets/images/NativeUI.ico")
        .background(Background::new(0xF3CB87, 0xC09954))
        .show()
        .create()
        .unwrap();

    run(move |event| match event {
        Event::Repaint { id } => unsafe {
            ValidateRect(HWND(id), None);
        },
        Event::Input {
            value: InputEvent::Keyboard(ke),
            ..
        } => match ke {
            KeyboardEvent::KeyDown(v) => {
                if v == Key::Virtual(VirtualKey::Escape) {
                    unsafe { PostQuitMessage(0) }
                }
            }
            _ => {}
        },
        _ => {}
    });
}
