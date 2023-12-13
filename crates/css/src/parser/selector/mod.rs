mod attribute;
mod combinator;
mod compound;
mod pseudo;

use crate::parser::selector::combinator::Combinator;
use crate::parser::selector::compound::CompoundSelector;
use crate::parser::stylesheet::StyleParseError;
use crate::parser::Parse;
use cssparser::Delimiter::CurlyBracketBlock;
use cssparser::Token::{CloseSquareBracket, Delim, WhiteSpace};
use cssparser::{CowRcStr, Delimiter, Delimiters, ParseError, ParseErrorKind, Parser, Token};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub enum PseudoSelector<'i> {
    Function { name: CowRcStr<'i> },
    Element(CowRcStr<'i>),
}

impl<'i> Display for PseudoSelector<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PseudoSelector::Function { name } => format!(":{}()", name.to_string()),
                PseudoSelector::Element(element) => format!("::{}", element.to_string()),
            }
        )
    }
}

impl<'i> Debug for PseudoSelector<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PseudoSelector::Function { name } => format!("Function({})", name.to_string()),
                PseudoSelector::Element(element) => format!("Element({})", element.to_string()),
            }
        )
    }
}

#[derive(Debug)]
pub enum ComplexSelector<'i> {
    Combinator(Combinator),
    Compound(CompoundSelector<'i>),
}

impl<'i> Display for ComplexSelector<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ComplexSelector::Combinator(combinator) => combinator.to_string(),
                ComplexSelector::Compound(compound) => compound.to_string(),
            }
        )
    }
}

/// Element and Combinator parts of a selector
#[derive(Debug, Default)]
pub struct RelativeSelector<'i> {
    parts: Vec<ComplexSelector<'i>>,
}

impl<'i> Display for RelativeSelector<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.parts.iter().peekable();
        if let Some(ComplexSelector::Combinator(Combinator::Descendant)) = iter.peek() {
            iter.next();
        }

        write!(
            f,
            "{}",
            iter
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl<'i> RelativeSelector<'i> {
    pub fn add_part(&mut self, part: ComplexSelector<'i>) {
        self.parts.push(part);
    }
}

enum Previous {
    Combinator(Combinator),
    Compound,
}
impl<'i, 't> Parse<'i, 't> for RelativeSelector<'i> {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let mut selector = RelativeSelector::default();

        let mut prev = Previous::Compound;
        'relative: loop {
            if input.is_exhausted() {
                break;
            }

            input.skip_whitespace();
            // Combinator or Compound. Cannot have two combinators in a row.
            // Compound after Compound assumes Descendant Combinator; inject it.
            let before = input.state();
            match Combinator::parse(input) {
                Ok(combinator) => {
                    if let Previous::Combinator(prev) = prev {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(StyleParseError::UnexpectedCombinator),
                            location: input.current_source_location(),
                        });
                    }
                    selector.add_part(ComplexSelector::Combinator(combinator));
                    prev = Previous::Combinator(combinator);
                }
                Err(..) => {
                    input.reset(&before);
                    let before = input.state();
                    match CompoundSelector::parse(input) {
                        Ok(result) => {
                            if let Previous::Compound = prev {
                                selector
                                    .add_part(ComplexSelector::Combinator(Combinator::Descendant));
                            }
                            selector.add_part(ComplexSelector::Compound(result));
                            prev = Previous::Compound;
                        }
                        Err(..) => {
                            input.reset(&before);
                            break 'relative;
                        }
                    }
                }
            }
        }

        Ok(selector)
    }
}

/// Comman seperated list of selectors
#[derive(Debug, Default)]
pub struct SelectorList<'i> {
    pattern: Vec<RelativeSelector<'i>>,
}

impl<'i> SelectorList<'i> {
    pub fn add_selector(&mut self, selector: RelativeSelector<'i>) {
        self.pattern.push(selector);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, RelativeSelector<'i>> {
        self.pattern.iter()
    }
}

impl<'i, 't> Parse<'i, 't> for SelectorList<'i> {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let mut list = SelectorList::default();
        loop {
            let selector =
                input.parse_until_before(Delimiter::Comma, |i| RelativeSelector::parse(i));

            let ok = selector.is_ok();
            match selector {
                Ok(selector) => {
                    list.add_selector(selector);
                }
                Err(err) => return Err(err),
            }

            loop {
                match input.next() {
                    Err(_) => return Ok(list),
                    Ok(&Token::Comma) => break,
                    Ok(_) => {
                        debug_assert!(!ok, "Shouldn't get a selector if there was an error");
                    }
                }
            }
        }
    }
}
