pub mod style;
pub mod image;
pub mod errors;
mod brush;
pub use brush::*;

#[derive(Debug)]
pub struct Rect {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

impl Rect {
    pub fn new(left: i16, top: i16, right: i16, bottom: i16) -> Self {
        Rect {
            left,
            top,
            right,
            bottom
        }
    }

    pub fn width(&self) -> i16 {
        self.right - self.left
    }

    pub fn height(&self) -> i16 {
        self.bottom - self.top
    }
}
