use std::fmt::Formatter;
use std::{ascii::AsciiExt, fmt::Display};

use cssparser::{CowRcStr, ParseError, ParseErrorKind, Parser};

use super::{
    base::{Angle, Length},
    color::Color,
    or::{AutoOr, GlobalOr},
};
use crate::parser::error::{Link, StyleParseError};
use crate::parser::types::base::Number;
use crate::parser::types::or::PercentOr;
use crate::parser::Parse;

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

impl Display for Baseline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Baseline::Normal => write!(f, "normal"),
            Baseline::First => write!(f, "first"),
            Baseline::Last => write!(f, "last"),
        }
    }
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

#[derive(Debug, Default)]
pub enum Align {
    Start,
    FlexStart,
    End,
    FlexEnd,
    Center,
    #[default]
    Normal,
    Baseline(Baseline),
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
}

impl Display for Align {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Align::Start => write!(f, "start"),
            Align::FlexStart => write!(f, "flex-start"),
            Align::End => write!(f, "end"),
            Align::FlexEnd => write!(f, "flex-end"),
            Align::Center => write!(f, "center"),
            Align::Normal => write!(f, "normal"),
            Align::Baseline(baseline) => write!(f, "{}", baseline),
            Align::SpaceBetween => write!(f, "space-between"),
            Align::SpaceAround => write!(f, "space-around"),
            Align::SpaceEvenly => write!(f, "space-evenly"),
            Align::Stretch => write!(f, "stretch"),
        }
    }
}

impl Parse for Align {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        input.skip_whitespace();
        let start = input.current_source_location();
        match input.expect_ident() {
            Ok(ident) => match ident.to_ascii_lowercase().as_str() {
                "start" => Ok(Align::Start),
                "flex-start" => Ok(Align::FlexStart),
                "end" => Ok(Align::End),
                "flex-end" => Ok(Align::FlexEnd),
                "center" => Ok(Align::Center),
                "normal" => Ok(Align::Normal),
                "baseline-normal" => Ok(Align::Baseline(Baseline::Normal)),
                "baseline-first" => Ok(Align::Baseline(Baseline::First)),
                "baseline-last" => Ok(Align::Baseline(Baseline::Last)),
                "space-between" => Ok(Align::SpaceBetween),
                "space-around" => Ok(Align::SpaceAround),
                "space-evenly" => Ok(Align::SpaceEvenly),
                "stretch" => Ok(Align::Stretch),
                _ => Err(start.new_custom_error(StyleParseError::ExpectedPattern(Link {
                    title: "normal | <baseline-position> | <content-distribution> | <overflow-position>? <content-position>",
                    url: "https://developer.mozilla.org/en-US/docs/Web/CSS/align-content#formal_syntax",
                }))),
            },
            _ => Err(start.new_custom_error(StyleParseError::ExpectedKeywords(vec![
                    "start",
                    "flex-start",
                    "end",
                    "flex-end",
                    "center",
                    "normal",
                    "baseline-normal",
                    "baseline-first",
                    "baseline-last",
                    "space-between",
                    "space-around",
                    "space-evenly",
                    "stretch",
                ]))),
        }
    }
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

impl Display for SafeUnsafe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SafeUnsafe::Safe => write!(f, "safe"),
            SafeUnsafe::Unsafe => write!(f, "unsafe"),
        }
    }
}

