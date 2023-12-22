use crate::parser::error::StyleParseError;
use crate::parser::types::base::Length;
use crate::parser::types::or::{GlobalOr, PercentOr};
use crate::parser::Parse;
use cssparser::{BasicParseErrorKind, ParseError, ParseErrorKind, Parser, Token};
use crate::parser::types::color::Alpha::Percentage;

#[derive(Debug, Default, Clone)]
pub struct Radius {
    pub horizontal: PercentOr<Length>,
    pub vertical: PercentOr<Length>,
}

impl Parse for Radius {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        Err(input.new_custom_error(StyleParseError::UnkownSyntax))
    }
}

/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius
#[derive(Debug, Default)]
pub struct BorderRadius {
    top_left: GlobalOr<Radius>,
    top_right: GlobalOr<Radius>,
    bottom_right: GlobalOr<Radius>,
    bottom_left: GlobalOr<Radius>
}

fn spread(mut values: Vec<PercentOr<Length>>) -> Vec<PercentOr<Length>> {
    if values.len() == 1 {
        (0..4).map(|_| values[0].clone()).collect()
    } else if values.len() == 2 {
        vec![values[0].clone(), values[1].clone(), values[0].clone(), values[1].clone()]
    } else if values.len() == 3 {
        values.insert(2, values[1].clone());
        values
    } else {
        values
    }
}

impl Parse for BorderRadius {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let mut border_radius = BorderRadius::default();

        let mut first = Vec::new();
        let mut second = Vec::new();
        let mut start = input.state();
        match input.next() {
            Ok(Token::Ident(value)) => {
                input.reset(&start);

                let global = GlobalOr::<Radius>::parse(input)?;
                border_radius.top_left = global.clone();
                border_radius.top_right = global.clone();
                border_radius.bottom_right = global.clone();
                border_radius.bottom_left = global.clone();
            },
            Ok(Token::Dimension {..}) | Ok(Token::Percentage {..}) | Ok(Token::Number {..}) => {
                input.reset(&start);
                let mut slashed = false;
                loop {
                    match input.next() {
                        Ok(Token::Percentage { unit_value, .. }) => {
                            if slashed {
                                second.push(PercentOr::Percent(unit_value.into()));
                            } else {
                                first.push(PercentOr::Percent(unit_value.into()));
                            }
                        },
                        Ok(Token::Number { value, .. }) if value == &0. => {
                            if slashed {
                                second.push(PercentOr::Or(Length::Zero));
                            } else {
                                first.push(PercentOr::Or(Length::Zero));
                            }
                        },
                        Ok(Token::Delim('/')) if !slashed => slashed = true,
                        Err(_) => {
                            break;
                        },
                        _ => return Err(input.new_custom_error(StyleParseError::ExpectedLengthOrPercent))
                    }
                }
            }
            _ => return Err(input.new_custom_error(StyleParseError::ExpectedLengthOrPercent))
        }

        if first.len() == 0 || first.len() > 4 || second.len() > 4 {
            return Err(input.new_custom_error(StyleParseError::RangeAllowedItems{min: 1, max: 4}));
        }

        let first = spread(first);
        let mut second = spread(second);
        if second.len() == 0 {
            second = first.clone();
        }

        if let GlobalOr::Or(radius) = &mut border_radius.top_left {
            radius.horizontal = first[0].clone();
            radius.vertical = second[0].clone();
        }
        if let GlobalOr::Or(radius) = &mut border_radius.top_right {
            radius.horizontal = first[1].clone();
            radius.vertical = second[1].clone();
        }
        if let GlobalOr::Or(radius) = &mut border_radius.bottom_right {
            radius.horizontal = first[2].clone();
            radius.vertical = second[2].clone();
        }
        if let GlobalOr::Or(radius) = &mut border_radius.bottom_left {
            radius.horizontal = first[3].clone();
            radius.vertical = second[3].clone();
        }
        Ok(border_radius)
    }
}
