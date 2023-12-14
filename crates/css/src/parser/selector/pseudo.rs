use std::fmt::Display;

use crate::parser::selector::compound::CompoundSelector;
use crate::parser::selector::SelectorList;
use crate::parser::stylesheet::StyleParseError;
use crate::parser::Parse;
use cssparser::{CowRcStr, ParseError, ParseErrorKind, Parser, Token};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl<'i, 't> Parse<'i, 't> for Direction {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let dir = input.expect_ident()?;
        match dir.as_ref() {
            "ltr" => Ok(Direction::Ltr),
            "rtl" => Ok(Direction::Rtl),
            _ => {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::UnkownSyntax),
                    location: input.current_source_location(),
                })
            }
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Rtl => "rtl",
                Self::Ltr => "ltr",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Parity {
    Even,
    Odd,
}

impl Display for Parity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Parity::Odd => "odd",
                Parity::Even => "event",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Nth<'i> {
    Functional {
        step: isize,
        offset: usize,
        of: Option<CompoundSelector<'i>>,
    },
    Parity(Parity),
}

fn parse_nth_step<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<isize, ParseError<'i, StyleParseError>> {
    let next = input.next();
    match next {
        Ok(Token::Dimension { value, unit, .. }) if unit.as_ref() == "n" => {
            if *value < -1. {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::InvalidNthFormat),
                    location: input.current_source_location(),
                });
            }
            Ok(*value as isize)
        }
        _ => {
            Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::InvalidNthFormat),
                location: input.current_source_location(),
            })
        }
    }
}

fn parse_nth_offset<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<usize, ParseError<'i, StyleParseError>> {
    let before = input.state();
    match input.next() {
        Ok(Token::Delim('+')) => {
            match input.next() {
                Ok(Token::Number { value, .. }) => {
                    if *value < 0. {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(StyleParseError::InvalidNthFormat),
                            location: input.current_source_location(),
                        });
                    }
                    Ok(*value as usize)
                }
                _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::InvalidNthFormat),
                        location: input.current_source_location(),
                    });
                }
            }
        }
        Ok(Token::Number { value, .. }) => {
            if *value < 0. {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::InvalidNthFormat),
                    location: input.current_source_location(),
                });
            }
            Ok(*value as usize)
        }
        _ => {
            input.reset(&before);
            Ok(0)
        }
    }
}

fn parse_nth_of<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Option<CompoundSelector<'i>>, ParseError<'i, StyleParseError>> {
    let before = input.state();
    match input.expect_ident_matching("of") {
        Ok(_) => {
            input.skip_whitespace();
            Ok(Some(CompoundSelector::parse(input)?))
        }
        _ => {
            input.reset(&before);
            Ok(None)
        }
    }
}

impl<'i, 't> Parse<'i, 't> for Nth<'i> {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let before = input.state();
        let next = input.next();
        Ok(match next {
            Ok(Token::Ident(val)) if val.as_ref() == "odd" => Nth::Parity(Parity::Odd),
            Ok(Token::Ident(val)) if val.as_ref() == "even" => Nth::Parity(Parity::Even),
            _ => {
                input.reset(&before);
                let step = parse_nth_step(input)?;
                let offset = parse_nth_offset(input)?;
                let of = parse_nth_of(input)?;
                Nth::Functional { step, offset, of }
            }
        })
    }
}

impl<'i> Display for Nth<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Nth::Functional { step, offset, of } => {
                    format!(
                        "{}n{:+}{}",
                        step,
                        offset,
                        match of {
                            Some(of) => {
                                format!(" of {}", of)
                            }
                            None => String::new(),
                        }
                    )
                }
                Nth::Parity(parity) => parity.to_string(),
            }
        )
    }
}

