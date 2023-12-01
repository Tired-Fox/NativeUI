use crate::error::Error;
use windows::core::{HSTRING, PCSTR, PCWSTR};
use windows::Win32::Foundation::BOOL;
use windows::Win32::UI::Controls::Dialogs::COMMON_DLG_ERRORS;
use windows::Win32::UI::Controls::Dialogs::{
    CDERR_DIALOGFAILURE, CDERR_FINDRESFAILURE, CDERR_INITIALIZATION, CDERR_LOADRESFAILURE,
    CDERR_LOADSTRFAILURE, CDERR_LOCKRESFAILURE, CDERR_MEMALLOCFAILURE, CDERR_MEMLOCKFAILURE,
    CDERR_NOHINSTANCE, CDERR_NOHOOK, CDERR_NOTEMPLATE, CDERR_REGISTERMSGFAIL, CDERR_STRUCTSIZE,
};
use windows::UI::ViewManagement::{UIColorType, UISettings};

pub mod event;
pub mod modal;
pub mod window;

lazy_static::lazy_static! {
    pub static ref UI_SETTINGS: UISettings = UISettings::new().unwrap();
}

impl From<windows::core::Error> for Error {
    fn from(error: windows::core::Error) -> Self {
        Self {
            code: error.code().0 as isize,
            message: error.message().to_string_lossy(),
        }
    }
}

impl From<COMMON_DLG_ERRORS> for Error {
    fn from(error: COMMON_DLG_ERRORS) -> Self {
        Self {
            code: error.0 as isize,
            message: match error {
                CDERR_DIALOGFAILURE => "Dialog could not be created".into(),
                CDERR_FINDRESFAILURE => "Failed to find specified resource".into(),
                CDERR_INITIALIZATION => "Failed to initialize dialog".into(),
                CDERR_LOADRESFAILURE => "Failed to load specified resource".into(),
                CDERR_LOADSTRFAILURE => "Failed to load specified string".into(),
                CDERR_LOCKRESFAILURE => "Failed to lock specified resource".into(),
                CDERR_MEMALLOCFAILURE => "Failed to allocate memory for internal structures".into(),
                CDERR_MEMLOCKFAILURE => "Failed to lock memory associated with handle".into(),
                CDERR_NOHINSTANCE => {
                    "No instance handle specified when template was expected".into()
                }
                CDERR_NOHOOK => "No hook handle specified when template was expected".into(),
                CDERR_NOTEMPLATE => "No template was specified".into(),
                CDERR_REGISTERMSGFAIL => "Failed to register message".into(),
                CDERR_STRUCTSIZE => "Incorrect size of template structure".into(),
                _ => "Unknown error".into(),
            },
        }
    }
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

// TODO: Needed for other platforms?
pub(crate) fn swap_rb(value: u32) -> u32 {
    let values = u32::to_be_bytes(value);
    u32::from_be_bytes([values[0], values[3], values[2], values[1]])
}
