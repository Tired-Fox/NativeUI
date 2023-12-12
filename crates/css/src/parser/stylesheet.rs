use std::{fs, path::Path};
use std::process::exit;

use cssparser::{AtRuleParser, CowRcStr, DeclarationParser, Delimiter, Delimiters, ParseError, ParseErrorKind, Parser, ParserInput, ParserState, QualifiedRuleParser, SourceLocation, SourcePosition, StyleSheetParser};
use cssparser::Token::{CurlyBracketBlock, Delim, Ident, IDHash};

macro_rules! not_implemented {
    ($location: expr) => {
        Err(ParseError {
            kind: ParseErrorKind::Custom(StyleParseError::NotImplemented),
            location: $location,
        })
    };
}

#[derive(Debug)]
pub struct AtRule;
pub enum AtRulePrelude {
    FontFace,
}

#[derive(Debug)]
pub struct QualifiedRule;
pub struct SelectorList;

#[derive(Debug)]
pub enum StyleParseError {
    NotImplemented,
    Unkown,
}

#[derive(Debug)]
pub enum Rule {
    At(AtRule),
    Qualified(QualifiedRule),
}

#[derive(Default, Debug)]
pub struct StyleSheet {
    rules: Vec<Rule>,
}

impl StyleSheet {
    pub fn source(src: &str) -> Self {
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);

        let mut stylesheet = Self::default();
        {
            let mut iter = StyleSheetParser::new(&mut parser, &mut stylesheet);
            while let Some(result) = iter.next() {
                match result {
                    Err((error, slice)) => {
                        let location = error.location;
                        let error = match error.kind {
                            ParseErrorKind::Custom(custom) => custom,
                            _ => StyleParseError::Unkown
                        };
                        eprintln!("[{}:{}]: {:?}\n\n{} │ {}\n", location.line, location.column, error, location.line, slice);
                        exit(0);
                    }
                    Ok(start) => {
                        let end = iter.input.position().byte_index();
                        print!("{}", &src[start.byte_index()..end])
                    }
                }
            }
        }
        stylesheet
    }

    pub fn path(path: impl AsRef<Path>) -> Self {
        let src = fs::read_to_string(path.as_ref()).unwrap();
        Self::source(src.as_str())
    }
}

impl<'i> QualifiedRuleParser<'i> for StyleSheet {
    type Prelude = SelectorList;
    type QualifiedRule = SourcePosition;
    type Error = StyleParseError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        input.parse_until_before(Delimiter::Comma, |i| {
            loop {
                match i.next() {
                    Err(e) => return Err(ParseError{
                        kind: ParseErrorKind::Custom(StyleParseError::Unkown),
                        location: i.current_source_location(),
                    }),
                    Ok(token) => {
                        match token {
                            IDHash(value) => {
                                println!("ID: {:?}", value);
                            },
                            Delim('.') => {
                                if let Ok(Ident(value)) = i.next() {
                                    println!("CLASS: {:?}", value);
                                } else {
                                    return Err(ParseError{
                                        kind: ParseErrorKind::Custom(StyleParseError::Unkown),
                                        location: i.current_source_location(),
                                    })
                                }
                            },
                            Ident(value) => {
                                println!("Element: {:?}", value);
                            },
                            Delim('>') => {
                                println!("Is Child")
                            }
                            token => println!("TOKEN: {:?}", token)
                        }
                    }
                }
            }
            Ok(SelectorList)
        })
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        not_implemented!(start.source_location())
    }
}

impl<'i> AtRuleParser<'i> for StyleSheet {
    type Prelude = AtRulePrelude;
    type AtRule = SourcePosition;
    type Error = StyleParseError;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        not_implemented!(input.current_source_location())
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        not_implemented!(start.source_location())
    }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
    ) -> Result<Self::AtRule, ()> {
        Err(())
    }
}