impl<'i> Default for Nth<'i> {
    fn default() -> Self {
        Nth::Parity(Parity::Odd)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PseudoClass<'i> {
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
    Has(SelectorList<'i>),
    Host(CompoundSelector<'i>),
    HostContext,
    Hover,
    InRange,
    Indeterminate,
    Invalid,
    Is(SelectorList<'i>),
    Lang(Vec<CowRcStr<'i>>),
    LastChild,
    LastOfType,
    Left,
    Link,
    LocalLink,
    Modal,
    Muted,
    Not(SelectorList<'i>),
    NthChild(Nth<'i>),
    NthLastChild(Nth<'i>),
    NthLastOfType(Nth<'i>),
    NthOfType(Nth<'i>),
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
    VolumeLocked,
    Where(SelectorList<'i>),
}

impl<'i> Display for PseudoClass<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ":{}",
            match self {
                PseudoClass::Active => "active".to_string(),
                PseudoClass::AnyLink => "any-link".to_string(),
                PseudoClass::AutoFill => "auto-fill".to_string(),
                PseudoClass::Blank => "blank".to_string(),
                PseudoClass::Buffering => "buffering".to_string(),
                PseudoClass::Checked => "checked".to_string(),
                PseudoClass::Current => "current".to_string(),
                PseudoClass::Default => "default".to_string(),
                PseudoClass::Defined => "defined".to_string(),
                PseudoClass::Dir(dir) => format!("dir({})", dir),
                PseudoClass::Disabled => "disabled".to_string(),
                PseudoClass::Empty => "empty".to_string(),
                PseudoClass::Enabled => "enabled".to_string(),
                PseudoClass::First => "first".to_string(),
                PseudoClass::FirstChild => "first-child".to_string(),
                PseudoClass::FirstOfType => "first-of-type".to_string(),
                PseudoClass::Focus => "focus".to_string(),
                PseudoClass::FocusVisible => "focus-visible".to_string(),
                PseudoClass::FocusWithin => "focus-within".to_string(),
                PseudoClass::FullScreen => "fullscreen".to_string(),
                PseudoClass::Future => "future".to_string(),
                PseudoClass::Has(selectors) => format!("has({})", selectors),
                PseudoClass::Host(compound) => format!("host({})", compound),
                PseudoClass::HostContext => "host-context".to_string(),
                PseudoClass::Hover => "hover".to_string(),
                PseudoClass::InRange => "in-range".to_string(),
                PseudoClass::Indeterminate => "indeterminate".to_string(),
                PseudoClass::Invalid => "invalid".to_string(),
                PseudoClass::Is(selectors) => format!("is({})", selectors),
                PseudoClass::Lang(langs) => format!("lang({})", langs.join(",")),
                PseudoClass::LastChild => "last-child".to_string(),
                PseudoClass::LastOfType => "last-of-type".to_string(),
                PseudoClass::Left => "left".to_string(),
                PseudoClass::Link => "link".to_string(),
                PseudoClass::LocalLink => "local-link".to_string(),
                PseudoClass::Modal => "modal".to_string(),
                PseudoClass::Muted => "muted".to_string(),
                PseudoClass::Not(selectors) => format!("not({})", selectors),
                PseudoClass::NthChild(nth) => format!("nth-child({})", nth),
                PseudoClass::NthLastChild(nth) => format!("nth-last-child({})", nth),
                PseudoClass::NthLastOfType(nth) => format!("nth-last-of-type({})", nth),
                PseudoClass::NthOfType(nth) => format!("nth-of-type({})", nth),
                PseudoClass::OnlyChild => "only-child".to_string(),
                PseudoClass::OnlyOfType => "only-of-type".to_string(),
                PseudoClass::Optional => "optional".to_string(),
                PseudoClass::OutOfRange => "out-of-range".to_string(),
                PseudoClass::Past => "past".to_string(),
                PseudoClass::Paused => "paused".to_string(),
                PseudoClass::PictureInPicture => "picture-in-picture".to_string(),
                PseudoClass::PlaceholderShown => "placeholder-shown".to_string(),
                PseudoClass::Playing => "playing".to_string(),
                PseudoClass::PopoverOpen => "popover-open".to_string(),
                PseudoClass::ReadOnly => "read-only".to_string(),
                PseudoClass::ReadWrite => "read-write".to_string(),
                PseudoClass::Required => "required".to_string(),
                PseudoClass::Right => "right".to_string(),
                PseudoClass::Root => "root".to_string(),
                PseudoClass::Scope => "scope".to_string(),
                PseudoClass::Seeking => "seeking".to_string(),
                PseudoClass::Stalled => "stalled".to_string(),
                PseudoClass::Target => "target".to_string(),
                PseudoClass::TargetWithin => "target-within".to_string(),
                PseudoClass::UserInvalid => "user-invalid".to_string(),
                PseudoClass::UserValid => "user-valid".to_string(),
                PseudoClass::Valid => "valid".to_string(),
                PseudoClass::Visited => "visited".to_string(),
                PseudoClass::VolumeLocked => "volume-locked".to_string(),
                PseudoClass::Where(selectors) => format!("where({})", selectors),
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PseudoElement<'i> {
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
    Highlight(CowRcStr<'i>),
    Marker,
    Part(CowRcStr<'i>),
    Placeholder,
    Selection,
    Slotted(CompoundSelector<'i>),
    SpellingError,
    TargetText,
    ViewTransition,
    ViewTransitionGroup,
    ViewTransitionImagePair,
    ViewTransitionNew,
    ViewTransitionOld,
}

impl<'i> Display for PseudoElement<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PseudoElement::After => "after".to_string(),
                PseudoElement::Backdrop => "backdrop".to_string(),
                PseudoElement::Before => "before".to_string(),
                PseudoElement::Cue => "cue".to_string(),
                PseudoElement::CueRegion => "cue-region".to_string(),
                PseudoElement::FileSelectorButton => "file-selector-button".to_string(),
                PseudoElement::FirstLetter => "first-letter".to_string(),
                PseudoElement::FirstLine => "first-line".to_string(),
                PseudoElement::GrammerError => "grammer-error".to_string(),
                PseudoElement::Highlight(highlight) => format!("highlight({})", highlight),
                PseudoElement::Marker => "marker".to_string(),
                PseudoElement::Part(part) => format!("part({})", part),
                PseudoElement::Placeholder => "placeholder".to_string(),
                PseudoElement::Selection => "selection".to_string(),
                PseudoElement::Slotted(compound) => format!("slotted({})", compound),
                PseudoElement::SpellingError => "spelling-error".to_string(),
                PseudoElement::TargetText => "target-text".to_string(),
                PseudoElement::ViewTransition => "view-transition".to_string(),
                PseudoElement::ViewTransitionGroup => "view-transition-group".to_string(),
                PseudoElement::ViewTransitionImagePair => "view-transition-image-pair".to_string(),
                PseudoElement::ViewTransitionNew => "view-transition-new".to_string(),
                PseudoElement::ViewTransitionOld => "view-transition-old".to_string(),
            }
        )
    }
}

fn parse_arguments<'i, 't, F, T>(
    input: &mut Parser<'i, 't>,
    function: bool,
    clbk: F,
) -> Result<T, ParseError<'i, StyleParseError>>
where
    F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, StyleParseError>>,
{
    if !function && input.expect_parenthesis_block().is_err() {
        return Err(ParseError {
            kind: ParseErrorKind::Custom(StyleParseError::ExpectedArguments),
            location: input.current_source_location(),
        });
    }

    input.parse_nested_block(clbk)
}

impl<'i, 't> PseudoClass<'i> {
    pub fn parse(
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        function: bool,
    ) -> Result<Self, ParseError<'i, StyleParseError>> {
        Ok(match name.as_ref() {
            "active" => PseudoClass::Active,
            "any-link" => PseudoClass::AnyLink,
            "autofill" => PseudoClass::AutoFill,
            "blank" => PseudoClass::Blank,
            "buffering" => PseudoClass::Buffering,
            "checked" => PseudoClass::Checked,
            "current" => PseudoClass::Current,
            "default" => PseudoClass::Default,
            "defined" => PseudoClass::Defined,
            "dir" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::Dir(Direction::parse(i)?))
            })?,
            "disabled" => PseudoClass::Disabled,
            "empty" => PseudoClass::Empty,
            "enabled" => PseudoClass::Enabled,
            "first" => PseudoClass::First,
            "first-child" => PseudoClass::FirstChild,
            "first-of-type" => PseudoClass::FirstOfType,
            "focus" => PseudoClass::Focus,
            "focus-visible" => PseudoClass::FocusVisible,
            "focus-within" => PseudoClass::FocusWithin,
            "fullscreen" => PseudoClass::FullScreen,
            "future" => PseudoClass::Future,
            "has" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::Has(SelectorList::parse(i, false)?))
            })?,
            "host" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::Host(CompoundSelector::parse(i)?))
            })?,
            "host-context" => PseudoClass::HostContext,
            "hover" => PseudoClass::Hover,
            "in-range" => PseudoClass::InRange,
            "indeterminate" => PseudoClass::Indeterminate,
            "invalid" => PseudoClass::Invalid,
            "is" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::Is(SelectorList::parse(i, true)?))
            })?,
            "lang" => parse_arguments(input, function, |i| {
                let values = i.parse_comma_separated(|i| match i.next() {
                    Ok(Token::Ident(string)) | Ok(Token::QuotedString(string)) => {
                        Ok(string.clone())
                    }
                    _ => Err(ParseError {
                        kind: ParseErrorKind::Custom(StyleParseError::ExpectedIdentOrString),
                        location: i.current_source_location(),
                    }),
                })?;
                Ok(PseudoClass::Lang(values))
            })?,
            "last-child" => PseudoClass::LastChild,
            "last-of-type" => PseudoClass::LastOfType,
            "left" => PseudoClass::Left,
            "link" => PseudoClass::Link,
            "local-link" => PseudoClass::LocalLink,
            "modal" => PseudoClass::Modal,
            "muted" => PseudoClass::Muted,
            "not" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::Not(SelectorList::parse(i, false)?))
            })?,
            "nth-child" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::NthChild(Nth::parse(i)?))
            })?,
            "nth-last-child" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::NthLastChild(Nth::parse(i)?))
            })?,
            "nthlastoftype" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::NthLastOfType(Nth::parse(i)?))
            })?,
            "nth-of-type" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::NthOfType(Nth::parse(i)?))
            })?,
            "only-child" => PseudoClass::OnlyChild,
            "only-of-type" => PseudoClass::OnlyOfType,
            "optional" => PseudoClass::Optional,
            "out-of-range" => PseudoClass::OutOfRange,
            "past" => PseudoClass::Past,
            "paused" => PseudoClass::Paused,
            "picture-in-picture" => PseudoClass::PictureInPicture,
            "placeholder-shown" => PseudoClass::PlaceholderShown,
            "playing" => PseudoClass::Playing,
            "popover-open" => PseudoClass::PopoverOpen,
            "read-only" => PseudoClass::ReadOnly,
            "read-write" => PseudoClass::ReadWrite,
            "required" => PseudoClass::Required,
            "right" => PseudoClass::Right,
            "root" => PseudoClass::Root,
            "scope" => PseudoClass::Scope,
            "seeking" => PseudoClass::Seeking,
            "stalled" => PseudoClass::Stalled,
            "target" => PseudoClass::Target,
            "target-within" => PseudoClass::TargetWithin,
            "userin-valid" => PseudoClass::UserInvalid,
            "user-valid" => PseudoClass::UserValid,
            "valid" => PseudoClass::Valid,
            "visited" => PseudoClass::Visited,
            "volume-locked" => PseudoClass::VolumeLocked,
            "where" => parse_arguments(input, function, |i| {
                Ok(PseudoClass::Where(SelectorList::parse(i, true)?))
            })?,
            _ => {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::UnkownPseudoClass),
                    location: input.current_source_location(),
                })
            }
        })
    }
}

