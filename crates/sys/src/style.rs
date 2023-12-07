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

/// Store light and dark theme colors as Alpha, Red, Green, Blue u32 numbers.
/// This allows the rust syntax of `0xAARRGGBB` for hex values. This means that `AA` can be omited as
/// it isn't used in most cases. The `rgb!` macro can be used to pack the rgb equivalent into a
/// single u32, `rgb!(255, 255, 255)`.
#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd)]
pub struct Background {
    light: u32,
    dark: u32,
}

impl Background {
    pub fn new(light: u32, dark: u32) -> Self {
        Self { light, dark }
    }
    pub fn with_light(color: u32) -> Self {
        Self {
            light: color,
            ..Default::default()
        }
    }

    pub fn with_dark(color: u32) -> Self {
        Self {
            dark: color,
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
            light: 0xFFFFFF,
            dark: 0x0B0B0B,
        }
    }
}

impl From<u32> for Background {
    fn from(v: u32) -> Self {
        Self { light: v, dark: v }
    }
}

