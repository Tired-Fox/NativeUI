use crate::parser::stylesheet::StyleParseError;
use crate::parser::types::base::{Angle, Percent};
use crate::parser::Parse;
use cssparser::{ParseError, Parser, Token};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum PercentOr<T: Debug> {
    Percent(Percent),
    Or(T),
}

impl<T: Debug + Display> Display for PercentOr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PercentOr::Percent(p) => write!(f, "{}%", p),
            PercentOr::Or(t) => write!(f, "{}", t),
        }
    }
}

impl<T: Parse + Debug + Clone> Clone for PercentOr<T> {
    fn clone(&self) -> Self {
        match self {
            PercentOr::Percent(p) => PercentOr::Percent(*p),
            PercentOr::Or(t) => PercentOr::Or(t.clone()),
        }
    }
}

impl<T: Parse + Debug + Default> Default for PercentOr<T> {
    fn default() -> Self {
        PercentOr::Or(T::default())
    }
}

impl<T: Parse + Debug> Parse for PercentOr<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let start = input.state();
        match input.next() {
            Ok(Token::Percentage { unit_value, .. }) => return Ok(PercentOr::Percent(unit_value.into())),
            _ => {
                input.reset(&start);
            }
        }
        Ok(PercentOr::Or(T::parse(input)?))
    }
}

#[derive(Debug)]
pub enum GlobalOr<T: Debug> {
    Inherit,
    Initial,
    Revert,
    RevertLayer,
    Unset,
    Or(T),
}

impl <T: Debug> From<&str> for GlobalOr<T> {
    fn from(value: &str) -> Self {
        match value {
            "inherit" => GlobalOr::Inherit,
            "initial" => GlobalOr::Initial,
            "revert" => GlobalOr::Revert,
            "revert-layer" => GlobalOr::RevertLayer,
            "unset" => GlobalOr::Unset,
            _ => panic!("Invalid global property value")
        }
    }
}

impl<T: Debug + Display> Display for GlobalOr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalOr::Inherit => write!(f, "inherit"),
            GlobalOr::Initial => write!(f, "initial"),
            GlobalOr::Revert => write!(f, "revert"),
            GlobalOr::RevertLayer => write!(f, "revert-layer"),
            GlobalOr::Unset => write!(f, "unset"),
            GlobalOr::Or(t) => write!(f, "{}", t),
        }
    }
}

impl<T: Debug + Default> Default for GlobalOr<T> {
    fn default() -> Self {
        GlobalOr::Or(T::default())
    }
}

impl<T: Parse + Debug> Parse for GlobalOr<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let start = input.state();
        match input.next() {
            Ok(Token::Ident(value)) => match value.to_ascii_lowercase().as_str() {
                "inherit" => return Ok(GlobalOr::Inherit),
                "initial" => return Ok(GlobalOr::Initial),
                "revert" => return Ok(GlobalOr::Revert),
                "revert-layer" => return Ok(GlobalOr::RevertLayer),
                "unset" => return Ok(GlobalOr::Unset),
                _ => {
                    input.reset(&start);
                },
            },
            _ => {
                input.reset(&start);
            }
        }
        Ok(GlobalOr::Or(T::parse(input)?))
    }
}

impl<T: Parse + Debug + Clone> Clone for GlobalOr<T> {
    fn clone(&self) -> Self {
        match self {
            GlobalOr::Inherit => GlobalOr::Inherit,
            GlobalOr::Initial => GlobalOr::Initial,
            GlobalOr::Revert => GlobalOr::Revert,
            GlobalOr::RevertLayer => GlobalOr::RevertLayer,
            GlobalOr::Unset => GlobalOr::Unset,
            GlobalOr::Or(t) => GlobalOr::Or(t.clone()),
        }
    }
}

#[derive(Debug)]
pub enum AutoOr<T: Debug> {
    Auto,
    Or(T),
}

impl<T: Debug + Display> Display for AutoOr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoOr::Auto => write!(f, "auto"),
            AutoOr::Or(t) => write!(f, "{}", t),
        }
    }
}

impl<T: Debug + Default> Default for AutoOr<T> {
    fn default() -> Self {
        AutoOr::Or(T::default())
    }
}

impl<T: Parse + Debug> Parse for AutoOr<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let start = input.state();
        match input.next() {
            Ok(Token::Ident(value)) => match value.to_ascii_lowercase().as_str() {
                "auto" => return Ok(AutoOr::Auto),
                _ => {
                    input.reset(&start);
                }
            },
            _ => {
                input.reset(&start);
            }
        }
        Ok(AutoOr::Or(T::parse(input)?))
    }
}

impl<T: Parse + Debug + Clone> Clone for AutoOr<T> {
    fn clone(&self) -> Self {
        match self {
            AutoOr::Auto => AutoOr::Auto,
            AutoOr::Or(t) => AutoOr::Or(t.clone()),
        }
    }
}

#[derive(Debug)]
pub enum NoneOr<T: Debug> {
    None,
    Or(T),
}

impl<T: Debug + Display> Display for NoneOr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoneOr::None => write!(f, "none"),
            NoneOr::Or(t) => write!(f, "{}", t),
        }
    }
}

impl<T: Debug + Default> Default for NoneOr<T> {
    fn default() -> Self {
        NoneOr::Or(T::default())
    }
}

impl<T: Parse + Debug> Parse for NoneOr<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let start = input.state();
        match input.next() {
            Ok(Token::Ident(value)) => match value.to_ascii_lowercase().as_str() {
                "none" => return Ok(NoneOr::None),
                _ => {
                    input.reset(&start);
                }
            },
            _ => {
                input.reset(&start);
            }
        }
        Ok(NoneOr::Or(T::parse(input)?))
    }
}

impl<T: Parse + Debug + Clone> Clone for NoneOr<T> {
    fn clone(&self) -> Self {
        match self {
            NoneOr::None => NoneOr::None,
            NoneOr::Or(t) => NoneOr::Or(t.clone()),
        }
    }
}

#[derive(Debug)]
pub enum Either<E1: Debug, E2: Debug> {
    Either(E1),
    Or(E2),
}

impl<E1: Debug + Display, E2: Debug + Display> Display for Either<E1, E2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Either::Either(value) => value.to_string(),
            Either::Or(value) => value.to_string(),
        })
    }
}

impl<E1: Parse + Debug, E2: Parse + Debug> Parse for Either<E1, E2> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let start = input.state();
        match E1::parse(input) {
            Ok(e1) => return Ok(Either::Either(e1)),
            Err(_) => {
                input.reset(&start);
                Ok(Either::Or(E2::parse(input)?))
            }
        }
    }
}

impl<E1: Debug + Clone, E2: Debug + Clone> Clone for Either<E1, E2> {
    fn clone(&self) -> Self {
        match self {
            Either::Either(value) => Either::Either(value.clone()),
            Either::Or(value) => Either::Or(value.clone()),
        }
    }
}
