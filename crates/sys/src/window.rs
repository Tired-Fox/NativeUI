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

pub trait WindowContext:
where Self: Sized {
    type Builder: WindowBuilder<Self>;

    fn create(options: WindowOptions) -> Result<Box<Self>, Error> where Self: Sized;
    fn builder() -> Box<Self::Builder> where Self: Sized;
    fn set_theme(&mut self, theme: Theme) -> Result<(), Error>;
    fn show(&self);
    fn hide(&self);
    fn minimize(&self);
    fn restore(&self);
    fn maximize(&self);
    fn update(&self);
    fn close(&self) -> Result<(), Error>;
    fn id(&self) -> isize;
    fn title(&self) -> String;
}

pub trait WindowBuilder<T>:
where T: WindowContext<Builder = Self>
{
    fn new() -> Self;
    fn title(self, title: &'static str) -> Self;
    fn theme(self, theme: Theme) -> Self;
    fn background(self, background: Background) -> Self;
    fn icon(self, icon: &'static str) -> Self;
    fn create(self) -> Result<Box<T>, Error> where Self: Sized;
    fn show(self) -> Result<Box<T>, Error> where Self: Sized;
}