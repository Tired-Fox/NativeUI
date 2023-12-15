use crate::parser::at_rule::{AtRule, AtRulePrelude};
use crate::parser::selector::SelectorList;
use crate::parser::stylesheet::{Rule, StyleParseError};
use cssparser::{AtRuleParser, CowRcStr, DeclarationParser, ParseError, ParseErrorKind, Parser, ParserState, QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, SourcePosition};

#[derive(Debug, Default)]
pub struct NestedParser {
    pub rules: Vec<Rule>,
}

impl<'i> QualifiedRuleParser<'i> for NestedParser {
    type Prelude = SelectorList;
    type QualifiedRule = ();
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
        let mut nested = NestedParser::default();
        let mut iter = RuleBodyParser::new(input, &mut nested);
        while let Some(result) = iter.next() {
            match result {
                Ok(()) => {},
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
                }
            }
        }
        // TODO: Join nested rules into stylesheet rules
        Ok(())
    }
}

impl<'i> AtRuleParser<'i> for NestedParser {
    type Prelude = AtRulePrelude;
    type AtRule = ();
    type Error = StyleParseError;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        // TODO: Validate at-rule is valid for scope
        AtRule::parse_prelude(name, input)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        println!("[Nested::AtRule] Block");
        // TODO: Custom at rule block parser
        todo!();
    }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
    ) -> Result<Self::AtRule, ()> {
        self.rules.push(Rule::At(AtRule {
            prelude,
            ..Default::default()
        }));
    }
}

impl<'i> DeclarationParser<'i> for NestedParser {
    type Declaration = ();
    type Error = StyleParseError;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        todo!()
    }
}

impl<'i> RuleBodyItemParser<'i, (), StyleParseError> for NestedParser {
    fn parse_declarations(&self) -> bool {
        // TODO: Set based on config
        true
    }

    fn parse_qualified(&self) -> bool {
        // TODO: Set based on config
        true
    }
}
