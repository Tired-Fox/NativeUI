use crate::parser::error::StyleParseError;
use crate::parser::Parse;
use cssparser::{ParseError, ParseErrorKind, Parser, Token};
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/number
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Number(pub f32);
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for Number {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<f32> for Number {
    fn from(num: f32) -> Self {
        Number(num)
    }
}
impl From<&f32> for Number {
    fn from(num: &f32) -> Self {
        Number(*num)
    }
}
impl Parse for Number {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        input.skip_whitespace();
        let start = input.current_source_location();
        match input.expect_number() {
            Ok(num) => Ok(Number(num)),
            _ => Err(start.new_custom_error(StyleParseError::ExpectedNumber)),
        }
    }
}

/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/percentage
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(pub f32);
impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0 * 100.0)
    }
}
impl Deref for Percent {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Percent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<f32> for Percent {
    fn from(num: f32) -> Self {
        Percent(num)
    }
}
impl From<&f32> for Percent {
    fn from(num: &f32) -> Self {
        Percent(*num)
    }
}
impl Parse for Percent {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        input.skip_whitespace();
        let start = input.current_source_location();
        match input.expect_percentage() {
            Ok(num) => Ok(Percent(num)),
            _ => Err(start.new_custom_error(StyleParseError::ExpectedPercent)),
        }
    }
}

/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/integer
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Integer(pub i32);
impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for Integer {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Integer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<i32> for Integer {
    fn from(num: i32) -> Self {
        Integer(num)
    }
}
impl From<&i32> for Integer {
    fn from(num: &i32) -> Self {
        Integer(*num)
    }
}
impl Parse for Integer {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.expect_integer() {
            Ok(num) => Ok(Integer(num)),
            _ => Err(input.new_custom_error(StyleParseError::ExpectedInteger)),
        }
    }
}

impl Parse for String {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.expect_string() {
            Ok(string) => Ok(string.to_string()),
            _ => Err(input.new_custom_error(StyleParseError::ExpectedString)),
        }
    }
}

impl Parse for () {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        Err(input.new_custom_error(StyleParseError::UnkownSyntax))
    }
}

/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/absolute-size
#[derive(Debug)]
pub enum AbsoluteSize {
    XSmall,
    XXSmall = 1,
    Small = 2,
    Medium = 3,
    Large = 4,
    XLarge = 5,
    XXLarge = 6,
    XXXLarge = 7,
}

impl Parse for AbsoluteSize {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.expect_ident()?.as_ref() {
            "xx-small" => Ok(AbsoluteSize::XXSmall),
            "x-small" => Ok(AbsoluteSize::XSmall),
            "small" => Ok(AbsoluteSize::Small),
            "medium" => Ok(AbsoluteSize::Medium),
            "large" => Ok(AbsoluteSize::Large),
            "x-large" => Ok(AbsoluteSize::XLarge),
            "xx-large" => Ok(AbsoluteSize::XXLarge),
            "xxx-large" => Ok(AbsoluteSize::XXXLarge),
            _ => Err(input.new_custom_error(StyleParseError::ExpectedKeywords(vec![
                "xx-small",
                "x-small",
                "small",
                "medium",
                "large",
                "x-large",
                "xx-large",
                "xxx-large",
            ]))),
        }
    }
}

/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/angle
#[derive(Debug)]
pub enum Angle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
    Turn(f32),
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Angle::Deg(deg) => format!("{}deg", deg),
                Angle::Grad(grad) => format!("{}grad", grad),
                Angle::Rad(rad) => format!("{}rad", rad),
                Angle::Turn(turn) => format!("{}turn", turn),
            }
        )
    }
}

impl Parse for Angle {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        input.skip_whitespace();
        let start = input.current_source_location();
        match input.next() {
            Ok(Token::Dimension {
                has_sign,
                value,
                int_value,
                unit,
            }) => match unit.as_ref() {
                "deg" => Ok(Angle::Deg(*value)),
                "grad" => Ok(Angle::Grad(*value)),
                "rad" => Ok(Angle::Rad(*value)),
                "turn" => Ok(Angle::Turn(*value)),
                _ => Err(start.new_custom_error(StyleParseError::ExpectedAngle)),
            },
            _ => Err(start.new_custom_error(StyleParseError::ExpectedAngle)),
        }
    }
}

/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/length
#[derive(Debug, Default, Clone)]
pub enum Length {
    #[default]
    Zero,
    // Relative to Font
    Cap(f32),
    Ch(f32),
    Rem(f32),
    Em(f32),
    Ex(f32),
    Ic(f32),
    Lh(f32),
    Rlh(f32),

