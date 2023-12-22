use crate::parser::error::StyleParseError;
use crate::parser::types::base::{Length, Percent};
use crate::parser::types::border::BorderRadius;
use crate::parser::types::or::{AutoOr, PercentOr};
use crate::parser::Parse;
use cssparser::{BasicParseError, ParseError, ParseErrorKind, Parser, Token};

/// [\<length\>](Length) | [\<percentage\>](Percent)
///
/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape
#[derive(Debug)]
pub enum ShapeArg {
    Length(Length),
    Percent(Percent),
}

/// [\<length\>](Length) | [\<percentage\>](Percent) | closest-side | farthest-side
///
/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape
#[derive(Debug)]
pub enum ShapeRadius {
    Length(Length),
    Percent(Percent),
    ClosestSide,
    FarthestSide,
}

/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape
#[derive(Debug)]
pub enum BasicShape {
    Inset {
        top: PercentOr<Length>,
        right: PercentOr<Length>,
        bottom: PercentOr<Length>,
        left: PercentOr<Length>,
        round: Option<BorderRadius>,
    },
    Rect {
        top: AutoOr<PercentOr<Length>>,
        right: AutoOr<PercentOr<Length>>,
        bottom: AutoOr<PercentOr<Length>>,
        left: AutoOr<PercentOr<Length>>,
        round: Option<BorderRadius>,
    },
    XYWH {
        x: PercentOr<Length>,
        y: PercentOr<Length>,
        width: PercentOr<Length>,
        height: PercentOr<Length>,
        round: Option<BorderRadius>,
    },
    Circle,
    Ellipse,
    Polygon,
    Path,
}

impl Parse for BasicShape {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.next() {
            Ok(Token::Function(name)) => match name.as_ref() {
                "inset" => input.parse_nested_block(|i| {
                    let top = PercentOr::<Length>::parse(i)?;
                    let right = PercentOr::<Length>::parse(i)?;
                    let bottom = PercentOr::<Length>::parse(i)?;
                    let left = PercentOr::<Length>::parse(i)?;
                    let mut round = None;

                    let _ = i.try_parse(|i| {
                        i.expect_ident_matching("round")
                            .map_err(|_| i.new_custom_error(StyleParseError::Unkown))?;

                        match BorderRadius::parse(i) {
                            Ok(result) => {
                                round = Some(result);
                                Ok(())
                            }
                            Err(err) => Err(err),
                        }
                    });

                    Ok(BasicShape::Inset {
                        top,
                        right,
                        bottom,
                        left,
                        round,
                    })
                }),
                "rect" => input.parse_nested_block(|i| {
                    let top = AutoOr::<PercentOr<Length>>::parse(i)?;
                    let right = AutoOr::<PercentOr<Length>>::parse(i)?;
                    let bottom = AutoOr::<PercentOr<Length>>::parse(i)?;
                    let left = AutoOr::<PercentOr<Length>>::parse(i)?;
                    let mut round = None;

                    let _ = i.try_parse(|i| {
                        i.expect_ident_matching("round")
                            .map_err(|_| i.new_custom_error(StyleParseError::Unkown))?;
                        match BorderRadius::parse(i) {
                            Ok(result) => {
                                round = Some(result);
                                Ok(())
                            }
                            Err(err) => Err(err),
                        }
                    });

                    Ok(BasicShape::Rect {
                        top,
                        right,
                        bottom,
                        left,
                        round,
                    })
                }),
                "xywh" => input.parse_nested_block(|i| {
                    let x = PercentOr::<Length>::parse(i)?;
                    let y = PercentOr::<Length>::parse(i)?;
                    let width = PercentOr::<Length>::parse(i)?;
                    let height = PercentOr::<Length>::parse(i)?;
                    let mut round = None;

                    let _ = i.try_parse(|i| {
                        i.expect_ident_matching("round")
                            .map_err(|_| i.new_custom_error(StyleParseError::Unkown))?;
                        match BorderRadius::parse(i) {
                            Ok(result) => {
                                round = Some(result);
                                Ok(())
                            }
                            Err(err) => Err(err),
                        }
                    });

                    Ok(BasicShape::XYWH {
                        x,
                        y,
                        width,
                        height,
                        round,
                    })
                }),
                _ => Err(
                    input.new_custom_error(StyleParseError::ExpectedFunctions(vec![
                        "inset", "rect", "xywh",
                    ])),
                ),
            },
            _ => Err(
                input.new_custom_error(StyleParseError::ExpectedFunctions(vec![
                    "inset", "rect", "xywh",
                ])),
            ),
        }
    }
}
