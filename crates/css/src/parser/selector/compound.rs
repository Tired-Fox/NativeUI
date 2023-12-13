use std::fmt::Display;
use cssparser::{CowRcStr, ParseError, ParseErrorKind, Parser, Token};
use crate::parser::Parse;
use crate::parser::selector::{PseudoSelector};
use crate::parser::selector::attribute::AttributeSelector;
use crate::parser::stylesheet::StyleParseError;

#[derive(Debug, Default)]
pub struct CompoundSelector<'i> {
    pub tag: Option<CowRcStr<'i>>,
    pub id: Option<CowRcStr<'i>>,
    pub classes: Vec<CowRcStr<'i>>,
    pub attributes: Vec<AttributeSelector<'i>>,
    pub pseudo: Vec<PseudoSelector<'i>>,
}

impl<'i> Display for CompoundSelector<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}{}{}{}{}",
           self.tag.as_ref().map(|v| v.as_ref()).unwrap_or(""),
           self.id.as_ref().map(|v| format!("#{}", v)).unwrap_or("".to_string()),
           self.classes.iter().map(|v| format!(".{}", v)).collect::<Vec<String>>().join(""),
           self.attributes.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(""),
           self.pseudo.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(""),
       )
    }
}

impl<'i, 't> Parse<'i, 't> for CompoundSelector<'i> {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let mut element = CompoundSelector::default();

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
                    match input.next() {
                        Ok(Token::Ident(name)) => {
                            element.pseudo.push(PseudoSelector::Function { name: name.clone() })
                        },
                        Ok(Token::Function(name)) => {
                            element.pseudo.push(PseudoSelector::Function { name: name.clone() });
                            input.parse_nested_block(|i| {
                                // TODO: Capture arguments
                                while let Ok(t) = i.next() {
                                    println!("Function Arg Token: {:?}", t);
                                }
                                Ok::<(), ParseError<'i, StyleParseError>>(())
                            });
                        },
                        Ok(Token::Colon) => {
                            let name = input.expect_ident()?;
                            element.pseudo.push(PseudoSelector::Element(name.clone()))
                        },
                        _ => {
                            return Err(ParseError {
                                kind: ParseErrorKind::Custom(StyleParseError::InvalidPseudoSelector),
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