use crate::parser::selector::attribute::AttributeSelector;
use crate::parser::stylesheet::StyleParseError;
use crate::parser::Parse;
use cssparser::{CowRcStr, ParseError, ParseErrorKind, Parser, Token};
use std::fmt::Display;

use super::{pseudo::{PseudoClass, PseudoElement}, SelectorCompare};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CompoundSelector<'i> {
    pub tag: Option<CowRcStr<'i>>,
    pub id: Option<CowRcStr<'i>>,
    pub classes: Vec<CowRcStr<'i>>,
    pub attributes: Vec<AttributeSelector<'i>>,
    pub pseudo_class: Option<Box<PseudoClass<'i>>>,
    pub pseudo_element: Option<Box<PseudoElement<'i>>>,
}

impl<'i> CompoundSelector<'i> {
    pub fn matches<T: SelectorCompare>(&self, other: &T) -> bool {
        other.matches(self)
    }
}

impl<'i> Display for CompoundSelector<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}",
            self.tag.as_ref().map(|v| v.as_ref()).unwrap_or(""),
            self.id
                .as_ref()
                .map(|v| format!("#{}", v))
                .unwrap_or("".to_string()),
            self.classes
                .iter()
                .map(|v| format!(".{}", v))
                .collect::<Vec<String>>()
                .join(""),
            self.attributes
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(""),
            self.pseudo_class.as_ref().map(|v| v.to_string()).unwrap_or(String::new()),
            self.pseudo_element.as_ref().map(|v| v.to_string()).unwrap_or(String::new()),
        )
    }
}

impl<'i, 't> Parse<'i, 't> for CompoundSelector<'i> {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let mut element = CompoundSelector::default();
        input.skip_whitespace();

        loop {
            let next = input.next_including_whitespace();
            match next {
                Ok(Token::Ident(name)) => {
                    if element.tag.is_some() {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(StyleParseError::DuplicateElementSelector),
                            location: input.current_source_location(),
                        });
                    }
                    element.tag = Some(name.clone());
                }
                Ok(Token::Delim('.')) => {
                    if let Ok(ident) = input.expect_ident() {
                        element.classes.push(ident.clone());
                    }
                }
                Ok(Token::IDHash(value)) => {
                    if element.id.is_some() {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(StyleParseError::DuplicateIDSelector),
                            location: input.current_source_location(),
                        });
                    }
                    element.id = Some(value.clone());
                }
                Ok(Token::Colon) => {
                    let before = input.state();
                    match input.next() {
                        Ok(Token::Ident(name)) => {
                            element.pseudo_class = Some(Box::new(PseudoClass::parse(name.clone(), input, false)?))
                        },
                        Ok(Token::Function(name)) => {
                            element.pseudo_class = Some(Box::new(PseudoClass::parse(name.clone(), input, true)?))
                        },
                        Ok(Token::Colon) => {
                            element.pseudo_element = match input.next() {
                                Ok(Token::Ident(name)) => Some(Box::new(PseudoElement::parse(name.clone(), input, false)?)),
                                Ok(Token::Function(name)) => Some(Box::new(PseudoElement::parse(name.clone(), input, true)?)),
                                _ => {
                                    return Err(ParseError {
                                        kind: ParseErrorKind::Custom(StyleParseError::InvalidPseudoSelector),
                                        location: input.current_source_location()
                                    })
                                }
                            };
                        }
                        _ => {
                            input.reset(&before);
                            return Err(ParseError {
                                kind: ParseErrorKind::Custom(
                                    StyleParseError::InvalidPseudoSelector,
                                ),
                                location: input.current_source_location(),
                            })
                        }
                    }
                }
                Ok(Token::SquareBracketBlock) => {
                    // Parse attribute selector
                    let attribute = AttributeSelector::parse(input)?;
                    element.attributes.push(attribute);
                }
                _ => break,
            }
        }

        Ok(element)
    }
}

#[cfg(test)]
mod test {
    use cssparser::{ParserInput, Parser};
    use crate::parser::{Parse, selector::{AttributeSelector, Matcher, PseudoElement, PseudoClass, Direction}};

    use super::CompoundSelector;

    #[test]
    fn parse_success() {
        let expected = CompoundSelector {
            tag: Some("p".into()),
            id: Some("something".into()),
            classes: vec!["green".into(), "bold".into()],
            attributes: vec![
                AttributeSelector { name: "data-valid".into(), matcher: Matcher::Exists, expects: None, insensitive: false },
                AttributeSelector { name: "data-info".into(), matcher: Matcher::Include, expects: Some("data".into()), insensitive: true }
            ],
            pseudo_element: Some(Box::new(PseudoElement::Before)),
            pseudo_class: Some(Box::new(PseudoClass::FirstChild))
        };
        let src = "p#something.green.bold[data-valid][data-info~=data i]:first-child::before";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = CompoundSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }

    #[test]
    fn parse_pseudo() {
        let expected = CompoundSelector {
            pseudo_element: Some(Box::new(PseudoElement::Part("a".into()))),
            pseudo_class: Some(Box::new(PseudoClass::Dir(Direction::Ltr))),
            ..Default::default()
        };
        let src = ":dir(ltr)::part(a)";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = CompoundSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }
}
