use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage, MSG, WM_DESTROY, WM_PAINT, WM_SYSCOLORCHANGE, PeekMessageW, PM_REMOVE};

pub fn watch(handle: HWND) {
    unsafe {
        let mut message = MSG::default();
        while GetMessageW(&mut message, handle, 0, 0).into() {
            DispatchMessageW(&message);
        }
    }
}

pub extern "system" fn wnd_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            },
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