impl Parse for SafeUnsafe {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        match input.expect_ident() {
            Ok(value) => match value.to_ascii_lowercase().as_str() {
                "safe" => Ok(SafeUnsafe::Safe),
                "unsafe" => Ok(SafeUnsafe::Unsafe),
                _ => Err(input
                    .new_custom_error(StyleParseError::ExpectedKeywords(vec!["safe", "unsafe"]))),
            },
            _ => {
                Err(input
                    .new_custom_error(StyleParseError::ExpectedKeywords(vec!["safe", "unsafe"])))
            }
        }
    }
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
pub enum FilterFunction {
    Blur(Length),
    Brightness(PercentOr<Number>),
    Contrast(PercentOr<Number>),
    DropShadow {
        x: Length,
        y: Length,
        standard_deviation: Option<Length>,
        color: Option<Color>,
    },
    Grayscale(PercentOr<Number>),
    HueRotate(Angle),
    Invert(PercentOr<Number>),
    Opacity(PercentOr<Number>),
    Saturate(PercentOr<Number>),
    Sepia(PercentOr<Number>),
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

// TODO: MOZ + WEBKIT
#[derive(Debug)]
pub enum Declaration {
    /// -moz-*
    Moz(Moz),
    /// -webkit-*
    Webkit(Webkit),
    /// accent-color
    AccentColor(GlobalOr<AutoOr<Color>>),
    // align-*
    AlignContent {
        safe: Option<SafeUnsafe>,
        value: GlobalOr<AutoOr<Align>>,
    },
    AlignItems {
        safe: Option<SafeUnsafe>,
        value: GlobalOr<AutoOr<Align>>,
    },
    AlignSelf {
        safe: Option<SafeUnsafe>,
        value: GlobalOr<AutoOr<Align>>,
    },
    AlignTracks {
        safe: Option<SafeUnsafe>,
        value: GlobalOr<AutoOr<Align>>,
    },
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
    Color(GlobalOr<Color>),
    // color-scheme
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
            "accent-color" => {
                let result = GlobalOr::<AutoOr<Color>>::parse(input);
                Ok(Self::AccentColor(result?))
            }
            "color" => Ok(Self::Color(GlobalOr::<Color>::parse(input)?)),
            "align-content" => Ok(Self::AlignContent {
                safe: Option::<SafeUnsafe>::parse(input)?,
                value: GlobalOr::<AutoOr<Align>>::parse(input)?,
            }),
            "align-items" => Ok(Self::AlignItems {
                safe: Option::<SafeUnsafe>::parse(input)?,
                value: GlobalOr::<AutoOr<Align>>::parse(input)?,
            }),
            "align-self" => Ok(Self::AlignSelf {
                safe: Option::<SafeUnsafe>::parse(input)?,
                value: GlobalOr::<AutoOr<Align>>::parse(input)?,
            }),
            "align-tracks" => Ok(Self::AlignTracks {
                safe: Option::<SafeUnsafe>::parse(input)?,
                value: GlobalOr::<AutoOr<Align>>::parse(input)?,
            }),
            // TODO: ...
            _ => Err(input.new_custom_error(StyleParseError::UnkownProperty)),
        };

        // TODO: Parse importance
        input.expect_exhausted()?;
        property
    }

    pub fn name(&self) -> &str {
        match self {
            Declaration::Color(_) => "color",
            Declaration::AccentColor(_) => "accent-color",
            Declaration::AlignContent { .. } => "align-content",
            Declaration::AlignItems { .. } => "align-items",
            Declaration::AlignSelf { .. } => "align-self",
            Declaration::AlignTracks { .. } => "align-tracks",
            // TODO: ...
            _ => "--cypress-error",
        }
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Declaration::Color(color) => color.to_string(),
            Declaration::AccentColor(color) => color.to_string(),
            Declaration::AlignContent { safe, value } => {
                if let Some(safe) = safe {
                    format!("{} {}", safe, value)
                } else {
                    value.to_string()
                }
            }

            Declaration::AlignItems { safe, value } => {
                if let Some(safe) = safe {
                    format!("{} {}", safe, value)
                } else {
                    value.to_string()
                }
            }
            Declaration::AlignSelf { safe, value } => {
                if let Some(safe) = safe {
                    format!("{} {}", safe, value)
                } else {
                    value.to_string()
                }
            }
            Declaration::AlignTracks { safe, value } => {
                if let Some(safe) = safe {
                    format!("{} {}", safe, value)
                } else {
                    value.to_string()
                }
            }
            // TODO: ...
            _ => String::new(),
        };
        write!(f, "{}: {};", self.name(), value)
    }
}
