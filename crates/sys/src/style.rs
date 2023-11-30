#[derive(Default, Clone, Copy, Debug)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    Auto,
}

#[macro_export]
macro_rules! rgb {
    ($red: literal, $green: literal, $blue: literal) => {
       u32::from_be_bytes([0, $red, $green, $blue])
    };
}
pub use crate::rgb;

/// Windows uses Alpha, Blue, Green, Red for the order of colors but this struct keep things
/// as Alpha, Red, Green, Blue to make copy pasting hex values into u32 literals easier. Alpha is not
/// moved to the end of the hex as having at the start makes it easier to make it optional. Ex: instead
/// of writing 0x00282828 you can just write 0x282828.
///
/// All u32 values in constructor methods should be the format 0xAARRGGBB for alpha, red, green, and blue respectively.
#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd)]
pub struct Background {
    light: u32,
    dark: u32,
}

impl Background {
    pub fn new(light: u32, dark: u32) -> Self {
        Self {
            light: reorder_u32(light),
            dark: reorder_u32(dark),
        }
    }
    pub fn with_light(color: u32) -> Self {
        Self {
            light: reorder_u32(color),
            ..Default::default()
        }
    }

    pub fn with_dark(color: u32) -> Self {
        Self {
            dark: reorder_u32(color),
            ..Default::default()
        }
    }

    pub fn dark(&self) -> u32 {
        self.dark
    }

    pub fn light(&self) -> u32 {
        self.light
    }

    pub fn color(&self, dark_mode: bool) -> u32 {
        if dark_mode {
            self.dark
        } else {
            self.light
        }
    }
}

impl Default for Background {
    fn default() -> Self {
        Self {
            light: 0x00FFFFFF,
            dark: 0x00282828,
        }
    }
}

impl From<u32> for Background {
    fn from(v: u32) -> Self {
        let v = reorder_u32(v);
        Self { light: v, dark: v }
    }
}

// TODO: Needed for other platforms?
fn reorder_u32(value: u32) -> u32 {
    let values = u32::to_be_bytes(value);
    u32::from_be_bytes([values[0], values[3], values[2], values[1]])
}
