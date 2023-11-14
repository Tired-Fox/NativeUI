use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::UI::WindowsAndMessaging::PostQuitMessage;

use cypress_sys::event::{Event, InputEvent};
use cypress_sys::event::keyboard::{Key, KeyboardEvent, VirtualKey};
use cypress_sys::windows::event::run;
use cypress_sys::windows::window::Window;

fn main() {
    let window = Window::builder()
        .title("Window from Rust")
        .show()
        .create()
        .unwrap();

    run(move |event| {
        match event {
            Event::Paint { id } => unsafe {
                ValidateRect(HWND(id), None);
            },
            Event::Input { value: InputEvent::Keyboard(ke), .. } => {
                match ke {
                    KeyboardEvent::KeyDown(v) => {
                        if v == Key::Virtual(VirtualKey::Escape) {
                            unsafe { PostQuitMessage(0) }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
