pub mod unit;

use std::{ascii::AsciiExt, fmt::Display};

use cssparser::{CowRcStr, ParseError, ParseErrorKind, Parser, Token};

use unit::{Angle, Length};

use super::{color::Color, stylesheet::StyleParseError, Parse};

/// -moz-* properties
#[derive(Debug)]
pub enum Moz {
    /// -moz-float-edge
    FloatEdge,
    /// -moz-force-broken-image-icon
    ForceBrokenImageIcon,
    /// -moz-image-region
    ImageRegion,
    /// -moz-orient
    Orient,
    /// -moz-user-focus
    UserFocus,
    /// -moz-user-input
    UserInput,
}

/// -webkit-* properties
#[derive(Debug)]
pub enum Webkit {
    /// -webkit-border-before
    BorderBefore,
    /// -webkit-box-reflect
    BoxReflect,
    /// -webkit-line-clamp
    LineClamp,
    /// -webkit-mask-attachment
    MaskAttachment,
    /// -webkit-mask-box-image
    MaskBoxImage,
    /// -webkit-mask-composite
    MaskComposite,
    /// -webkit-mask-position-x
    MaskPositionX,
    /// -webkit-mask-position-y
    MaskPositionY,
    /// -webkit-mask-repeat-x
    MaskRepeatX,
    /// -webkit-mask-repeat-y
    MaskRepeatY,
    /// -webkit-overflow-scrolling
    OverflowScrolling,
    /// -webkit-tap-highlight-color
    TapHighlightColor,
    /// -webkit-text-fill-color
    TextFillColor,
    /// -webkit-text-security
    TextSecurity,
    /// -webkit-text-stroke
    TextStroke,
    /// -webkit-text-stroke-color
    TextStrokeColor,
    /// -webkit-text-stroke-width
    TextStrokeWidth,
    /// -webkit-touch-callout
    TouchCallout,
}

#[derive(Debug)]
pub enum Baseline {
    Normal,
    First,
    Last,
}

impl Baseline {
    pub fn fallback(self) -> Align {
        match self {
            Baseline::Normal => Align::Normal,
            Baseline::First => Align::Start,
            Baseline::Last => Align::End,
        }
    }
}

#[derive(Debug)]
pub enum Align {
    Start,
    FlexStart,
    End,
    FlexEnd,
    Center,
    Normal,
    Baseline(Baseline),
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
    Safe,
    Unsafe,
}

#[derive(Debug, Default)]
pub enum Composition {
    #[default]
    Replace,
    Add,
    Accumulate,
}

#[derive(Debug)]
pub enum Time {
    Ms(f32),
    S(f32),
}

impl Default for Time {
    fn default() -> Self {
        Time::S(0.)
    }
}

#[derive(Debug, Default)]
pub enum Direction {
    #[default]
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Debug)]
pub enum Parity {
    Odd,
    Even,
}

#[derive(Debug)]
pub enum Keyframe {
    To,
    From,
    Custom(f32),
}

#[derive(Debug, Default)]
pub enum FillMode {
    #[default]
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Debug, Default)]
pub enum AutoOr<T> {
    #[default]
    Auto,
    Or(T),
}

impl<T: Parse + Display> Display for AutoOr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AutoOr::Auto => "auto".to_string(),
                AutoOr::Or(value) => value.to_string(),
            }
        )
    }
}

impl<T: Parse> Parse for AutoOr<T> {
    fn parse<'i, 't>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, super::stylesheet::StyleParseError>> {
        let start = input.state();
        if let Ok(value) = input.expect_ident() {
            match value.to_ascii_lowercase().as_str() {
                "auto" => return Ok(AutoOr::Auto),
                _ => input.reset(&start),
            }
        }
        Ok(AutoOr::Or(T::parse(input)?))
    }
}

#[derive(Debug, Default)]
pub enum NoneOr<T: Parse> {
    #[default]
    None,
    Or(T),
}

impl<T: Parse + Display> Display for NoneOr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoneOr::None => "none".to_string(),
                NoneOr::Or(value) => value.to_string(),
            }
        )
    }
}

#[derive(Debug, Default)]
pub enum PlayState {
    Running,
    #[default]
    Paused,
}

#[derive(Debug)]
pub enum TimelineRangeName {
    Cover,
    Contain,
    Entry,
    Exit,
    EntryCrossing,
    ExitCrossing,
}
#[derive(Debug, Default)]
pub enum TimelineRange {
    #[default]
    Normal,
    LengthPercentage(f32),
    TimelineRangeName(TimelineRangeName, f32),
}

#[derive(Debug, Default)]
pub enum Scroller {
    #[default]
    Nearest,
    Root,
    _Self,
}

