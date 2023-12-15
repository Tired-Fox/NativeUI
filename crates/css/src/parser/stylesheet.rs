use std::process::exit;
use std::{fs, path::Path};

use crate::parser::at_rule::{AtRule, AtRulePrelude};
use crate::parser::nested::NestedParser;
use crate::parser::selector::SelectorList;
use cssparser::{
    AtRuleParser, CowRcStr, ParseError, ParseErrorKind, Parser, ParserInput, ParserState,
    QualifiedRuleParser, SourcePosition, StyleSheetParser,
};

use crate::parser::Parse;

macro_rules! not_implemented {
    ($location: expr) => {
        Err(ParseError {
            kind: ParseErrorKind::Custom(StyleParseError::NotImplemented),
            location: $location,
        })
    };
}

#[derive(Debug)]
pub struct QualifiedRule;

#[derive(Debug)]
pub enum StyleParseError {
    NotImplemented,
    Unkown,
    UnkownSyntax,
    UnkownAtRule,
    UnkownPseudoClass,
    UnkownPseudoElement,
    EndOfStream,
    ExpectedCombinator,
    ExpectedIdentOrString,
    ExpectedEqualSign,
    DuplicateIDSelector,
    DuplicateElementSelector,
    InvalidPseudoSelector,
    InvalidNthFormat,
    UnexpectedCombinator,
    ExpectedArguments,
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

pub fn parse_styles<'i, 't, P>(input: &mut Parser<'i, 't>, parser: &mut P)
where
    P: QualifiedRuleParser<'i, QualifiedRule = SourcePosition, Error = StyleParseError>
        + AtRuleParser<'i, AtRule = SourcePosition, Error = StyleParseError>,
{
    let mut iter = StyleSheetParser::new(input, parser);
    while let Some(result) = iter.next() {
        match result {
            Err((error, slice)) => {
                let location = error.location;
                let error = match error.kind {
                    ParseErrorKind::Custom(custom) => custom,
                    kind => {
                        eprintln!("{:?}", kind);
                        StyleParseError::Unkown
                    }
                };
                eprintln!(
                    "[{}:{}]: {:?}\n\n{} │ {}\n",
                    location.line, location.column, error, location.line, slice
                );
                exit(0);
            }
            Ok(start) => {
                // Used to construct a sanatized out of the input
                let end = iter.input.position().byte_index();
                // print!("{}", &src[start.byte_index()..end])
            }
        }
    }
}

impl StyleSheet {
    pub fn source(src: &str) -> Self {
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);

        let mut stylesheet = Self::default();
        parse_styles(&mut parser, &mut stylesheet);
        stylesheet
    }

    pub fn path(path: impl AsRef<Path>) -> Self {
        let src = fs::read_to_string(path.as_ref()).unwrap();
        Self::source(src.as_str())
    }
}

impl<'i> QualifiedRuleParser<'i> for StyleSheet {
    type Prelude = SelectorList<'i>;
    type QualifiedRule = SourcePosition;
    type Error = StyleParseError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        println!("[QualifiedRule] Prelude");
        SelectorList::parse(input, false)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut nested = NestedParser::default();
        parse_styles(input, &mut nested);
        // TODO: Join nested rules into stylesheet rules
        Ok(start.position())
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
        AtRule::parse_prelude(name, input)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        println!("[AtRule] Block");
        // TODO: Custom at rule block parser
        todo!();
        Ok(start.position())
    }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
    ) -> Result<Self::AtRule, ()> {
        self.rules.push(Rule::At(AtRule {
            prelude,
            block: None,
        }));
        Ok(start.position())
    }
}
