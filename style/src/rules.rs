use std::borrow::Borrow;

use cssparser::CowRcStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    PX(f32),
    Percent(f32),
    Default 
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Default
    }
}

impl Unit {
    pub fn from_unit(unit: &CowRcStr, value: &f32) -> Self {
        match unit.borrow() {
            "px" => Unit::PX(value.clone()),
            _ => Unit::Default,
            
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique
}

#[derive(Debug)]
pub enum Style {
    FontStyle(FontStyle),

    Height(Unit),
    Width(Unit)
}
