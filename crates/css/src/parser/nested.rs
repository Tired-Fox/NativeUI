use crate::parser::at_rule::AtRulePrelude;
use crate::parser::selector::SelectorList;
use crate::parser::stylesheet::{Rule, StyleParseError};
use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, SourcePosition,
};

#[derive(Debug, Default)]
pub struct NestedParser {
    pub rules: Vec<Rule>,
}

impl<'i> QualifiedRuleParser<'i> for NestedParser {
    type Prelude = SelectorList<'i>;
    type QualifiedRule = SourcePosition;
    type Error = StyleParseError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        SelectorList::parse(input, false)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        todo!()
    }
}

impl<'i> AtRuleParser<'i> for NestedParser {
    type Prelude = AtRulePrelude;
    type AtRule = SourcePosition;
    type Error = StyleParseError;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        todo!()
    }
    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        todo!()
    }
    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
    ) -> Result<Self::AtRule, ()> {
        todo!()
    }
}

impl<'i> DeclarationParser<'i> for NestedParser {
    type Declaration = SourcePosition;
    type Error = StyleParseError;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let position = input.position();
        while input.next().is_ok() {}
        Ok(position)
    }
}
