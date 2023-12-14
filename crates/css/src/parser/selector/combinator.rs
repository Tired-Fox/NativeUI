use std::fmt::{Display, Formatter};
use cssparser::{ParseError, ParseErrorKind, Parser, Token};
use crate::parser::Parse;
use crate::parser::stylesheet::StyleParseError;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Combinator {
    #[default]
    Descendant,
    /// `>`
    Child,
    /// `+`
    AdjacentSibling,
    /// `~`
    Sibling,
    /// `||`
    Column,
    /// `|`
    Namespace
}

impl Display for Combinator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Combinator::Descendant => " ",
            Combinator::Child => ">",
            Combinator::AdjacentSibling => "+",
            Combinator::Sibling => "~",
            Combinator::Column => "||",
            Combinator::Namespace => "|",
        })
    }
}

impl<'i, 't> Parse<'i, 't> for Combinator {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let mut whitespaced = false;
        loop {
            match input.next_including_whitespace() {
                Ok(Token::Delim('>')) => return Ok(Combinator::Child),
                Ok(Token::Delim('+')) => return Ok(Combinator::AdjacentSibling),
                Ok(Token::Delim('~')) => return Ok(Combinator::Sibling),
                Ok(Token::Delim('|')) => {
                    if let Ok(_) = input.expect_delim('|') {
                        return Ok(Combinator::Column)
                    }
                    return Ok(Combinator::Namespace)
                },
                Ok(Token::WhiteSpace(_)) => whitespaced = true,
                _ => {
                    if whitespaced {
                        return Ok(Combinator::Descendant);
                    } else {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(StyleParseError::ExpectedCombinator),
                            location: input.current_source_location(),
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use cssparser::{ParserInput, Parser};

    use crate::parser::Parse;

    use super::Combinator;

    #[test]
    fn parse_success() {
        let src = "> p";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = Combinator::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == Combinator::Child);
    }

    #[test]
    fn parse_fail() {
        let src = "p";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = Combinator::parse(&mut parser);
        assert!(result.is_err());
    }
}
