use crate::parser::stylesheet::StyleParseError;
use crate::parser::Parse;
use cssparser::{ParseError, ParseErrorKind, Parser, Token};
use std::fmt::{Debug, Display};

use super::{
    base::{Angle, Number, Percent},
    or::{Either, NoneOr, PercentOr},
};

use color_space::*;
use named::*;
use system::*;

mod color_space;
mod named;
mod system;

/// Alpha or Transparency amount
///
/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/alpha-value
#[derive(Debug)]
pub enum Alpha {
    Number(Number),
    Percentage(Percent),
}

impl Parse for Alpha {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.next() {
            Ok(Token::Percentage { unit_value, .. }) => Ok(Alpha::Percentage(unit_value.into())),
            Ok(Token::Number { value, .. }) => Ok(Alpha::Number(value.into())),
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                location: input.current_source_location(),
            }),
        }
    }
}

impl Display for Alpha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Alpha::Number(num) => num.to_string(),
                Alpha::Percentage(perc) => format!("{}%", perc),
            }
        )
    }
}

/// Color reprecents a color value with an optional alpha channel which inidicates how the color
/// should composit with its background.
///
/// # Includes
/// - [Transparent](Color::Transparent)
/// - [Named](Named)
/// - [System](System)
/// - [CurrentColor](Color::CurrentColor)
/// - [Hex](Color::HEX)
/// - [RGB](Color::RGB)
/// - [HSL](Color::HSL)
/// - [HWB](Color::HWB)
/// - [Lab](Color::Lab)
/// - [Lch](Color::Lch)
/// - [Color](Color::Color)
/// - [ColorMix](Color::ColorMix)
/// - [LightDark](Color::LightDark)
///
/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
#[derive(Debug)]
pub enum Color {
    /// Transparent color
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
    Transparent,
    /// Predefined named color
    Named(Named),
    /// Default color for parts of the page
    System(System),
    /// Current color
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value#currentcolor_keyword
    CurrentColor,
    /// Hex Color
    /// # Format
    /// - RGB
    /// - RGBA
    /// - RRGGBB
    /// - RRGGBBAA
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color
    HEX {
        red: u8,
        green: u8,
        blue: u8,
        alpha: Option<u8>,
    },
    /// RGB Color
    /// # Format
    /// - rgb(r g b)
    /// - rgba(r g b / a)
    ///
    /// _**Legacy**_
    /// - rgb(r, g, b)
    /// - rgba(r, g, b, a)
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/rgb
    RGB {
        red: NoneOr<PercentOr<Number>>,
        green: NoneOr<PercentOr<Number>>,
        blue: NoneOr<PercentOr<Number>>,
        alpha: Option<NoneOr<Alpha>>,
    },
    /// HSL Color
    /// # Format
    /// - hsl(h s l)
    /// - hsl(h s l / a)
    ///
    /// _**Legacy**_
    /// - hsl(h, s, l)
    /// - hsl(h, s, l, a)
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/hsl
    HSL {
        hue: NoneOr<Either<Angle, Number>>,
        saturation: NoneOr<Percent>,
        lightness: NoneOr<Percent>,
        alpha: Option<NoneOr<Alpha>>,
    },
    /// HWB Color
    /// # Format
    /// - hwb(h w b)
    /// - hwb(h w b / a)
    ///
    /// _**Legacy**_
    /// - hwb(h, w, b)
    /// - hwb(h, w, b, a)
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/hwb
    HWB {
        hue: NoneOr<Either<Angle, Number>>,
        whiteness: NoneOr<Percent>,
        blackness: NoneOr<Percent>,
        alpha: Option<NoneOr<Alpha>>,
    },
    /// (OK)LAB Color
    /// # Format
    /// - lab(l a b)
    /// - lab(l a b / a)
    /// - oklab(l a b)
    /// - oklab(l a b / a)
    ///
    /// _**Legacy**_
    /// - lab(l, a, b)
    /// - lab(l, a, b, a)
    /// - oklab(l, a, b)
    /// - oklab(l, a, b, a)
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/lab
    /// Reference(OK): https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklab
    LAB {
        ok: bool,
        lightness: NoneOr<PercentOr<Number>>,
        a_axis: NoneOr<PercentOr<Number>>,
        b_axis: NoneOr<PercentOr<Number>>,
        alpha: Option<NoneOr<Alpha>>,
    },
    /// (OK)LCH Color
    /// # Format
    /// - lch(l c h)
    /// - lch(l c h / a)
    /// - oklch(l c h)
    /// - oklch(l c h / a)
    ///
    /// _**Legacy**_
    /// - lch(l, c, h)
    /// - lch(l, c, h, a)
    /// - oklch(l, c, h)
    /// - oklch(l, c, h, a)
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/lch
    /// Reference(OK): https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklch
    LCH {
        ok: bool,
        lightness: NoneOr<PercentOr<Number>>,
        chroma: NoneOr<PercentOr<Number>>,
        hue: NoneOr<Either<Angle, Number>>,
        alpha: Option<NoneOr<Alpha>>,
    },
    /// Color with `ColorSpace`
    /// # Format
    /// - color([color_space](ColorSpace) c1 c2 c3)
    /// - color([color_space](ColorSpace) c1 c2 c3 / a)
    ///
    /// _**Legacy**_
    /// - color([color_space](ColorSpace), c1, c2, c3)
    /// - color([color_space](ColorSpace), c1, c2, c3, a)
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/color
    Color {
        color_space: ColorSpace,
        c1: NoneOr<PercentOr<Number>>,
        c2: NoneOr<PercentOr<Number>>,
        c3: NoneOr<PercentOr<Number>>,
        alpha: Option<NoneOr<Alpha>>,
    },
    /// Mix two colors
    /// # Format
    ///
    /// color-mix(
    ///     [<color-interpolation-method>](ColorInterpolationMethod),
    ///     [\<color\>](Color) [\<p1\>](Percent)?,
    ///     [\<color\>](Color) [\<p2\>](Percent)?
    /// )
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/color-mix
    ColorMix {
        method: ColorInterpolationMethod,
        color1: Box<Color>,
        color2: Box<Color>,
        p1: Option<Percent>,
        p2: Option<Percent>,
    },
    /// Light or Dark color
    /// # Format
    /// - light-dark([light](Color), [dark](Color))
    ///
    /// note: Requires that `color-scheme: light dark` is set
    ///
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/light-dark
    LightDark { light: Box<Color>, dark: Box<Color> },
}

