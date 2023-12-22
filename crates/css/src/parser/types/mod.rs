use cssparser::{ParseError, Parser};
use crate::parser::Parse;
use crate::parser::error::StyleParseError;

pub mod base;
pub mod color;
pub mod or;
pub mod shape;
pub mod border;
pub mod decleration;

pub use decleration::Declaration;

impl<T: Parse> Parse for Option<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        input.skip_whitespace();
        let start = input.state();
        match T::parse(input) {
            Ok(t) => Ok(Some(t)),
            Err(_) => {
                input.reset(&start);
                Ok(None)
            }
        }
    }
}
