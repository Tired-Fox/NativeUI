pub mod popup;
mod win;

pub use win::*;

use windows::Win32::UI::WindowsAndMessaging::{MSG, GetMessageA, DispatchMessageA};

pub fn run(mut windows: Vec<&mut Window>) -> Result<(), &'static str> {
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
