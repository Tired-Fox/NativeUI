//! Core css parser
//!
//! Contains logic for parsing rules and properties to a token stream.
//! Uses [servo's cssparser](https://github.com/servo/rust-cssparser) with custom
//! data types to parse the data.

use cssparser::{
    AtRuleParser, BasicParseError, CowRcStr, DeclarationListParser, DeclarationParser, ParseError,
    Parser, ParserState, QualifiedRuleParser, Token,
};

use crate::{size::Size, Color};

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

        let token = input.next()?;
        let selector = match token {
            Token::Ident(ref element_name) => element_name.to_string(),
            Token::Delim(delim) => match delim {
                '.' => {
                    let token = input.next()?;
                    match token {
                        Token::Ident(ref element_name) => format!(".{}", element_name),
                        t => return Err(location.new_unexpected_token_error(t.clone())),
                    }
                },
                _ => return Err(location.new_unexpected_token_error(token.clone()))
            },
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
    type Prelude = ();
    type AtRule = Style;
    type Error = BasicParseError<'i>;
}

/// A utility method for dereferencing a value, to make some code later on a bit more clean.
fn ident<'a>(token: &'a Token) -> &'a str {
    match token {
        Token::Ident(ref value) => &*value,
        _ => "",
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
            "font-style" => {
                let s = input.current_source_location();
                let t = input.next()?;
                match ident(&t) {
                    "normal" => Style::FontStyle(FontStyle::Normal),
                    "italic" => Style::FontStyle(FontStyle::Italic),
                    "oblique" => Style::FontStyle(FontStyle::Oblique),
                    _ => {
                        return Err(s.new_unexpected_token_error(t.clone()));
                    }
                }
            }

            "background-color" => Style::BackgroundColor(Color::parse(input)?),

            "height" => Style::Height(parse_value(input)?),
            "width" => Style::Width(parse_value(input)?),

            "inset" => Style::Inset(Size::parse(input)?),
            "inset-block" => Style::InsetBlock(parse_value(input)?),
            "inset-inline" => Style::InsetInline(parse_value(input)?),
            "left" => Style::Left(parse_value(input)?),
            "top" => Style::Top(parse_value(input)?),
            "right" => Style::Right(parse_value(input)?),
            "bottom" => Style::Bottom(parse_value(input)?),

            "padding" => Style::Padding(Size::parse(input)?),
            "padding-inline" => Style::PaddingInline(parse_value(input)?),
            "padding-block" => Style::PaddingBlock(parse_value(input)?),
            "padding-left" => Style::PaddingLeft(parse_value(input)?),
            "padding-top" => Style::PaddingTop(parse_value(input)?),
            "padding-right" => Style::PaddingRight(parse_value(input)?),
            "padding-bottom" => Style::PaddingBottom(parse_value(input)?),

            "margin" => Style::Margin(Size::parse(input)?),
            "margin-inline" => Style::MarginInline(parse_value(input)?),
            "margin-block" => Style::MarginBlock(parse_value(input)?),
            "margin-left" => Style::MarginLeft(parse_value(input)?),
            "margin-top" => Style::MarginTop(parse_value(input)?),
            "margin-right" => Style::MarginRight(parse_value(input)?),
            "margin-bottom" => Style::MarginBottom(parse_value(input)?),

            t => {
                let location = input.current_source_location();
                return Err(location.new_unexpected_token_error(Token::Ident(t.to_string().into())));
            }
        };

        Ok(style)
    }
}

/// A utility method for handling some values.
/// Mostly used to reduce code verbosity in the massive switch table for `Styles` parsing.
fn parse_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Unit, BasicParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;

    match token {
        Token::Number { value, .. } => Ok(Unit::PX(*value)),
        Token::Dimension { value, unit, .. } => Ok(Unit::from_unit(unit, value)),
        Token::Percentage { unit_value, .. } => Ok(Unit::Percent(*unit_value)),
        _ => Err(location.new_basic_unexpected_token_error(token.clone())),
    }
}
