use std::fmt::{Display, Formatter};

use cssparser::{ParseError, Parser, Token};

use crate::interned;
use crate::parser::error::StyleParseError;
use crate::parser::selector::attribute::AttributeSelector;
use crate::parser::Parse;

use super::{
    pseudo::{PseudoClass, PseudoElement},
    SelectorCompare,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Selector {
    Parent,
    Break,
    Tag(&'static str),
    Id(&'static str),
    Class(&'static str),
    Attribute(AttributeSelector),
    PseudoClass(Box<PseudoClass>),
    PseudoElement(Box<PseudoElement>),
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::Parent => write!(f, "&"),
            Selector::Break => write!(f, ""),
            Selector::Tag(tag) => write!(f, "{}", tag),
            Selector::Id(id) => write!(f, "#{}", id),
            Selector::Class(class) => write!(f, ".{}", class),
            Selector::Attribute(attribute) => write!(f, "{}", attribute),
            Selector::PseudoClass(pseudo_class) => write!(f, "{}", pseudo_class),
            Selector::PseudoElement(pseudo_element) => write!(f, "{}", pseudo_element),
        }
    }
}

impl Parse for Selector {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let start = input.state();

        match input.next_including_whitespace() {
            Ok(Token::Ident(name)) => Ok(Selector::Tag(interned!(name.as_ref()))),
            Ok(Token::IDHash(id)) => Ok(Selector::Id(interned!(id.as_ref()))),
            Ok(Token::Colon) => match input.try_parse(|i| i.expect_colon()) {
                Ok(_) => Ok(Selector::PseudoElement(Box::new(PseudoElement::parse(
                    input,
                )?))),
                Err(_) => Ok(Selector::PseudoClass(Box::new(PseudoClass::parse(input)?))),
            },
            Ok(Token::SquareBracketBlock) => {
                Ok(Selector::Attribute(AttributeSelector::parse(input)?))
            }
            Ok(Token::Delim(ch)) if ch == &'.' || ch == &'&' => {
                if ch == &'.' {
                    Ok(Selector::Class(interned!(input.expect_ident()?.as_ref())))
                } else {
                    Ok(Selector::Parent)
                }
            }
            _ => {
                input.reset(&start);
                Ok(Selector::Break)
            }
        }
    }
}

#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub enum Parent {
    #[default]
    Prepend,
    Append,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CompoundSelector(pub Vec<Selector>);

impl<'i> CompoundSelector {
    pub fn matches<T: SelectorCompare>(&self, other: &T) -> bool {
        other.matches(self)
    }

    pub fn resolve(&mut self, other: Self) {
        let mut selectors = Vec::new();
        for selector in self.0.iter_mut() {
            if let Selector::Parent = selector {
                selectors.extend(other.0.clone());
            } else {
                selectors.push(selector.clone());
            }
        }
        self.0 = selectors;
    }
}

impl<'i> Display for CompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

enum State {
    Regular,
    PseudoClass,
    PseudoElement,
}
macro_rules! invalid_selector {
    ($from: expr, [$($cond: expr),*], $msg: literal) => {
        $(
            if $cond {
                return Err($from.new_custom_error(StyleParseError::InvalidSelector($msg)))
            }
        )*
    };
}
impl Parse for CompoundSelector {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        input.skip_whitespace();

        let mut selectors = vec![];
        let mut state = State::Regular;

        while !input.is_exhausted() {
            let start = input.state();
            let next = Selector::parse(input)?;

            match &next {
                Selector::Parent => {
                    // Decide where to put it
                    selectors.push(next);
                }
                Selector::Class(class) => {
                    invalid_selector!(
                        start.source_location(),
                        [
                            matches!(state, State::PseudoClass),
                            matches!(state, State::PseudoElement)
                        ],
                        "Invalid class selector after pseudo selector"
                    );
                    selectors.push(next);
                }
                Selector::Id(id) => {
                    invalid_selector!(
                        start.source_location(),
                        [
                            matches!(state, State::PseudoClass),
                            matches!(state, State::PseudoElement)
                        ],
                        "Invalid id selector after pseudo selector"
                    );
                    selectors.push(next);
                }
                Selector::Tag(tag) => {
                    invalid_selector!(
                        start.source_location(),
                        [selectors.len() > 0],
                        "Invalid tag selector, can only occur at start of compound selector"
                    );
                    selectors.push(next);
                }
                Selector::Attribute(attr) => {
                    invalid_selector!(
                        start.source_location(),
                        [
                            matches!(state, State::PseudoClass),
                            matches!(state, State::PseudoElement)
                        ],
                        "Invalid attribute selector after pseudo selector"
                    );
                    selectors.push(next);
                }
                Selector::PseudoClass(pseudo) => {
                    invalid_selector!(
                        start.source_location(),
                        [matches!(state, State::PseudoElement)],
                        "Invalid pseudo class selector after pseudo element selector"
                    );
                    invalid_selector!(
                        start.source_location(),
                        [matches!(state, State::PseudoClass)],
                        "More than one pseudo class selector"
                    );
                    selectors.push(next);
                    state = State::PseudoClass
                }
                Selector::PseudoElement(pseudo) => {
                    invalid_selector!(
                        start.source_location(),
                        [matches!(state, State::PseudoElement)],
                        "More than one pseudo element selector"
                    );
                    selectors.push(next);
                    state = State::PseudoElement
                }
                Selector::Break => {
                    return Ok(CompoundSelector(selectors));
                }
            }
        }
        Ok(CompoundSelector(selectors))
    }
}

#[cfg(test)]
mod test {
    use cssparser::{Parser, ParserInput};

    use crate::parser::{
        selector::{AttributeSelector, Direction, Matcher, PseudoClass, PseudoElement},
        Parse,
    };

    use super::{CompoundSelector, Selector};

    #[test]
    fn parse_success() {
        let expected = CompoundSelector(vec![
            Selector::Tag("p".into()),
            Selector::Id("something".into()),
            Selector::Class("green".into()),
            Selector::Class("bold".into()),
            Selector::Attribute(AttributeSelector {
                name: "data-valid".into(),
                matcher: Matcher::Exists,
                ..Default::default()
            }),
            Selector::Attribute(AttributeSelector {
                name: "data-info".into(),
                matcher: Matcher::Include,
                expects: Some("data".into()),
                insensitive: true,
            }),
            Selector::PseudoClass(Box::new(PseudoClass::FirstChild)),
            Selector::PseudoElement(Box::new(PseudoElement::Before)),
        ]);
        let src = "p#something.green.bold[data-valid][data-info~=data i]:first-child::before";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = CompoundSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }

    #[test]
    fn parse_parent_append() {
        let expected = CompoundSelector(vec![
            Selector::Tag("p".into()),
            Selector::Parent,
            Selector::Class("green"),
        ]);
        let src = "p&.green";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = CompoundSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }

    #[test]
    fn parse_parent_prepend() {
        let expected = CompoundSelector(vec![Selector::Parent, Selector::Class("green")]);
        let src = "&.green";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = CompoundSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }

    #[test]
    fn parse_pseudo() {
        let expected = CompoundSelector(vec![
            Selector::PseudoClass(Box::new(PseudoClass::Dir(Direction::Ltr))),
            Selector::PseudoElement(Box::new(PseudoElement::Part("a"))),
        ]);
        let src = ":dir(ltr)::part(a)";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = CompoundSelector::parse(&mut parser);
        if let Err(err) = &result {
            println!("{:?}", err);
        }
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }
}
