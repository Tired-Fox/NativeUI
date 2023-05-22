use windows::{
    core::{HSTRING, PCWSTR},
    Win32::UI::WindowsAndMessaging::{
        LoadImageW, IMAGE_ICON, LOADIMAGE_HANDLE, LR_DEFAULTSIZE, LR_LOADFROMFILE,
        LR_LOADTRANSPARENT, LR_SHARED,
    },
};

use super::errors::WinError;

pub fn icon(path: &str) -> Result<LOADIMAGE_HANDLE, String> {
    unsafe {
        match LoadImageW(
            None,
            PCWSTR(HSTRING::from(path).as_ptr()),
            IMAGE_ICON,
            0,
            0,
            LR_LOADFROMFILE | LR_SHARED | LR_LOADTRANSPARENT | LR_DEFAULTSIZE,
        ) {
            Ok(handle) => Ok(handle),
            _ => Err(format!("Failed to load icon '{}': {}", path, WinError::last().message)),
        }
    }
}
