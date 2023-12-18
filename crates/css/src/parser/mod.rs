//! Example syntax from [Parcel CSS](https://parceljs.org/blog/parcel-css/):
//!
//! ```
//! Background([Background {
//!   image: Url(Url { url: "img.png" }),
//!   color: CssColor(RGBA(RGBA { red: 0, green: 0, blue: 0, alpha: 0 })),
//!   position: Position {
//!     x: Length(Dimension(Px(20.0))),
//!     y: Length(Dimension(Px(10.0))),
//!   },
//!   repeat: BackgroundRepeat {
//!     x: Repeat,
//!     y: Repeat,
//!   },
//!   size: Explicit {
//!     width: LengthPercentage(Dimension(Px(50.0))),
//!     height: LengthPercentage(Dimension(Px(100.0))),
//!   },
//!   attachment: Scroll,
//!   origin: PaddingBox,
//!   clip: BorderBox,
//! }])
//! ```
//!
//! Goal Strict Typing and Objects
use crate::parser::stylesheet::StyleParseError;
use cssparser::{ParseError, Parser, ParseErrorKind};

mod at_rule;
mod color;
mod decleration;
mod nested;
pub mod selector;
pub mod stylesheet;

pub trait Parse
where
    Self: Sized,
{
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>>;
}

impl Parse for f32 {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.expect_number() {
            Ok(num) => Ok(num),
            Err(err) => {
                Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                    location: input.current_source_location()
                })
            }
        }
    }
}

impl Parse for i32 {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.next() {
            Ok(cssparser::Token::Number { has_sign, value, int_value }) => {
                if value < &(i32::MIN as f32) || int_value.is_none() || value > &(i32::MAX as f32) {
                    return Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                        location: input.current_source_location()
                    });
                }

                return Ok(int_value.unwrap() as i32)
            }
            _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                        location: input.current_source_location()
                    });
            }
        }
    }
}

impl Parse for u8 {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.next() {
            Ok(cssparser::Token::Number { has_sign, value, int_value }) => {
                if value < &(u8::MIN as f32) || int_value.is_none() || value > &(u8::MAX as f32) {
                    return Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                        location: input.current_source_location()
                    });
                }

                return Ok(int_value.unwrap() as u8)
            }
            _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                        location: input.current_source_location()
                    });
            }
        }
    }
}
