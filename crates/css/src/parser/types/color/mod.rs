use crate::parser::error::StyleParseError;
use crate::parser::Parse;
use cssparser::{Delimiter, ParseError, ParseErrorKind, Parser, Token};
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
        input.skip_whitespace();
        let start = input.current_source_location();
        match input.next() {
            Ok(Token::Percentage { unit_value, .. }) => Ok(Alpha::Percentage(unit_value.into())),
            Ok(Token::Number { value, .. }) => Ok(Alpha::Number(value.into())),
            _ => Err(start.new_custom_error(StyleParseError::ExpectedNumberOrPercent)),
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
                Alpha::Percentage(perc) => perc.to_string(),
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
    /// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/lab
    /// Reference(OK): https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklab
    LAB {
        ok: bool,
        lightness: NoneOr<PercentOr<Number>>,
        a: NoneOr<PercentOr<Number>>,
        b: NoneOr<PercentOr<Number>>,
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

/// For parsing RGB and HSL modern and legacy syntax since they are similar.
///
/// Technically rgb and hsl legacy syntax have all values being consistant while this will
/// parse each value as it's modern syntax with comma seperation. This is known and allows for more lenient legacy
/// css syntax, but this is more or less to support users writing legacy syntax of rgb and hsl not validating that they are
/// 100% correct.
///
/// # Modern RGB
/// - rgb|rgba: `rgb([<number>|<percent>|none]#{3} [ / [<alpha-value> | None]]?)`
///
/// # Legacy RGB
/// - rgb|rgba: `rgb(<number>#{3}, <alplha-value>?)`
/// - rgb|rgba: `rgb(<percent>#{3}, <alplha-value>?)`
///
/// # Modern HSL
/// - hsl|hsla: `hsl([<hue> | none] [<number>|<percent>|none]{2} [ / [<alpha-value> | None]]?)`
///
/// # Legacy HSL
/// - hsl|hsla: `hsl(<hue>, <percent>, <percent>, <alplha-value>?)`
fn parse_color_with_legacy<'i, 't, A1: Parse, A2: Parse, A3: Parse, A4: Parse>(
    input: &mut Parser<'i, 't>,
) -> Result<(A1, A2, A3, Option<A4>), ParseError<'i, StyleParseError>> {
    let a1 = A1::parse(input)?;
    let bfr = input.state();
    match input.expect_comma() {
        Ok(_) => {
            let a2 = input.parse_until_before(Delimiter::Comma, |i| {
                // parse
                let result = A2::parse(i)?;
                i.skip_whitespace();
                if let Err(err) = i.expect_exhausted() {
                    return Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::InvalidArgument),
                        location: err.location,
                    });
                }
                Ok(result)
            })?;
            input.skip_whitespace();
            if let Err(err) = input.expect_comma() {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::InvalidArgument),
                    location: err.location,
                });
            }
            let a3 = A3::parse(input)?;

            let bfr = input.state();
            let a4 = if let Ok(_) = input.expect_comma() {
                Option::<A4>::parse(input)?
            } else {
                input.reset(&bfr);
                None
            };

            input.skip_whitespace();
            if let Err(err) = input.expect_exhausted() {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::InvalidArgument),
                    location: err.location,
                });
            }
            Ok((a1, a2, a3, a4))
        }
        Err(_) => {
            input.reset(&bfr);
            let a2 = A2::parse(input)?;
            let a3 = A3::parse(input)?;
            let bfr = input.state();
            let a4 = if let Ok(_) = input.expect_delim('/') {
                Some(A4::parse(input)?)
            } else {
                input.reset(&bfr);
                None
            };

            input.skip_whitespace();
            if let Err(err) = input.expect_exhausted() {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::InvalidArgument),
                    location: err.location,
                });
            }
            Ok((a1, a2, a3, a4))
        }
    }
}

