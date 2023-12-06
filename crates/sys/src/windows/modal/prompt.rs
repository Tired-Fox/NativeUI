use windows::core::HSTRING;
use windows::Win32::UI::WindowsAndMessaging::{MB_DEFAULT_DESKTOP_ONLY, MESSAGEBOX_STYLE, MessageBoxW};
use crate::error::Error;
use crate::modal::{Button, Prompt};
use crate::windows::IntoPCWSTR;

pub struct MsgBox<'a>(&'a Prompt);
impl<'a> MsgBox<'a> {
    pub fn new(context: &'a Prompt) -> Self {
        Self(context)
    }

    pub fn show(&self) -> Result<Button, Error> {
        Ok(unsafe {
            MessageBoxW(
                None,
                HSTRING::from(self.0.message).as_pcwstr(),
                HSTRING::from(self.0.title).as_pcwstr(),
                MB_DEFAULT_DESKTOP_ONLY
                    | MESSAGEBOX_STYLE::from(self.0.icon)
                    | MESSAGEBOX_STYLE::from(self.0.buttons),
            )
                .into()
        })
    }
}
