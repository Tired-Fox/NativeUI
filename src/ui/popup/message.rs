pub use super::{Buttons, Icon, MessageReturn};

#[cfg(target_os = "windows")]
pub use skylight::ui::popup::{ButtonLayout, Icon as PopupIcon, MessageBox};

#[cfg(target_os = "windows")]
pub fn message(title: &str, message: &str, buttons: Buttons, icon: Icon) -> MessageReturn {
    MessageReturn(MessageBox::is_success(MessageBox::new(
        None,
        title,
        message,
        buttons.to_name().into(),
        icon.to_name().into()
    )))
}
