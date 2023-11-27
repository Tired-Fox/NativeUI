use windows::core::{HSTRING, PCSTR, PCWSTR};
use windows::Win32::Foundation::BOOL;
use windows::UI::ViewManagement::{UIColorType, UISettings};

pub mod event;
pub mod win_error;
pub mod window;

lazy_static::lazy_static! {
    pub static ref UI_SETTINGS: UISettings = UISettings::new().unwrap();
}

pub trait IntoPCSTR {
    fn as_pcstr(&self) -> PCSTR;
}
pub trait IntoPCWSTR {
    fn as_pcwstr(&self) -> PCWSTR;
}

impl IntoPCSTR for HSTRING {
    fn as_pcstr(&self) -> PCSTR {
        PCSTR::from_raw(self.as_ptr() as _)
    }
}

impl IntoPCWSTR for HSTRING {
    fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR::from_raw(self.as_ptr() as _)
    }
}

pub fn hiword(v: usize) -> u16 {
    (v >> 16) as u16
}

pub fn loword(v: usize) -> u16 {
    v as u16
}

pub fn get_wheel_delta_wparam(wparam: usize) -> i16 {
    hiword(wparam) as i16
}

pub fn is_dark_mode() -> BOOL {
    let color = UI_SETTINGS.GetColorValue(UIColorType::Foreground).unwrap();
    BOOL((((5 * color.G as u32) + (2 * color.R as u32) + color.B as u32) > (8u32 * 128u32)) as i32)
}