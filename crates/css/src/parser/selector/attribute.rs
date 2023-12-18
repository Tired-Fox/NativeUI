use crate::parser::stylesheet::StyleParseError;
use crate::parser::Parse;
use cssparser::{ParseError, ParseErrorKind, Parser, Token};
use std::fmt::{Display, Formatter};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct AttributeSelector {
    pub name: String,
    pub expects: Option<String>,
    pub matcher: Matcher,
    pub insensitive: bool,
}

impl<'i> Display for AttributeSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}{}{}{}]",
            self.name,
            self.matcher,
            self.expects
                .as_ref()
                .map(|v| format!("{:?}", v))
                .unwrap_or(String::new()),
            if self.insensitive { " i" } else { "" }
        )
    }
}

impl<'i> AttributeSelector {
    pub fn matches(&self, value: Option<&str>) -> bool {
        self.matcher
            .matches(self.expects.as_ref().clone(), value, self.insensitive)
    }
}

impl Parse for AttributeSelector {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        input.parse_nested_block(|i| {
            let mut attribute = AttributeSelector::default();
            attribute.name = i.expect_ident()?.to_string();

            loop {
                let next = i.next();
                match next {
                    Ok(Token::Delim('=')) => {
                        attribute.matcher = Matcher::Equal;
                    }
                    Ok(Token::IncludeMatch) => {
                        attribute.matcher = Matcher::Include;
                    }
                    Ok(Token::DashMatch) => {
                        attribute.matcher = Matcher::Dash;
                    }
                    Ok(Token::PrefixMatch) => {
                        attribute.matcher = Matcher::Prefix;
                    }
                    Ok(Token::SuffixMatch) => {
                        attribute.matcher = Matcher::Suffix;
                    }
                    Ok(Token::SubstringMatch) => {
                        attribute.matcher = Matcher::Substring;
                    }
                    Ok(Token::Delim('i')) => {
                        attribute.insensitive = true;
                    }
                    Ok(Token::Ident(string)) if string.as_ref() == "i" => {
                        attribute.insensitive = true;
                    }
                    Ok(Token::QuotedString(string)) | Ok(Token::Ident(string)) => {
                        if attribute.matcher == Matcher::Exists {
                            return Err(ParseError {
                                kind: ParseErrorKind::Custom(StyleParseError::ExpectedCombinator),
                                location: i.current_source_location(),
                            });
                        }
                        attribute.expects = Some(string.to_string());
                    }
                    _ => break,
                }
            }
            Ok(attribute)
        })
    }
}

#[derive(Debug, Default, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum Matcher {
    #[default]
    Exists,
    Equal,
    Include,
    Dash,
    Prefix,
    Suffix,
    Substring,
}

impl Display for Matcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Matcher::Equal => "=",
                Matcher::Include => "~=",
                Matcher::Dash => "|=",
                Matcher::Prefix => "^=",
                Matcher::Suffix => "$=",
                Matcher::Substring => "*=",
                _ => "",
            }
        )
    }
}

impl Matcher {
    pub fn matches<'i>(
        &self,
        pattern: Option<&String>,
        value: Option<&str>,
        case_sensitive: bool,
    ) -> bool {
        let (pattern, value) = if case_sensitive {
            (
                pattern.map(|s| s.to_ascii_lowercase()),
                value
                    .map(|v| v.to_ascii_lowercase())
                    .unwrap_or(String::new()),
            )
        } else {
            (
                pattern.map(|s| s.to_string()),
                value.map(|v| v.to_string()).unwrap_or(String::new()),
            )
        };

        if pattern.is_none() {
            if let Matcher::Exists = self {
                return value.is_empty() || value.as_str() == "true";
            }
            return false;
        }

        let pattern = pattern.unwrap();
        match self {
            Matcher::Equal => pattern == value,
            Matcher::Include => {
                pattern == value
                    || value
                        .split(' ')
                        .collect::<Vec<&str>>()
                        .contains(&pattern.as_str())
            }
            Matcher::Dash => pattern == value || value.starts_with(&format!("{}-", pattern)),
            Matcher::Prefix => value.starts_with(&pattern),
            Matcher::Suffix => value.ends_with(&pattern),
            Matcher::Substring => value.contains(&pattern),
            _ => return false,
        }
    }
}

#[cfg(test)]
mod test {
    use cssparser::{Parser, ParserInput};

    use crate::parser::{
        selector::{AttributeSelector, Matcher},
        Parse,
    };

    #[test]
    fn parse_exists() {
        let mut expected = AttributeSelector {
            name: "data-value".into(),
            matcher: Matcher::Exists,
            expects: None,
            insensitive: false,
        };

        let src = "[data-value]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(None));
        assert!(result.as_ref().unwrap().matches(Some("true")));

        let src = "[data-value i]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        expected.insensitive = true;
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(None));
        assert!(result.as_ref().unwrap().matches(Some("TRue")));
    }
    #[test]
    fn parse_equal() {
        let mut expected = AttributeSelector {
            name: "data-value".into(),
            matcher: Matcher::Equal,
            expects: Some("test".into()),
            insensitive: false,
        };

        let src = "[data-value=test]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(Some("test")));

        let src = "[data-value=\"test\"]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(Some("test")));

        let src = "[data-value=\"test\" i]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        expected.insensitive = true;
        parser.next();
        let result = AttributeSelector::parse(&mut parser);

        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(Some("TeST")));
    }
    #[test]
    fn parse_include() {
        let expected = AttributeSelector {
            name: "data-value".into(),
            matcher: Matcher::Include,
            expects: Some("val".into()),
            insensitive: false,
        };

        let src = "[data-value~=val]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(Some("val")));
        assert!(result
            .as_ref()
            .unwrap()
            .matches(Some("some space seperated list of val")));
    }
    #[test]
    fn parse_dash() {
        let expected = AttributeSelector {
            name: "data-value".into(),
            matcher: Matcher::Dash,
            expects: Some("val".into()),
            insensitive: false,
        };

        let src = "[data-value|=val]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(Some("val")));
        assert!(result.as_ref().unwrap().matches(Some("val-dashed")));
    }
    #[test]
    fn parse_prefix() {
        let expected = AttributeSelector {
            name: "data-value".into(),
            matcher: Matcher::Prefix,
            expects: Some("val".into()),
            insensitive: false,
        };

        let src = "[data-value^=val]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(Some("val")));
        assert!(result.as_ref().unwrap().matches(Some("valdashed")));
    }
    #[test]
    fn parse_suffix() {
        let expected = AttributeSelector {
            name: "data-value".into(),
            matcher: Matcher::Suffix,
            expects: Some("val".into()),
            insensitive: false,
        };

        let src = "[data-value$=val]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(Some("val")));
        assert!(result.as_ref().unwrap().matches(Some("someval")));
    }
    #[test]
    fn parse_substring() {
        let expected = AttributeSelector {
            name: "data-value".into(),
            matcher: Matcher::Substring,
            expects: Some("val".into()),
            insensitive: false,
        };

        let src = "[data-value*=val]";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        parser.next();
        let result = AttributeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap() == &expected);
        assert!(result.as_ref().unwrap().matches(Some("val")));
        assert!(result.as_ref().unwrap().matches(Some("somevalbetween")));
    }
}
