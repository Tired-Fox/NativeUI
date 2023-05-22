use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, GetMessageA, MSG};
use super::core::Renderable;
mod window;

pub mod popup;
pub use window::{Window, HookType};

pub fn run(mut windows: Vec<Window>) -> Result<(), String> {
    for win in windows.iter_mut() {
        win.init()?;
        win.alive = true;
        win.show();
    }

    unsafe {
        let mut message = MSG::default();

        while windows.iter().any(|e| e.alive) {
            GetMessageA(&mut message, None, 0, 0);
            DispatchMessageA(&message);
        }
    }
    Ok(())
}