#[derive(Debug, Default)]
pub enum Axis {
    #[default]
    Block,
    Inline,
    X,
    Y,
}

#[derive(Debug, Default)]
pub struct ViewTimelineInset {
    start: AutoOr<f32>,
    end: Option<AutoOr<f32>>,
}

#[derive(Debug, Default)]
pub enum Timeline {
    #[default]
    None,
    Auto,
    Scroll(Scroller, Axis),
    View(Axis, ViewTimelineInset),
    Custom(String),
}

#[derive(Debug)]
pub enum JumpTerm {
    Start,
    End,
    None,
    Both,
}

#[derive(Debug, Default)]
pub enum TimingFunction {
    #[default]
    Ease,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Steps(f32, JumpTerm),
    StepStart,
    StepEnd,
}

#[derive(Debug)]
pub enum GlobalOr<T: Parse> {
    Inherit,
    Initial,
    Revert,
    RevertLayer,
    Unset,
    Or(T),
}

impl<T: Parse + Display> Display for GlobalOr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GlobalOr::Inherit => "inherit".to_string(),
                GlobalOr::Initial => "initial".to_string(),
                GlobalOr::Revert => "revert".to_string(),
                GlobalOr::RevertLayer => "revert-layer".to_string(),
                GlobalOr::Unset => "unset".to_string(),
                GlobalOr::Or(val) => val.to_string(),
            }
        )
    }
}

impl<T: Parse> Parse for GlobalOr<T> {
    fn parse<'i, 't>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, super::stylesheet::StyleParseError>> {
        let start = input.state();
        if let Ok(value) = input.expect_ident() {
            match value.to_ascii_lowercase().as_str() {
                "inherit" => return Ok(GlobalOr::Inherit),
                "initial" => return Ok(GlobalOr::Initial),
                "revert" => return Ok(GlobalOr::Revert),
                "revert-layer" => return Ok(GlobalOr::RevertLayer),
                "unset" => return Ok(GlobalOr::Unset),
                _ => input.reset(&start),
            }
        }
        Ok(GlobalOr::Or(T::parse(input)?))
    }
}

#[derive(Debug)]
pub struct Animation {
    composition: Composition,
    delay: Time,
    direction: Direction,
    duration: AutoOr<Time>,
    fill_mode: FillMode,
    iteration_count: f32,
    name: Option<String>,
    play_state: PlayState,
    start: TimelineRange,
    end: TimelineRange,
    timeline: Vec<Timeline>,
    timing_function: TimingFunction,
}

#[derive(Debug)]
pub enum Attachement {
    Scroll,
    Fixed,
    Local,
}

#[derive(Debug)]
pub struct Background {
    attachment: Vec<Attachement>,
}

#[derive(Debug)]
pub enum SafeUnsafe {
    Safe,
    Unsafe,
}

#[derive(Debug)]
pub enum Appearance {
    None,
    Auto,
}

#[derive(Debug)]
pub struct Ratio {
    width: f32,
    height: f32,
}

#[derive(Debug)]
pub enum AspectRatio {
    Auto,
    Ratio(Ratio),
}

pub trait FromNumber {
    fn from_number(number: f32) -> Self;
}

impl FromNumber for f32 {
    fn from_number(number: f32) -> Self {
        number
    }
}

impl FromNumber for i32 {
    fn from_number(number: f32) -> Self {
        number as i32
    }
}

impl FromNumber for i8 {
    fn from_number(number: f32) -> Self {
        number as i8
    }
}

impl FromNumber for u8 {
    fn from_number(number: f32) -> Self {
        number as u8
    }
}

#[derive(Debug)]
pub enum PercentOrNumber<T: FromNumber = f32> {
    Number(T),
    Percent(f32),
}

impl<T: FromNumber> Parse for PercentOrNumber<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.next() {
            Ok(Token::Percentage {
                has_sign,
                unit_value,
                int_value,
            }) => Ok(PercentOrNumber::Percent(*unit_value)),
            Ok(Token::Number {
                has_sign,
                value,
                int_value,
            }) => {
                if int_value.is_none() {
                    return Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                        location: input.current_source_location(),
                    })
                }

                Ok(PercentOrNumber::Number(T::from_number(*value)))
            }
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                location: input.current_source_location(),
            }),
        }
    }
}

impl<T: FromNumber + Display> Display for PercentOrNumber<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PercentOrNumber::Percent(val) => format!("{}%", val),
                PercentOrNumber::Number(num) => num.to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub enum FilterFunction {
    Blur(Length),
    Brightness(PercentOrNumber),
    Contrast(PercentOrNumber),
    DropShadow {
        x: Length,
        y: Length,
        standard_deviation: Option<Length>,
        color: Option<Color>,
    },
    Grayscale(PercentOrNumber),
    HueRotate(Angle),
    Invert(PercentOrNumber),
    Opacity(PercentOrNumber),
    Saturate(PercentOrNumber),
    Sepia(PercentOrNumber),
}

