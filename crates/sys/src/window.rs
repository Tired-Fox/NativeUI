use crate::error::Error;
use crate::style::{Background, Theme};

#[cfg(target_os = "windows")]
pub use crate::windows::window::Window;

#[derive(Debug)]
pub struct WindowOptions {
    pub title: &'static str,
    pub icon: Option<&'static str>,

    pub theme: Theme,
    pub background: Background,

    pub show: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "",
            icon: None,

            theme: Theme::Auto,
            background: Background::default(),

            show: false,
        }
    }
}

pub trait WindowContext
where
    Self: Sized,
{
    type Builder: WindowBuilder;

    fn create(options: WindowOptions) -> Result<isize, Error>
    where
        Self: Sized;
    fn builder() -> Box<Self::Builder>
    where
        Self: Sized;
    fn set_theme(&mut self, theme: Theme) -> Result<(), Error>;
    fn show(id: isize);
    fn hide(id: isize);
    fn minimize(id: isize);
    fn restore(id: isize);
    fn maximize(id: isize);
    fn close(id: isize) -> Result<(), Error>;
}

pub trait WindowBuilder {
    fn new() -> Self;
    fn title(self, title: &'static str) -> Self;
    fn theme(self, theme: Theme) -> Self;
    fn background(self, background: Background) -> Self;
    fn icon(self, icon: &'static str) -> Self;
    fn create(self) -> Result<isize, Error>
    where
        Self: Sized;
    fn show(self) -> Result<isize, Error>
    where
        Self: Sized;
}
