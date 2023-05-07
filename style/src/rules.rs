use std::borrow::Borrow;

use cssparser::CowRcStr;

#[derive(Debug)]
pub enum Unit {
    PX(f32),
    Percent(f32),
    Uknown
}

impl Unit {
    pub fn from_unit(unit: &CowRcStr, value: &f32) -> Self {
        match unit.borrow() {
            "px" => Unit::PX(value.clone()),
            _ => Unit::Uknown,
            
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontStyle {
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