    // Relative to Viewport
    Vh(f32),
    Vw(f32),
    Vmax(f32),
    Vmin(f32),
    Vb(f32),
    Vi(f32),

    // Small
    Svw(f32),
    Svh(f32),
    Svmin(f32),
    Svmax(f32),
    Svb(f32),
    Svi(f32),

    // Large
    Lvw(f32),
    Lvh(f32),
    Lvmin(f32),
    Lvmax(f32),
    Lvb(f32),
    Lvi(f32),

    // Dynamic
    Dvw(f32),
    Dvh(f32),
    Dvmin(f32),
    Dvmax(f32),
    Dvb(f32),
    Dvi(f32),

    // Relative to Container
    Cqw(f32),
    Cqh(f32),
    Cqb(f32),
    Cqi(f32),
    Cqmin(f32),
    Cqmax(f32),

    // Absolute
    Px(f32),
    Cm(f32),
    Mm(f32),
    Q(f32),
    In(f32),
    Pc(f32),
    Pt(f32),
}

impl Parse for Length {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.next() {
            Ok(Token::Dimension {
                has_sign,
                value,
                int_value,
                unit,
            }) => match unit.as_ref() {
                "cap" => Ok(Length::Cap(*value)),
                "ch" => Ok(Length::Ch(*value)),
                "rem" => Ok(Length::Rem(*value)),
                "em" => Ok(Length::Em(*value)),
                "ex" => Ok(Length::Ex(*value)),
                "ic" => Ok(Length::Ic(*value)),
                "lh" => Ok(Length::Lh(*value)),
                "rlh" => Ok(Length::Rlh(*value)),
                "vh" => Ok(Length::Vh(*value)),
                "vw" => Ok(Length::Vw(*value)),
                "vmax" => Ok(Length::Vmax(*value)),
                "vmin" => Ok(Length::Vmin(*value)),
                "vb" => Ok(Length::Vb(*value)),
                "vi" => Ok(Length::Vi(*value)),
                "svw" => Ok(Length::Svw(*value)),
                "svh" => Ok(Length::Svh(*value)),
                "svmin" => Ok(Length::Svmin(*value)),
                "svmax" => Ok(Length::Svmax(*value)),
                "svb" => Ok(Length::Svb(*value)),
                "svi" => Ok(Length::Svi(*value)),
                "lvw" => Ok(Length::Lvw(*value)),
                "lvh" => Ok(Length::Lvh(*value)),
                "lvmin" => Ok(Length::Lvmin(*value)),
                "lvmax" => Ok(Length::Lvmax(*value)),
                "lvb" => Ok(Length::Lvb(*value)),
                "lvi" => Ok(Length::Lvi(*value)),
                "dvw" => Ok(Length::Dvw(*value)),
                "dvh" => Ok(Length::Dvh(*value)),
                "dvmin" => Ok(Length::Dvmin(*value)),
                "dvmax" => Ok(Length::Dvmax(*value)),
                "dvb" => Ok(Length::Dvb(*value)),
                "dvi" => Ok(Length::Dvi(*value)),
                "cqw" => Ok(Length::Cqw(*value)),
                "cqh" => Ok(Length::Cqh(*value)),
                "cqb" => Ok(Length::Cqb(*value)),
                "cqi" => Ok(Length::Cqi(*value)),
                "cqmin" => Ok(Length::Cqmin(*value)),
                "cqmax" => Ok(Length::Cqmax(*value)),
                "px" => Ok(Length::Px(*value)),
                "cm" => Ok(Length::Cm(*value)),
                "mm" => Ok(Length::Mm(*value)),
                "q" => Ok(Length::Q(*value)),
                "in" => Ok(Length::In(*value)),
                "pc" => Ok(Length::Pc(*value)),
                "pt" => Ok(Length::Pt(*value)),
                _ => Err(input.new_custom_error(StyleParseError::ExpectedKeywords(vec![
                    "cap", "ch", "rem", "em", "ex", "ic", "lh", "rlh", "vh", "vw", "vmax", "vmin",
                    "vb", "vi", "svw", "svh", "svmin", "svmax", "svb", "svi", "lvw", "lvh",
                    "lvmin", "lvmax", "lvb", "lvi", "dvw", "dvh", "dvmin", "dvmax", "dvb", "dvi",
                    "cqw", "cqh", "cqb", "cqi", "cqmin", "cqmax", "px", "cm", "mm", "q", "in",
                    "pc", "pt",
                ]))),
            },
            Ok(Token::Number { value, .. }) if value == &0. => Ok(Length::Zero),
            _ => Err(input.new_custom_error(StyleParseError::ExpectedZero)),
        }
    }
}
