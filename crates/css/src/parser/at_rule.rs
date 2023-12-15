use cssparser::{CowRcStr, ParseError, ParseErrorKind, SourcePosition};
use crate::parser::Parse;
use crate::parser::stylesheet::StyleParseError;

#[derive(Debug)]
pub struct AtRule {
    pub prelude: AtRulePrelude,
    pub block: Option<SourcePosition>,
}

impl<'i, 't> AtRule {
    pub fn parse_prelude(name: CowRcStr<'i>, input: &mut cssparser::Parser<'i, 't>) -> Result<AtRulePrelude, ParseError<'i, StyleParseError>> {
        AtRulePrelude::parse(name, input)
    }
}

#[derive(Debug)]
pub enum AtRulePrelude {
    Charset,
    ColorProfile,
    Container,
    CounterStyle,
    Document,
    FontFace,
    FontFeatureValues,
    FontPaletteValues,
    Import,
    Keyframes,
    Layer,
    Media,
    Namespace,
    Page,
    Scope,
    Supports,
    StartingStyle
}

impl<'i, 't> AtRulePrelude {
    pub fn parse(name: CowRcStr<'i>, input: &mut cssparser::Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, StyleParseError>> {
        let location = input.current_source_location();
        let result = match name.to_ascii_lowercase().as_str() {
            "charset" => Ok(AtRulePrelude::Charset),
            "color-profile" => Ok(AtRulePrelude::ColorProfile),
            "container" => Ok(AtRulePrelude::Container),
            "counter-style" => Ok(AtRulePrelude::CounterStyle),
            "document" => Ok(AtRulePrelude::Document),
            "font-face" => Ok(AtRulePrelude::FontFace),
            "font-feature-values" => Ok(AtRulePrelude::FontFeatureValues),
            "font-palette-values" => Ok(AtRulePrelude::FontPaletteValues),
            "import" => Ok(AtRulePrelude::Import),
            "keyframes" => Ok(AtRulePrelude::Keyframes),
            "layer" => Ok(AtRulePrelude::Layer),
            "media" => Ok(AtRulePrelude::Media),
            "namespace" => Ok(AtRulePrelude::Namespace),
            "page" => Ok(AtRulePrelude::Page),
            "scope" => Ok(AtRulePrelude::Scope),
            "supports" => Ok(AtRulePrelude::Supports),
            "starting-style" => Ok(AtRulePrelude::StartingStyle),
            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(StyleParseError::UnkownAtRule),
                location
            })
        };
        while input.next().is_ok() {}
        result
    }
}
