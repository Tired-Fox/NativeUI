use core::slice;
use std::{fmt::{Display, Debug}, ptr};

use windows::{
    core::PWSTR,
    imp::{
        FormatMessageW, GetLastError, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
    },
    Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_ARGUMENT_ARRAY,
};

#[derive(Clone)]
pub enum Error {
    WinError(WinError),
    GeneralError(String),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::GeneralError(message) => message,
                Self::WinError(WinError { code: _, message }) => message,
            }
        )
    }
}

impl From<WinError> for Error {
    fn from(value: WinError) -> Self {
        Error::WinError(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::GeneralError(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::GeneralError(value)
    }
}

#[derive(Debug, Clone)]
pub struct WinError {
    pub code: u32,
    pub message: String,
}

impl WinError {
    pub fn last() -> Self {
        unsafe { GetLastError().into() }
    }

    pub fn message<S>(message: S) -> Self 
    where S: Display
    {
        let mut win_error: WinError = unsafe { GetLastError().into() };
        win_error.message = format!("[NativeUI] {} [WinError] {}", message, win_error.message);
        win_error
    }
}

impl From<u32> for WinError {
    fn from(code: u32) -> Self {
        let buff_size = 256;

        let mut buff: Vec<u16> = Vec::with_capacity(buff_size);
        for _ in 0..buff_size {
            buff.push(u16::default());
        }

        unsafe {
            let chars = FormatMessageW(
                FORMAT_MESSAGE_ARGUMENT_ARRAY.0
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                ptr::null(),
                code,
                0,
                PWSTR(buff.as_mut_ptr()),
                (buff_size + 1) as u32,
                ptr::null_mut(),
            );

            if chars == 0 {
                return WinError {
                    code,
                    message: String::new(),
                };
            }

            // GetLastError()
            let s1 = slice::from_raw_parts(buff.as_ptr(), chars as usize);
            let error = String::from_utf16(s1);
            let message = match error {
                Ok(message) => message,
                _ => String::new(),
            };
            WinError { code, message }
        }
    }
}
