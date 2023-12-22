use crate::parser::at_rule::{AtRule, AtRulePrelude};
use crate::parser::types::Declaration;
use crate::parser::selector::SelectorList;
use crate::parser::stylesheet::{QualifiedRule, Rule};
use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, ParseErrorKind, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser,
};
use crate::parser::error::{Error, StyleParseError};

#[derive(Debug, Default)]
pub struct NestedParser {
    pub rules: Vec<Box<Rule>>,
    pub declerations: Vec<Declaration>,
    pub errors: Vec<Error>,
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
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut nested = NestedParser::default();
        let mut iter = RuleBodyParser::new(input, &mut nested);
        while let Some(result) = iter.next() {
            match result {
                Ok(()) => {}
                Err((error, slice)) => {
                    let location = error.location;
                    let error = match error.kind {
                        ParseErrorKind::Custom(custom) => custom,
                        kind => {
                            eprintln!("{:?}", kind);
                            StyleParseError::Unkown
                        }
                    };
                    self.errors.push(Error {
                        kind: error,
                        line: location.line,
                        column: location.column,
                        ..Default::default()
                    });
                }
            }
        }
        self.rules.push(Box::new(Rule::Qualified(QualifiedRule {
            selectors: prelude,
            rules: nested.rules,
            declarations: nested.declerations,
        })));
        self.errors.extend(nested.errors);
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
        // TODO: Custom at rule block parser
        todo!();
    }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
    ) -> Result<Self::AtRule, ()> {
        self.rules.push(Box::new(Rule::At(AtRule {
            prelude,
            ..Default::default()
        })));
        Ok(())
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
        self.declerations.push(Declaration::parse(name, input)?);
        Ok(())
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
