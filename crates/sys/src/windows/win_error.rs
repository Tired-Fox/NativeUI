use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct WinError {
    pub code: i32,
    pub message: String,
}

impl From<windows::core::Error> for WinError {
    fn from(value: windows::core::Error) -> Self {
        WinError {
            code: value.code().0,
            message: value.message().to_string(),
        }
    }
}

impl Display for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[WinError {}]: {}",
            self.code,
            self.message
        )
    }
}

impl std::error::Error for WinError {}