fn parse_with_opt_comma<'i, 't, T: Parse + Debug>(
    input: &mut Parser<'i, 't>,
) -> Result<T, ParseError<'i, StyleParseError>> {
    let result = T::parse(input)?;
    let _ = input.try_parse(|i| i.expect_comma());
    Ok(result)
}
impl Parse for Color {
    fn parse<'i, 't>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, StyleParseError>> {
        let next = input.next();
        match next {
            Ok(Token::Ident(value)) => match value.to_ascii_lowercase().as_str() {
                "transparent" => Ok(Color::Transparent),
                "currentcolor" => Ok(Color::CurrentColor),
                _ => {
                    if let Some(named) = Named::parse(value.as_ref()) {
                        Ok(Color::Named(named))
                    } else if let Some(system) = System::parse(value.as_ref()) {
                        Ok(Color::System(system))
                    } else {
                        Err(ParseError {
                            kind: cssparser::ParseErrorKind::Custom(
                                StyleParseError::InvalidColorKeyword,
                            ),
                            location: input.current_source_location(),
                        })
                    }
                }
            },
            Ok(Token::IDHash(hex)) => {
                if hex.len() != 3 && hex.len() != 6 && hex.len() != 4 && hex.len() != 8 {
                    return Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                        location: input.current_source_location(),
                    });
                }

                let hex = match hex.len() {
                    3 => {
                        format!(
                            "{r}{r}{g}{g}{b}{b}",
                            r = hex.chars().nth(0).unwrap(),
                            g = hex.chars().nth(1).unwrap(),
                            b = hex.chars().nth(2).unwrap(),
                        )
                    }
                    4 => {
                        format!(
                            "{r}{r}{g}{g}{b}{b}{a}{a}",
                            r = hex.chars().nth(0).unwrap(),
                            g = hex.chars().nth(1).unwrap(),
                            b = hex.chars().nth(2).unwrap(),
                            a = hex.chars().nth(3).unwrap()
                        )
                    }
                    _ => hex.to_string(),
                };

                let hex = u32::from_str_radix(hex.as_str(), 16)
                    .map_err(|_| ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                        location: input.current_source_location(),
                    })?
                    .to_be_bytes();

