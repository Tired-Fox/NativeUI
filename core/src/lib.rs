mod rect;
mod style_manager;

use std::{cell::RefCell, sync::Arc};

pub use rect::Rect;
pub use style_manager::STYLESHEET;


pub trait Renderable {
    // Handle
    // id
    // class
    // rect
    // default rect
}

pub trait Container: Renderable {
    // Parent alignment styles
}

pub trait Component: Renderable {}


pub enum Child {
    Component(Arc<RefCell<dyn Component>>),
    Layout(Layout)
}

pub struct Layout {
    parent: Arc<RefCell<dyn Container>>,
    children: Vec<Child>
}

impl Layout {
    pub fn update(&mut self) {
    }
}
