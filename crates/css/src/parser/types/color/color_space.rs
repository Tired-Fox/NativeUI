use crate::parser::stylesheet::StyleParseError;
use crate::parser::Parse;
use cssparser::{ParseError, ParseErrorKind, Parser};
use std::fmt::Display;

#[derive(Debug, Default)]
pub enum ColorSpace {
    #[default]
    SRGB,
    SRGBLinear,
    DisplayP3,
    A98RGB,
    ProPhoto,
    Rec2020,
    XYZ,
    XYZD50,
    XYZD65,
}

impl Display for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ColorSpace::SRGB => "srgb",
                ColorSpace::SRGBLinear => "srgb-linear",
                ColorSpace::DisplayP3 => "display-p3",
                ColorSpace::A98RGB => "a98-rgb",
                ColorSpace::ProPhoto => "prophoto-rgb",
                ColorSpace::Rec2020 => "rec2020",
                ColorSpace::XYZ => "xyz",
                ColorSpace::XYZD50 => "xyz-d50",
                ColorSpace::XYZD65 => "xyz-d65",
            }
        )
    }
}

impl Parse for ColorSpace {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let i = input.current_source_location();
        match input.expect_ident()?.as_ref() {
            "srgb" => Ok(ColorSpace::SRGB),
            "srgb-linear" => Ok(ColorSpace::SRGBLinear),
            "display-p3" => Ok(ColorSpace::DisplayP3),
            "a98-rgb" => Ok(ColorSpace::A98RGB),
            "prophoto-rgb" => Ok(ColorSpace::ProPhoto),
            "rec2020" => Ok(ColorSpace::Rec2020),
            "xyz" => Ok(ColorSpace::XYZ),
            "xyz-d50" => Ok(ColorSpace::XYZD50),
            "xyz-d65" => Ok(ColorSpace::XYZD65),
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                location: i,
            }),
        }
    }
}

#[derive(Debug, Default)]
pub enum RectangularColorSpace {
    SRGB,
    SRGBLinear,
    LAB,
    #[default]
    OKLAB,
    XYZ,
    XYZD50,
    XYZD65,
}

impl Display for RectangularColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SRGB => "in srgb",
                Self::SRGBLinear => "in srgb-linear",
                Self::OKLAB => "",
                Self::LAB => "in lab",
                Self::XYZD65 => "in xyz-d65",
                Self::XYZ => "in xyz",
                Self::XYZD50 => "in xyz-d50",
            }
        )
    }
}

impl Parse for RectangularColorSpace {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let i = input.current_source_location();
        input.expect_ident_matching("in")?;
        match input.expect_ident()?.as_ref() {
            "srgb" => Ok(RectangularColorSpace::SRGB),
            "srgb-linear" => Ok(RectangularColorSpace::SRGBLinear),
            "lab" => Ok(RectangularColorSpace::LAB),
            "xyz-d65" => Ok(RectangularColorSpace::XYZD65),
            "xyz" => Ok(RectangularColorSpace::XYZ),
            "xyz-d50" => Ok(RectangularColorSpace::XYZD50),
            "oklab" => Ok(RectangularColorSpace::OKLAB),
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                location: i,
            }),
        }
    }
}

#[derive(Debug)]
pub enum PolarColorSpace {
    HSL,
    HWB,
    LCH,
    OKLCH,
}

impl Display for PolarColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::HSL => "in hsl",
                Self::HWB => "in hwb",
                Self::LCH => "in lch",
                Self::OKLCH => "in oklch",
            }
        )
    }
}

impl Parse for PolarColorSpace {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let i = input.current_source_location();
        input.expect_ident_matching("in")?;
        match input.expect_ident()?.as_ref() {
            "hsl" => Ok(PolarColorSpace::HSL),
            "hwb" => Ok(PolarColorSpace::HWB),
            "lch" => Ok(PolarColorSpace::LCH),
            "oklch" => Ok(PolarColorSpace::OKLCH),
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                location: i,
            }),
        }
    }
}

#[derive(Debug, Default)]
pub enum HueInterpolationMethod {
    #[default]
    Shorter,
    Longer,
    Increasing,
    Decreasing,
}

impl Display for HueInterpolationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} hue",
            match self {
                Self::Longer => "longer",
                Self::Shorter => "shorter",
                Self::Increasing => "increasing",
                Self::Decreasing => "decreasing",
            }
        )
    }
}

impl Parse for HueInterpolationMethod {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let i = input.current_source_location();
        let result = match input.expect_ident()?.as_ref() {
            "longer" => Ok(HueInterpolationMethod::Longer),
            "shorter" => Ok(HueInterpolationMethod::Shorter),
            "increasing" => Ok(HueInterpolationMethod::Increasing),
            "decreasing" => Ok(HueInterpolationMethod::Decreasing),
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                location: i,
            }),
        }?;

        input.expect_ident_matching("hue")?;
        Ok(result)
    }
}

#[derive(Debug)]
pub enum ColorInterpolationMethod {
    Rectangular(RectangularColorSpace),
    Polar(PolarColorSpace, HueInterpolationMethod),
}

impl Display for ColorInterpolationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Rectangular(cs) => cs.to_string(),
                Self::Polar(cs, him) => format!("{} {}", cs, him),
            }
        )
    }
}

impl Default for ColorInterpolationMethod {
    fn default() -> Self {
        ColorInterpolationMethod::Rectangular(RectangularColorSpace::default())
    }
}

impl Parse for ColorInterpolationMethod {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let i = input.current_source_location();
        let state = input.state();
        input.expect_ident_matching("in")?;
        match input.expect_ident()?.as_ref() {
            "srgb" | "srgb-linear" | "lab" | "xyz-d65" | "xyz" | "xyz-d50" | "oklab" => {
                input.reset(&state);
                Ok(ColorInterpolationMethod::Rectangular(
                    RectangularColorSpace::parse(input)?,
                ))
            }
            "hsl" | "hwb" | "lch" | "oklch" => {
                input.reset(&state);
                Ok(ColorInterpolationMethod::Polar(
                    PolarColorSpace::parse(input)?,
                    HueInterpolationMethod::parse(input)
                        .unwrap_or(HueInterpolationMethod::default()),
                ))
            }
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                location: i,
            }),
        }
    }
}