#[derive(Debug)]
pub enum BackdropFilter {
    None,
    Functions(Vec<FilterFunction>),
}

#[derive(Debug)]
pub enum Visibility {
    Visible,
    Hidden,
}

#[derive(Debug)]
pub enum Declaration {
    /// -moz-*
    Moz(Moz),
    /// -webkit-*
    Webkit(Webkit),
    /// accent-color
    AccentColor(GlobalOr<AutoOr<Color>>),
    // align-*
    // AlignContent { safe: Option<SafeUnsafe>, value: GlobalOr<AutoOr<Align>> },
    // AlignItems { safe: Option<SafeUnsafe>, value: GlobalOr<AutoOr<Align>> },
    // AlignSelf { safe: Option<SafeUnsafe>, value: GlobalOr<AutoOr<Align>> },
    // AlignTracks { safe: Option<SafeUnsafe>, value: GlobalOr<AutoOr<Align>> },
    // all
    // All(GlobalOr<()>),
    // animation-*
    // Animation(Animation),
    // appearance
    // Appearance(Appearance),
    // aspect-ratio
    // AspectRatio(AspectRatio),
    // backdrop-filter
    // BackdropFilter(BackdropFilter),
    // backface-visibility
    // BackfaceVisibility(Visibility),
    // background-*
    // Background(Background),
    // block-size
    // border-*
    // bottom
    // box-*
    // break-*
    // caption-side
    // caret-color
    // clear
    // clipDeprecated
    // clip-path
    // color
    Color(GlobalOr<Color>), // color-scheme
                            // column-*
                            // columns
                            // contain-*
                            // container-*
                            // content
                            // content-visibilityExperimental
                            // counter-*
                            // cursor
                            // direction
                            // display
                            // empty-cells
                            // filter
                            // flex-*
                            // float
                            // font-*
                            // forced-color-adjust
                            // gap
                            // grid-*
                            // hanging-punctuation
                            // height
                            // hyphenate-character
                            // hyphenate-limit-chars
                            // hyphens
                            // image-*
                            // initial-letterExperimental
                            // initial-letter-alignExperimental
                            // inline-size
                            // inset-*
                            // isolation
                            // justify-*
                            // left
                            // letter-spacing
                            // line-*
                            // list-*
                            // margin-*
                            // mask-*
                            // masonry-auto-flowExperimental
                            // math-*
                            // max-*
                            // min-*
                            // mix-blend-mode
                            // object-fit
                            // object-position
                            // offset-*
                            // opacity
                            // order
                            // orphans
                            // outline-*
                            // overflow-*
                            // overlayExperimental
                            // overscroll-*
                            // padding-*
                            // page-*
                            // paint-order
                            // perspective
                            // perspective-origin
                            // place-*
                            // pointer-events
                            // position
                            // print-color-adjust
                            // quotes
                            // resize
                            // right
                            // rotate
                            // row-gap
                            // ruby-alignExperimental
                            // ruby-position
                            // scale
                            // scroll-*
                            // scrollbar-*
                            // shape-*
                            // tab-size
                            // table-layout
                            // text-*
                            // timeline-scopeExperimental
                            // top
                            // touch-action
                            // transform-*
                            // transition-*
                            // translate
                            // unicode-bidi
                            // user-modifyNon-standardDeprecated
                            // user-select
                            // vertical-align
                            // view-*
                            // visibility
                            // white-space
                            // white-space-collapseExperimental
                            // widows
                            // width
                            // will-change
                            // word-break
                            // word-spacing
                            // writing-mode
                            // z-index
                            // zoomNon-standard
}

impl Declaration {
    pub fn parse<'i, 't>(
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseError>> {
        let property = match name.as_ref() {
            "accent-color" => Ok(Self::AccentColor(GlobalOr::<AutoOr<Color>>::parse(input)?)),
            "color" => Ok(Self::Color(GlobalOr::<Color>::parse(input)?)),
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownProperty),
                location: input.current_source_location(),
            }),
        };

        // TODO: Parse importance
        input.expect_exhausted()?;
        property
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, value) = match self {
            Declaration::Color(color) => ("color", color.to_string()),
            Declaration::AccentColor(color) => ("accent-color", color.to_string()),
            _ => ("-cyp", String::new()),
        };
        write!(f, "{}: {};", name, value)
    }
}
