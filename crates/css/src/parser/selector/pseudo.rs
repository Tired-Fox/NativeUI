use cssparser::{ParseError, Parser};
use crate::parser::Parse;
use crate::parser::selector::compound::CompoundSelector;
use crate::parser::selector::SelectorList;
use crate::parser::stylesheet::StyleParseError;

#[derive(Default, Debug, Clone)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

#[derive(Debug, Clone)]
pub enum Parity {
    Even,
    Odd,
}

#[derive(Debug, Clone)]
pub enum Nth {
    ANB(String),
    Parity(Parity),
}

pub enum PseudoFunction {
    // -moz classes
    Active,
    AnyLink,
    AutoFill,
    Blank,
    Buffering,
    Checked,
    Current,
    Default,
    Defined,
    Dir(Direction),
    Disabled,
    Empty,
    Enabled,
    First,
    FirstChild,
    FirstOfType,
    Focus,
    FocusVisible,
    FocusWithin,
    FullScreen,
    Future,
    Has(SelectorList),
    Host(CompoundSelector),
    HostContext,
    Hover,
    InRange,
    indeeterminate,
    Invalid,
    Is(SelectorList),
    Lang(Vec<String>),
    LastChild,
    LastOfType,
    Left,
    Link,
    LocalLink,
    Modal,
    Muted,
    Not(SelectorList),
    // TODO: An + B | even | odd
    NthChild(Nth),
    // TODO: An + B | even | odd
    NthLastChild(Nth),
    // TODO: An + B | even | odd
    NthLastOfType(Nth),
    // TODO: An + B | even | odd
    NthOfType(Nth),
    OnlyChild,
    OnlyOfType,
    Optional,
    OutOfRange,
    Past,
    Paused,
    PictureInPicture,
    PlaceholderShown,
    Playing,
    PopoverOpen,
    ReadOnly,
    ReadWrite,
    Required,
    Right,
    Root,
    Scope,
    Seeking,
    Stalled,
    Target,
    TargetWithin,
    UserInvalid,
    UserValid,
    Valid,
    Visited,
    VolumnLocked,
    Where(SelectorList)
}

pub enum PseudoElement {
    // -moz, --webkit functions
    After,
    Backdrop,
    Before,
    Cue,
    CueRegion,
    FileSelectorButton,
    FirstLetter,
    FirstLine,
    GrammerError,
    Highlight,
    Marker,
    Part(String),
    Placeholder,
    Selection,
    Slotted(CompoundSelector),
    SpellingError,
    TargetText,
    ViewTransition,
    ViewTransitionGroup,
    ViewTransitionImagePair,
    ViewTransitionNew,
    ViewTransitionOld
}

impl<'i, 't> Parse<'i, 't> for PseudoFunction {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        todo!()
    }
}

impl<'i, 't> Parse<'i, 't> for PseudoElement {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        todo!()
    }
}