impl Parse for Color {
    fn parse<'i, 't>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, StyleParseError>> {
        input.skip_whitespace();
        let begin = input.current_source_location();
        match input.next() {
            Ok(Token::Ident(value)) => match value.to_ascii_lowercase().as_str() {
                "transparent" => Ok(Color::Transparent),
                "currentcolor" => Ok(Color::CurrentColor),
                _ => {
                    if let Some(named) = Named::parse(value.as_ref()) {
                        Ok(Color::Named(named))
                    } else if let Some(system) = System::parse(value.as_ref()) {
                        Ok(Color::System(system))
                    } else {
                        Err(begin.new_custom_error(StyleParseError::InvalidColorKeyword))
                    }
                }
            },
            Ok(Token::IDHash(hex)) | Ok(Token::Hash(hex)) => {
                if hex.len() != 3 && hex.len() != 6 && hex.len() != 4 && hex.len() != 8 {
                    return Err(begin.new_custom_error(StyleParseError::InvalidHexFormat));
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
                    .map_err(|_| begin.new_custom_error(StyleParseError::InvalidHexFormat))?
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
                    let parts = parse_color_with_legacy(i)?;
                    Ok(Color::RGB {
                        red: parts.0,
                        green: parts.1,
                        blue: parts.2,
                        alpha: parts.3,
                    })
                }),
                "hsl" | "hsla" => input.parse_nested_block(|i| {
                    let parts = parse_color_with_legacy(i)?;
                    Ok(Color::HSL {
                        hue: parts.0,
                        saturation: parts.1,
                        lightness: parts.2,
                        alpha: parts.3,
                    })
                }),
                "hwb" => input.parse_nested_block(|i| {
                    Ok(Color::HWB {
                        hue: NoneOr::<Either<Angle, Number>>::parse(i)?,
                        whiteness: NoneOr::<Percent>::parse(i)?,
                        blackness: NoneOr::<Percent>::parse(i)?,
                        alpha: if !i.is_exhausted() {
                            if let Err(_) = i.try_parse(|i| i.expect_delim('/')) {
                                return Err(i.new_custom_error(StyleParseError::InvalidArgument));
                            }
                            Some(NoneOr::<Alpha>::parse(i)?)
                        } else {
                            None
                        },
                    })
                }),
                "oklab" | "lab" => {
                    let ok = name.to_ascii_lowercase().starts_with("ok");
                    input.parse_nested_block(|i| {
                        Ok(Color::LAB {
                            ok,
                            lightness: NoneOr::<PercentOr<Number>>::parse(i)?,
                            a: NoneOr::<PercentOr<Number>>::parse(i)?,
                            b: NoneOr::<PercentOr<Number>>::parse(i)?,
                            alpha: if !i.is_exhausted() {
                                if let Err(_) = i.try_parse(|i| i.expect_delim('/')) {
                                    return Err(
                                        i.new_custom_error(StyleParseError::InvalidArgument)
                                    );
                                }
                                Some(NoneOr::<Alpha>::parse(i)?)
                            } else {
                                None
                            },
                        })
                    })
                }
                "oklch" | "lch" => {
                    let ok = name.to_ascii_lowercase().starts_with("ok");
                    input.parse_nested_block(|i| {
                        Ok(Color::LCH {
                            ok,
                            lightness: NoneOr::<PercentOr<Number>>::parse(i)?,
                            chroma: NoneOr::<PercentOr<Number>>::parse(i)?,
                            hue: NoneOr::<Either<Angle, Number>>::parse(i)?,
                            alpha: if !i.is_exhausted() {
                                if let Err(_) = i.try_parse(|i| i.expect_delim('/')) {
                                    return Err(
                                        i.new_custom_error(StyleParseError::InvalidArgument)
                                    );
                                }
                                Some(NoneOr::<Alpha>::parse(i)?)
                            } else {
                                None
                            },
                        })
                    })
                }
                "color" => input.parse_nested_block(|i| {
                    Ok(Color::Color {
                        color_space: ColorSpace::parse(i)?,
                        c1: NoneOr::<PercentOr<Number>>::parse(i)?,
                        c2: NoneOr::<PercentOr<Number>>::parse(i)?,
                        c3: NoneOr::<PercentOr<Number>>::parse(i)?,
                        alpha: if !i.is_exhausted() {
                            if let Err(_) = i.try_parse(|i| i.expect_delim('/')) {
                                return Err(i.new_custom_error(StyleParseError::InvalidArgument));
                            }
                            Some(NoneOr::<Alpha>::parse(i)?)
                        } else {
                            None
                        },
                    })
                }),
                "color-mix" => input.parse_nested_block(|i| {
                    let method = i.parse_until_before(Delimiter::Comma, |i| {
                        ColorInterpolationMethod::parse(i)
                    })?;
                    let _ = i.expect_comma()?;
                    let c1 = {
                        let result = (Color::parse(i)?, Option::<Percent>::parse(i)?);
                        let _ = i.expect_comma()?;
                        result
                    };
                    let c2 = (Color::parse(i)?, Option::<Percent>::parse(i)?);
                    Ok(Color::ColorMix {
                        method,
                        color1: Box::new(c1.0),
                        p1: c1.1,
                        color2: Box::new(c2.0),
                        p2: c2.1,
                    })
                }),
                "light-dark" => input.parse_nested_block(|i| {
                    Ok(Color::LightDark {
                        light: {
                            let result = Color::parse(i)?;
                            let _ = i.expect_comma()?;
                            Box::new(result)
                        },
                        dark: Box::new(Color::parse(i)?),
                    })
                }),
                _ => Err(
                    begin.new_custom_error(StyleParseError::ExpectedFunctions(vec![
                        "rgb",
                        "hsl",
                        "hwb",
                        "oklab",
                        "lab",
                        "oklch",
                        "lch",
                        "color",
                        "color-mix",
                        "light-dark",
                    ])),
                ),
            },
            _ => Err(begin.new_custom_error(StyleParseError::Expected(
                "hex, transparent, currentColor, function colors, named colors, or system colors",
            ))),
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
                        Some(alpha) => u32::from_be_bytes([*red, *green, *blue, *alpha]),
                        None => u32::from_be_bytes([0, *red, *green, *blue]),
                    };
                    format!("#{:x}", num)
                }
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
                    a: a_axis,
                    b: b_axis,
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
