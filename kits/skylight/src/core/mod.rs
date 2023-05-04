mod brush;
pub mod constants;
pub mod errors;
pub mod image;
use std::{rc::Rc, cell::RefCell};

pub use brush::*;
use windows::Win32::Foundation::{HMODULE, HWND, RECT};

use crate::control::{Button, Text, Control};

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
    fn update(&self) -> Result<(), String> {
        Ok(())
    }

    fn show(&self);
}

// Styling and layout

#[derive(Debug, Clone)]
pub enum ChildType {
    Control(Rc<RefCell<dyn Control>>),
}

// #[derive(Debug)]
// pub enum ControlType {
//     Text(Text),
//     Button(Button),
//     None,
// }

#[derive(Debug)]
pub enum ViewType {
    Window(HWND, HMODULE),
    None,
}
