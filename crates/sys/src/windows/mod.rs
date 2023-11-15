use windows::core::{HSTRING, PCSTR, PCWSTR};
use windows::Win32::Foundation::BOOL;
use windows::UI::ViewManagement::{UIColorType, UISettings};

use clap::Parser;

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

/// Windows uses Alpha, Blue, Green, Red for the order of colors but this struct keep things
/// as Alpha, Red, Green, Blue to make copy pasting hex values into u32 literals easier. Alpha is not
/// moved to the end of the hex as having at the start makes it easier to make it optional. Ex: instead
/// of writing 0x00282828 you can just write 0x282828.
///
/// All u32 values in constructor methods should be the format 0xAARRGGBB for alpha, red, green, and blue respectively.
#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd)]
pub struct Background {
    light: u32,
    dark: u32,
}

impl Background {
    pub fn new(light: u32, dark: u32) -> Self {
        Self { light: reorder_u32(light), dark: reorder_u32(dark) }
    }
    pub fn with_light(color: u32) -> Self {
        Self {
            light: reorder_u32(color),
            ..Default::default()
        }
    }

    pub fn with_dark(color: u32) -> Self {
        Self {
            dark: reorder_u32(color),
            ..Default::default()
        }
    }

    pub fn color(&self) -> u32 {
        if is_dark_mode().into() {
            self.dark
        } else {
            self.light
        }
    }
}

impl Default for Background {
    fn default() -> Self {
        Self {
            light: 0x00FFFFFF,
            dark: 0x00282828,
        }
    }
}

impl From<u32> for Background {
    fn from(v: u32) -> Self {
        let v = reorder_u32(v);
        Self { light: v, dark: v }
    }
}

fn reorder_u32(value: u32) -> u32 {
    let values = u32::to_be_bytes(value);
    u32::from_be_bytes([values[0], values[3], values[2], values[1]])
}
