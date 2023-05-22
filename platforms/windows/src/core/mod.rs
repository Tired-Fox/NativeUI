mod brush;
pub mod constants;
pub mod errors;
pub mod image;
pub mod layout;
use std::{cell::RefCell, rc::Rc};

use native_core::Rect;

pub use brush::*;
use style::{Appearance, Dimensions};
use windows::Win32::{
    Foundation::{HMODULE, HWND, RECT},
    UI::WindowsAndMessaging::{GetWindowLongPtrW, GWLP_USERDATA},
};

use crate::{control::Control, ui::Window};

pub enum ProcResult {
    Default,
    Success,
    Fail,
}

pub trait Renderable {
    fn update(
        &mut self,
        parent: (Rect, (Dimensions, Appearance)),
        previous: Option<(Rect, (Dimensions, Appearance))>,
    ) -> Result<(), String>;

    fn show(&self);
    fn hide(&self);
    fn rect(&self) -> &Rect;
    fn style(&self) -> &(Dimensions, Appearance);
    fn handle(&self) -> &HWND;
}

pub trait View: Renderable {
    fn children(&mut self) -> &mut Vec<ChildType>;
}

pub fn to_RECT(rect: Rect) -> RECT {
    RECT {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
    }
}

pub fn to_Rect(rect: RECT) -> Rect {
    Rect {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
    }
}

// Styling and layout

#[derive(Debug, Clone)]
pub enum ChildType {
    Control(Rc<RefCell<dyn Control>>),
}

#[derive(Debug, Clone)]
pub enum ViewType {
    Window(HWND, HMODULE),
    None,
}

pub fn get_window<'a>(window: HWND) -> Result<&'a Window, String> {
    unsafe {
        let this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Window;
        if !this.is_null() {
            Ok(&*this)
        } else {
            Err("No window assigned to handle".to_owned())
        }
    }
}
