use native_core::prelude::Renderable;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, GetMessageA, MSG};
mod window;
mod brush;

pub mod component;
pub mod popup;
pub use window::{Window, HookType};
pub use brush::Brush;

use crate::core::error::Error;

pub fn run(mut windows: Vec<Window>) -> Result<(), Error> {
    for win in windows.iter_mut() {
        win.build()?;
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
