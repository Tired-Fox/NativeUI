mod brush;
pub mod constants;
pub mod errors;
pub mod image;
pub mod layout;
use std::{cell::RefCell, rc::Rc};

pub use brush::*;
use style::{Appearance, Dimensions, Stylesheet};
use windows::Win32::{
    Foundation::{HMODULE, HWND, RECT},
    UI::WindowsAndMessaging::{GetWindowLongPtrW, GWLP_USERDATA},
};

use crate::{control::Control, Window};

pub enum ProcResult {
    Default,
    Success,
    Fail,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rect {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
}

impl From<RECT> for Rect {
    fn from(value: RECT) -> Self {
        Rect {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

impl From<Rect> for RECT {
    fn from(value: Rect) -> Self {
        RECT {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

pub trait Renderable {
    fn update(
        &mut self,
        parent: (Rect, (Dimensions, Appearance)),
        previous: (Rect, (Dimensions, Appearance)),

    ) -> Result<(), String> {
        Ok(())
    }

    fn show(&self);
    fn rect(&self) -> &Rect;
    fn style(&self) -> &(Dimensions, Appearance);
    fn handle(&self) -> &HWND;
}

pub trait View: Renderable {
    fn children(&mut self) -> &mut Vec<ChildType>;
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