                if hex.len() == 6 {
                    Ok(Color::HEX {
                        red: hex[1],
                        green: hex[2],
                        blue: hex[3],
                        alpha: None,
                    })
                } else {
                    Ok(Color::HEX {
                        red: hex[0],
                        green: hex[1],
                        blue: hex[2],
                        alpha: Some(hex[3]),
                    })
                }
            }
            Ok(Token::Function(name)) => match name.to_ascii_lowercase().as_str() {
                "rgb" | "rgba" => input.parse_nested_block(|i| {
                    Ok(Color::RGB {
                        red: parse_with_opt_comma(i)?,
                        green: parse_with_opt_comma(i)?,
                        blue: parse_with_opt_comma(i)?,
                        alpha: if !i.is_exhausted() {
                            if let Err(_) = i.try_parse(|i| i.expect_comma()) {
                                if let Err(_) = i.try_parse(|i| i.expect_delim('/')) {
                                    return Err(ParseError {
                                        kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                                        location: i.current_source_location(),
                                    });
                                }
                            }
                            Option::<NoneOr<Alpha>>::parse(i)?
                        } else {
                            None
                        },
                    })
                }),
                _ => {
                    println!("Invalid color function: {:?}", name);
                    Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::NotImplemented),
                        location: input.current_source_location(),
                    })
                }
            },
            Ok(color) => {
                println!("Invalid color ident: {:?}", color);
                Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::NotImplemented),
                    location: input.current_source_location(),
                })
            }
            Err(err) => {
                println!("[ColorError]: {:?}", err);
                Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::NotImplemented),
                    location: input.current_source_location(),
                })
            } // srgb
              // rgb function
              // rgba function
              // hsl function
              // color
              // hwb
              // lch
              // oklch
              // lab
              // oklab
              //
              // #rrggbb
              // #rrggbbaa
              // #rgba
              // hsla
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::System(system) => system.to_string(),
                Color::Named(named) => named.to_string(),
                Color::Transparent => "transparent".to_string(),
                Color::CurrentColor => "currentcolor".to_string(),
                Color::HEX {
                    red,
                    green,
                    blue,
                    alpha,
                } => {
                    let num = match alpha {
                        Some(alpha) => {
                            u32::from_be_bytes([*red, *green, *blue, *alpha])
                        },
                        None => {
                            u32::from_be_bytes([0, *red, *green, *blue])
                        }
                    };
                    format!("#{:x}", num)
                },
                Color::RGB {
                    red,
                    green,
                    blue,
                    alpha,
                } => {
                    let alpha_s = match alpha {
                        Some(alpha) => format!(" / {}", alpha),
                        None => String::new(),
                    };
                    format!("rgb({} {} {}{})", red, green, blue, alpha_s)
                }
                Color::HSL {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                } => {
                    {
                        let alpha = match alpha {
                            Some(alpha) => format!(" / {}", alpha),
                            None => String::new(),
                        };
                        format!("hsl({} {} {}{})", hue, saturation, lightness, alpha)
                    }
                }
                Color::HWB {
                    hue,
                    whiteness,
                    blackness,
                    alpha,
                } => {
                    {
                        let alpha = match alpha {
                            Some(alpha) => format!(" / {}", alpha),
                            None => String::new(),
                        };
                        format!("hwb({} {} {}{})", hue, whiteness, blackness, alpha)
                    }
                }
                Color::LAB {
                    ok,
                    lightness,
                    a_axis,
                    b_axis,
                    alpha,
                } => {
                    {
                        let alpha = match alpha {
                            Some(alpha) => format!(" / {}", alpha),
                            None => String::new(),
                        };
                        let ok = if *ok { "ok" } else { "" };
                        format!("{}lab({} {} {}{})", ok, lightness, a_axis, b_axis, alpha)
                    }
                }
                Color::LCH {
                    ok,
                    lightness,
                    chroma,
                    hue,
                    alpha,
                } => {
                    {
                        let alpha = match alpha {
                            Some(alpha) => format!(" / {}", alpha),
                            None => String::new(),
                        };
                        let ok = if *ok { "ok" } else { "" };
                        format!("{}lch({} {} {}{})", ok, lightness, chroma, hue, alpha)
                    }
                }
                Color::Color {
                    color_space,
                    c1,
                    c2,
                    c3,
                    alpha,
                } => {
                    {
                        let alpha = match alpha {
                            Some(alpha) => format!(" / {}", alpha),
                            None => String::new(),
                        };
                        format!("color({} {} {} {}{})", color_space, c1, c2, c3, alpha)
                    }
                }
                Color::ColorMix {
                    method,
                    color1,
                    color2,
                    p1,
                    p2,
                } => {
                    format!(
                        "color-mix({}, {}{}, {}{})",
                        method,
                        color1,
                        p1.map_or(String::new(), |f| format!("{}%", f)),
                        color2,
                        p2.map_or(String::new(), |f| format!("{}%", f))
                    )
                }
                Color::LightDark { light, dark } => format!("light-dark({}, {})", light, dark),
            }
        )
    }
}
