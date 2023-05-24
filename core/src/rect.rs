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

    pub fn shift(&self, amount: &(i32, i32, i32, i32)) -> Rect {
        // top right bottom left
        Rect {
            top: self.top + amount.0,
            right: self.right - amount.1,
            bottom: self.bottom - amount.2,
            left: self.left + amount.3,
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect::from(0)
    }
}

impl From<i32> for Rect {
    fn from(value: i32) -> Self {
        Rect {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}

impl From<[i32; 1]> for Rect {
    fn from(value: [i32; 1]) -> Self {
        Rect::from(value[0])
    }
}

impl From<[i32; 2]> for Rect {
    fn from(value: [i32; 2]) -> Self {
        Rect {
            left: 0,
            top: 0,
            right: value[0],
            bottom: value[1],
        }
    }
}

impl From<[i32; 4]> for Rect {
    fn from(value: [i32; 4]) -> Self {
        Rect {
            left: value[0],
            top: value[1],
            right: value[2],
            bottom: value[3],
        }
    }
}
