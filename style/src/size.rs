//! Size is a collection of values for shorthand css.
//!
//! Currently this applies to `inset`, `padding`, and `margin`. This data type is
//! used to old top, right, bottom, and left values for shorthand css. There are
//! also helpers to convert to a Size object from a list of values, max of 4, and
//! from single unit values.

use cssparser::{BasicParseError, ParseError, Parser, Token};

use crate::Unit;

/// A sizing shorthand for `inset`, `padding`, and `margin`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Size {
    pub left: Unit,
    pub top: Unit,
    pub right: Unit,
    pub bottom: Unit,
}

impl From<Vec<Unit>> for Size {
    fn from(value: Vec<Unit>) -> Self {
        match value.len() {
            1 => Size {
                top: value[0],
                right: value[0],
                bottom: value[0],
                left: value[0],
            },
            2 => Size {
                top: value[0],
                right: value[1],
                bottom: value[0],
                left: value[1],
            },
            3 => Size {
                top: value[0],
                right: value[1],
                bottom: value[2],
                left: value[1],
            },
            4 => Size {
                top: value[0],
                right: value[1],
                bottom: value[2],
                left: value[3],
            },
            _ => Size::default(),
        }
    }
}

impl From<Unit> for Size {
    fn from(value: Unit) -> Self {
        Size {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size {
            left: Unit::default(),
            top: Unit::default(),
            right: Unit::default(),
            bottom: Unit::default(),
        }
    }
}

impl Size {
    pub fn new(left: Unit, top: Unit, right: Unit, bottom: Unit) -> Self {
        Size {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Calculate the i32 values for top, right, bottom, left in that order.
    ///
    /// Provided a total with and total height the final values are calculated
    pub fn calc(&self, width: i32, height: i32) -> (i32, i32, i32, i32) {
        (
            self.top.as_i32(height, 0),
            self.right.as_i32(width, 0),
            self.bottom.as_i32(height, 0),
            self.left.as_i32(width, 0),
        )
    }

    /// Create a block shorthand size
    ///
    /// This refers to styles like `padding-block`. Left and right values
    /// are assigned and top and bottom are set to zero.
    pub fn block(size: Unit) -> Self {
        Size {
            left: size.clone(),
            top: Unit::default(),
            right: size,
            bottom: Unit::default(),
        }
    }

    /// Create a inline shorthand size
    ///
    /// This refers to styles like `padding-inline`. Top and bottom values
    /// are assigned and left and right are set to zero.
    pub fn inline(size: Unit) -> Self {
        Size {
            top: size.clone(),
            left: Unit::default(),
            bottom: size,
            right: Unit::default(),
        }
    }

    pub fn parse_with<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Size, ParseError<'i, BasicParseError<'i>>> {
        let mut padding: Vec<Unit> = Vec::new();
        padding.push(parse_value(input)?);
        for _ in 0..3 {
            match parse_value(input) {
                Ok(unit) => {
                    padding.push(unit);
                }
                _ => break,
            }
        }

        Ok(Size::from(padding))
    }

    /// Parse a <size shorthand> value, per CSS Module Level 3.
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BasicParseError<'i>> {
        Self::parse_with(input).map_err(ParseError::basic)
    }
}

fn parse_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Unit, BasicParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;

    match token {
        Token::Number { value, .. } => Ok(Unit::PX(*value)),
        Token::Dimension { value, unit, .. } => Ok(Unit::from_unit(unit, value)),
        Token::Percentage { unit_value, .. } => Ok(Unit::Percent(*unit_value)),
        _ => Err(location.new_basic_unexpected_token_error(token.clone())),
    }
}