impl<'i, 't> PseudoElement<'i> {
    pub fn parse(
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        function: bool,
    ) -> Result<Self, ParseError<'i, StyleParseError>> {
        Ok(match name.as_ref() {
            "after" => PseudoElement::After,
            "backdrop" => PseudoElement::Backdrop,
            "before" => PseudoElement::Before,
            "cue" => PseudoElement::Cue,
            "cueregion" => PseudoElement::CueRegion,
            "fileselectorbutton" => PseudoElement::FileSelectorButton,
            "firstletter" => PseudoElement::FirstLetter,
            "firstline" => PseudoElement::FirstLine,
            "grammererror" => PseudoElement::GrammerError,
            "highlight" => parse_arguments(input, function, |i| {
                Ok(PseudoElement::Highlight(i.expect_ident()?.clone()))
            })?,
            "marker" => PseudoElement::Marker,
            "part" => parse_arguments(input, function, |i| {
                let value = i.expect_ident()?;
                Ok(PseudoElement::Part(value.clone()))
            })?,
            "placeholder" => PseudoElement::Placeholder,
            "selection" => PseudoElement::Selection,
            "slotted" => parse_arguments(input, function, |i| {
                Ok(PseudoElement::Slotted(CompoundSelector::parse(i)?))
            })?,
            "spellingerror" => PseudoElement::SpellingError,
            "targettext" => PseudoElement::TargetText,
            "viewtransition" => PseudoElement::ViewTransition,
            "viewtransitiongroup" => PseudoElement::ViewTransitionGroup,
            "viewtransitionimagepair" => PseudoElement::ViewTransitionImagePair,
            "viewtransitionnew" => PseudoElement::ViewTransitionNew,
            "viewtransitionold" => PseudoElement::ViewTransitionOld,
            _ => {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(StyleParseError::UnkownPseudoElement),
                    location: input.current_source_location(),
                })
            }
        })
    }
}

#[cfg(test)]
mod test {
    use cssparser::{Parser, ParserInput};

    use crate::parser::{
        selector::{CompoundSelector, Nth, Parity},
        Parse,
    };

    use super::Direction;

    #[test]
    fn direction_parse() {
        let src = "ltr";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = Direction::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == Direction::Ltr)
    }

    #[test]
    fn nth_parse() {
        let src = "odd";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = Nth::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == Nth::Parity(Parity::Odd));

        let src = "even";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = Nth::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == Nth::Parity(Parity::Even));

        let src = "3n + 2";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = Nth::parse(&mut parser);
        assert!(result.is_ok());
        assert!(
            result.unwrap()
                == Nth::Functional {
                    step: 3,
                    offset: 2,
                    of: None
                }
        );

        let src = "2n of p";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = Nth::parse(&mut parser);
        assert!(result.is_ok());
        assert!(
            result.unwrap()
                == Nth::Functional {
                    step: 2,
                    offset: 0,
                    of: Some(CompoundSelector {
                        tag: Some("p".into()),
                        ..Default::default()
                    })
                }
        );
    }
}
