use std::{borrow::Borrow, default};

use crate::size::Size;

use super::Color;

use cssparser::CowRcStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    PX(f32),
    Percent(f32),
    FitConent,
    Default,
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

    pub fn as_i32(&self, total: i32, default: i32) -> i32 {
        match self {
            Self::PX(px) => px.clone() as i32,
            Self::Percent(percent) => (total as f32 * percent) as i32,
            Self::FitConent => default,
            Self::Default => default,
        }
    }

    pub fn as_f32(&self, total: f32, default: f32) -> f32 {
        match self {
            Self::PX(px) => px.clone(),
            Self::Percent(percent) => total * percent,
            Self::FitConent => default,
            Self::Default => default,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Position {
    Absolute,
    #[default]
    Relative,
}

#[derive(Debug)]
pub enum Style {
    BackgroundColor(Color),
    FontStyle(FontStyle),

    MinHeight(Unit),
    Height(Unit),
    MaxHeight(Unit),
    MinWidth(Unit),
    Width(Unit),
    MaxWidth(Unit),

    Position(Position),

    Inset(Size),
    InsetBlock(Unit),
    InsetInline(Unit),
    Left(Unit),
    Top(Unit),
    Right(Unit),
    Bottom(Unit),

    Padding(Size),
    PaddingBlock(Unit),
    PaddingInline(Unit),
    PaddingLeft(Unit),
    PaddingTop(Unit),
    PaddingRight(Unit),
    PaddingBottom(Unit),

    Margin(Size),
    MarginBlock(Unit),
    MarginInline(Unit),
    MarginLeft(Unit),
    MarginTop(Unit),
    MarginRight(Unit),
    MarginBottom(Unit),
}
