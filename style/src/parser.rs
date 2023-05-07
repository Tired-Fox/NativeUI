use cssparser::{
    AtRuleParser, BasicParseError, CowRcStr, DeclarationListParser, DeclarationParser, ParseError,
    Parser, QualifiedRuleParser, Token, ParserState
};

use super::rules::*;

#[derive(Debug)]
pub struct Rule {
    pub key: String,
    pub styles: Vec<Style>,
}

#[derive(Debug)]
pub struct RuleParser;

impl<'i> AtRuleParser<'i> for RuleParser {
    type Prelude = ();
    type AtRule = Rule;
    type Error = BasicParseError<'i>;
}

impl<'i> QualifiedRuleParser<'i> for RuleParser {
    type Prelude = String;
    type QualifiedRule = Rule;
    type Error = BasicParseError<'i>;

    /// Parses out the selector.
    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let location = input.current_source_location();

        let selector = match input.next()? {
            Token::Ident(ref element_name) => element_name.to_string(),
            t => {
                return Err(location.new_unexpected_token_error(t.clone()));
            }
        };

        // If there's a next, someone is writing their code assuming cascading. Let's... warn them.
        /*match input.next()? {
            Ok(_) => {},
            Err(e) => {}
        };*/

        Ok(selector)
    }

    /// Parses the block (`{...}`) into a Rule struct.
    fn parse_block<'t>(
        &mut self,
        key: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let styles = DeclarationListParser::new(input, StyleParser {}).collect::<Vec<_>>();

        Ok(Rule {
            key,
            styles: styles
                .into_iter()
                .filter_map(|decl| {
                    if !decl.is_ok() {
                        eprintln!("{:?}", decl);
                    }

                    decl.ok()
                })
                .collect(),
        })
    }
}

/// Contains logic for matching CSS attributes to their `Styles` counterpart.
#[derive(Debug)]
pub struct StyleParser {}

/// Types, etc.
impl<'i> AtRuleParser<'i> for StyleParser {
    type Prelude= ();
    type AtRule = Style;
    type Error = BasicParseError<'i>;
}

/// A utility method for dereferencing a value, to make some code later on a bit more clean.
fn ident<'a>(token: &'a Token) -> &'a str {
    match token {
        Token::Ident(ref value) => &*value,
        _ => ""
    }
}

impl<'i> QualifiedRuleParser<'i> for StyleParser {
    type Prelude = ();
    type QualifiedRule = Style;
    type Error = BasicParseError<'i>;
}

impl<'i> DeclarationParser<'i> for StyleParser {
    type Declaration = Style;
    type Error = BasicParseError<'i>;

    /// Parses a value (e.g, `background-color: #307ace;`) into a `Styles` value.
    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let style = match &*name {
            "font-style" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "normal" => Style::FontStyle(FontStyle::Normal),
                "italic" => Style::FontStyle(FontStyle::Italic),
                "oblique" => Style::FontStyle(FontStyle::Oblique),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            "height" => Style::Height(parse_floaty_value(input)?),
            "width" => Style::Width(parse_floaty_value(input)?),
            t => {
                let location = input.current_source_location();
                return Err(location.new_unexpected_token_error(Token::Ident(t.to_string().into())));
            }
        };

        Ok(style)
    }
}

/// A utility method for handling some float values.
/// Mostly used to reduce code verbosity in the massive switch table for `Styles` parsing.
fn parse_floaty_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Unit, BasicParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;

    match token {
        Token::Number { value, .. } => Ok(Unit::PX(*value)),
        Token::Dimension {value, unit, ..} => Ok(Unit::from_unit(unit, value)),
        Token::Percentage { unit_value, ..} => Ok(Unit::Percent(*unit_value)),
        _ => Err(location.new_basic_unexpected_token_error(token.clone()))
    }
}
