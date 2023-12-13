use crate::parser::stylesheet::StyleParseError;
use crate::parser::Parse;
use cssparser::{CowRcStr, ParseError, ParseErrorKind, Parser, Token};
use std::fmt::{Display, Formatter};

#[derive(Default, Debug)]
pub struct AttributeSelector<'i> {
    name: CowRcStr<'i>,
    expects: Option<CowRcStr<'i>>,
    matcher: Matcher,
    insensitive: bool,
}

impl<'i> Display for AttributeSelector<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}{}{}{}]",
            self.name,
            self.matcher,
            self.expects.as_ref().map(|v| format!("{:?}", v)).unwrap_or(String::new()),
            if self.insensitive { " i" } else { "" }
        )
    }
}

impl<'i> AttributeSelector<'i> {
    pub fn matches(&self, value: Option<&str>) -> bool {
        self.matcher
            .matches(self.expects.clone(), value, self.insensitive)
    }
}

impl<'i, 't> Parse<'i, 't> for AttributeSelector<'i> {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        input.parse_nested_block(|i| {
            let mut attribute = AttributeSelector::default();
            attribute.name = i.expect_ident()?.clone();

            macro_rules! expect_equal {
                ($input: ident) => {
                    if let Err(_) = $input.expect_delim('=') {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(StyleParseError::ExpectedEqualSign),
                            location: $input.current_source_location(),
                        });
                    }
                };
            }

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
                        expect_equal!(i);
                        attribute.matcher = Matcher::Dash;
                    }
                    Ok(Token::PrefixMatch) => {
                        expect_equal!(i);
                        attribute.matcher = Matcher::Prefix;
                    }
                    Ok(Token::SuffixMatch) => {
                        expect_equal!(i);
                        attribute.matcher = Matcher::Suffix;
                    }
                    Ok(Token::SubstringMatch) => {
                        expect_equal!(i);
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
                        attribute.expects = Some(string.clone());
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
                _ => ""
            }
        )
    }
}

impl Matcher {
    pub fn matches<'i>(
        &self,
        pattern: Option<CowRcStr<'i>>,
        value: Option<&str>,
        case_sensitive: bool,
    ) -> bool {
        let (pattern, value) = if case_sensitive {
            (
                pattern.map(|s| s.to_lowercase()),
                value.map(|v| v.to_lowercase()).unwrap_or(String::new()),
            )
        } else {
            (
                pattern.map(|s| s.to_string()),
                value.map(|v| v.to_string()).unwrap_or(String::new()),
            )
        };

        if pattern.is_none() {
            return false;
        }

        let pattern = pattern.unwrap();
        match self {
            Matcher::Exists => true,
            Matcher::Equal => pattern == value,
            Matcher::Include => {
                pattern == value
                    || pattern
                        .split(' ')
                        .collect::<Vec<&str>>()
                        .contains(&value.as_str())
            }
            Matcher::Dash => pattern == value || pattern.starts_with(&format!("{}-", value)),
            Matcher::Prefix => pattern.starts_with(&value),
            Matcher::Suffix => pattern.ends_with(&value),
            Matcher::Substring => pattern.contains(&value),
        }
    }
}
