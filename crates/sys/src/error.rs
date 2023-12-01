use std::fmt::Display;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct Error {
    pub code: isize,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CYPRESS] {}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self {
            code: error.raw_os_error().unwrap_or(0) as isize,
            message: error.to_string(),
        }
    }
}