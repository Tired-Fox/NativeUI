//! [CSS Syntax L3](https://www.w3.org/TR/css-syntax-3/#syntax)
//! TODO: check for source map
//! TODO: continuation bytes
//! TODO: 4byte leading bytes

use crate::parser::tokenizer::Token::BadString;
use std::borrow::Cow;

#[derive(Default, Debug)]
pub enum HashType {
    Id,
    #[default]
    Unrestricted,
}

#[derive(Default, Debug)]
pub enum Unit {
    Rem,
    #[default]
    Px,
    Em,
    Cm,
    In,
    Pt,
    Pc,
    Percent,
    Ch,
    Mm,
}

#[derive(Default, Debug)]
pub enum NumberType {
    #[default]
    Integer,
    Number,
}

#[derive(Debug)]
pub enum Token<'i> {
    Ident(&'i str),
    Function(&'i str),
    AtKeyword(&'i str),
    Hash {
        value: &'i str,
        tag: HashType,
    },
    String(String),
    Url(&'i str),
    Whitespace(&'i str),
    Comment(&'i str),
    Delim(char),
    Number {
        value: f64,
        flag: NumberType,
    },
    Percentage(f64),
    Dimension {
        value: f64,
        flag: NumberType,
        unit: &'i str,
    },
    BadString,
    BadUrl,
    CDO,
    CDC,
    Colon,
    Semicolon,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'i> Tokenizer<'i> {
    pub fn new(input: &'i str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    #[inline]
    pub fn next_byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.position).copied()
    }

    #[inline]
    pub fn next_byte_unchecked(&self) -> u8 {
        self.input.as_bytes()[self.position]
    }

    #[inline]
    pub fn byte_at(&self, n: usize) -> Option<u8> {
        self.input.as_bytes().get(self.position + n).copied()
    }

    #[inline]
    pub fn consume(&mut self, n: usize) {
        self.position += n;
    }

    #[inline]
    pub fn reconsume(&mut self, n: usize) {
        self.position -= n;
    }

    #[inline]
    pub fn consume_newline(&mut self) {
        let byte = self.next_byte_unchecked();
        debug_assert!(byte == b'\n' || byte == b'\r' || byte == b'\x0C');
        self.position += 1;

        if byte == b'\r' && self.next_byte() == Some(b'\n') {
            self.position += 1;
        }

        self.line += 1;
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    #[inline]
    pub fn starts_with(&self, s: &str) -> bool {
        !self.is_eof() && (&self.input[self.position..]).starts_with(s)
    }

    #[inline]
    pub fn starts_with_escape(&self) -> bool {
        if self.position() + 1 >= self.input.len() {
            return false;
        }
        valid_escape(
            self.next_byte_unchecked(),
            self.input.as_bytes()[self.position + 1],
        )
    }

    #[inline]
    pub fn starts_with_ident(&self) -> bool {
        if self.position() + 2 >= self.input.len() {
            return false;
        }

        let first = self.next_byte_unchecked();
        let second = self.input.as_bytes()[self.position + 1];
        let third = self.input.as_bytes()[self.position + 2];

        if first == b'-' {
            is_ident_start(second) || second == b'-' || valid_escape(second, third)
        } else if is_ident_start(first) {
            true
        } else {
            valid_escape(first, self.input.as_bytes()[self.position + 1])
        }
    }

    #[inline]
    pub fn starts_with_number(&self) -> bool {
        if self.position + 2 >= self.input.len() {
            return false;
        }

        let first = self.next_byte_unchecked();
        let second = self.input.as_bytes()[self.position + 1];
        let third = self.input.as_bytes()[self.position + 2];

        if first == b'-' || first == b'+' {
            second.is_ascii_digit() || (second == b'.' && third.is_ascii_digit())
        } else if first == b'.' {
            second.is_ascii_digit()
        } else {
            first.is_ascii_digit()
        }
    }

    #[inline]
    pub fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            let byte = self.next_byte_unchecked();
            match byte {
                b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' => {
                    self.position += 1;
                },
                _ => {
                    break;
                }
            }
        }
    }

    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> &'i str {
        &self.input[start..end]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.input.len()
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }

    pub fn next(&mut self) -> Result<Token<'i>, ()> {
        next_token(self)
    }
}

fn valid_escape(first: u8, second: u8) -> bool {
    first == b'\\' && second != b'\n'
}
fn is_ident_start(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'_')
}
fn is_ident(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-')
}

fn next_token<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<Token<'i>, ()> {
    if tokenizer.is_eof() {
        return Err(());
    }

    match tokenizer.next_byte_unchecked() {
        b'\n' | b'\r' | b'\x0C' | b' ' | b'\t' => consume_whitespace(tokenizer),
        b'/' => {
            if tokenizer.starts_with("/*") {
                Ok(Token::Comment(consume_comment(tokenizer)?))
            } else {
                tokenizer.consume(1);
                Ok(Token::Delim('/'))
            }
        }
        b'\\' => {
            if tokenizer.starts_with_escape() {
                consume_ident_like(tokenizer)
            } else {
                Err(())
            }
        }
        b'"' | b'\'' => consume_string(tokenizer),
        b'(' => consumes(tokenizer, Token::LeftParen),
        b')' => consumes(tokenizer, Token::RightParen),
        b'[' => consumes(tokenizer, Token::LeftBracket),
        b']' => consumes(tokenizer, Token::RightBracket),
        b'{' => consumes(tokenizer, Token::LeftBrace),
        b'}' => consumes(tokenizer, Token::RightBrace),
        b',' => consumes(tokenizer, Token::Comma),
        b':' => consumes(tokenizer, Token::Colon),
        b';' => consumes(tokenizer, Token::Semicolon),
        b'@' => {
            tokenizer.consume(1);
            if tokenizer.starts_with_ident() {
                Ok(Token::AtKeyword(consume_ident(tokenizer)))
            } else {
                Ok(Token::Delim('@'))
            }
        }
        b'<' => {
            if tokenizer.starts_with("<!--") {
                tokenizer.consume(4);
                Ok(Token::CDO)
            } else {
                consumes(tokenizer, Token::Delim('<'))
            }
        }
        b'-' => {
            if tokenizer.starts_with_number() {
                consume_numeric(tokenizer)
            } else {
                if tokenizer.byte_at(1) == Some(b'-') && tokenizer.byte_at(2) == Some(b'>') {
                    tokenizer.consume(3);
                    Ok(Token::CDC)
                } else if tokenizer.starts_with_ident() {
                    consume_ident_like(tokenizer)
                } else {
                    consumes(tokenizer, Token::Delim('-'))
                }
            }
        }
        b'+' => {
            if tokenizer.starts_with_number() {
                consume_numeric(tokenizer)
            } else {
                consumes(tokenizer, Token::Delim('-'))
            }
        },
        b'.' => {
            if tokenizer.starts_with_number() {
                consume_numeric(tokenizer)
            } else {
                consumes(tokenizer, Token::Delim('.'))
            }
        }
        b'#' => {
            tokenizer.consume(1);
            if !tokenizer.is_eof()
                && (is_ident(tokenizer.next_byte_unchecked()) || tokenizer.starts_with_escape())
            {
                let mut tag = HashType::default();
                if tokenizer.starts_with_ident() {
                    tag = HashType::Id;
                }
                return Ok(Token::Hash {
                    value: consume_ident(tokenizer),
                    tag,
                });
            }
            Ok(Token::Delim('#'))
        },
        b'0'..=b'9' => consume_numeric(tokenizer),
        b'a'..=b'z' | b'A'..=b'Z' | b'_' => consume_ident_like(tokenizer),
        byte => consumes(tokenizer, Token::Delim(byte as char)),
    }
}

pub fn consumes<'i>(tokenizer: &mut Tokenizer<'i>, token: Token<'i>) -> Result<Token<'i>, ()> {
    tokenizer.consume(1);
    Ok(token)
}

pub fn consume_ident_like<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<Token<'i>, ()> {
    let ident = consume_ident(tokenizer);
    if ident == "url" && tokenizer.next_byte() == Some(b'(') {
        tokenizer.consume(1);
        tokenizer.skip_whitespace();
        if tokenizer.next_byte() == Some(b'\'') || tokenizer.next_byte() == Some(b'"') {
            Ok(Token::Function(ident))
        } else {
            Ok(Token::Url(ident))
        }
    } else if tokenizer.next_byte() == Some(b'(') {
        tokenizer.consume(1);
        Ok(Token::Function(ident))
    } else {
        Ok(Token::Ident(ident))
    }
}

pub fn consume_numeric<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<Token<'i>, ()> {
    let (number, ty) = consume_number(tokenizer)?;
    if tokenizer.starts_with_ident() {
        let unit = consume_ident(tokenizer);
        println!("{:?}", unit);
        Ok(Token::Dimension {
            value: number,
            unit,
            flag: ty,
        })
    } else if tokenizer.next_byte() == Some(b'%') {
        tokenizer.consume(1);
        Ok(Token::Percentage(number))
    } else {
        Ok(Token::Number {
            value: number,
            flag: ty,
        })
    }
}

pub fn consume_number<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<(f64, NumberType), ()> {
    let start = tokenizer.position();
    let mut ty = NumberType::default();
    let mut repr = String::new();

    // Example: -123.45e-16
    // Assume: ([+-])?(\d+)?(\.\d+)?([eE][+-]?\d+)?
    // Parts:
    //   1. ([+-])?
    //   2. (\d+)?(\.\d+)?([eE][+-]?\d+)?
    //   3. (\.\d+)?([eE][+-]?\d+)?
    //   4. ([eE][+-]?\d+)?

    // Part 1
    match tokenizer.next_byte() {
        Some(val) if val == b'-' || val == b'+' => {
            repr.push(val as char);
            tokenizer.consume(1);
        }
        _ => {}
    }

    // Part 2
    match tokenizer.next_byte() {
        Some(val) if val.is_ascii_digit() => {
            while !tokenizer.is_eof() {
                let mut byte = tokenizer.next_byte_unchecked();
                if !byte.is_ascii_digit() {
                    break;
                }
                repr.push(byte as char);
                tokenizer.consume(1);
            }
        }
        _ => {}
    }

    // Part 3
    if tokenizer.position() + 1 < tokenizer.len()
        && tokenizer.next_byte_unchecked() == b'.'
        && tokenizer
            .input
            .as_bytes()
            .get(tokenizer.position() + 1)
            .is_some_and(|val| val.is_ascii_digit())
    {
        repr.push(tokenizer.next_byte_unchecked() as char);
        tokenizer.consume(1);
        repr.push(tokenizer.next_byte_unchecked() as char);
        tokenizer.consume(1);
        ty = NumberType::Number;

        while !tokenizer.is_eof() {
            let mut byte = tokenizer.next_byte_unchecked();
            if !byte.is_ascii_digit() {
                break;
            }
            repr.push(byte as char);
            tokenizer.consume(1);
        }
    }

    // Part 4
    match tokenizer.next_byte() {
        Some(b'e') | Some(b'E') => {
            let e = tokenizer.next_byte_unchecked();
            tokenizer.consume(1);
            let numbers = match tokenizer.next_byte() {
                Some(b'-') | Some(b'+') => {
                    let sign = tokenizer.next_byte_unchecked();
                    tokenizer.consume(1);
                    match tokenizer.next_byte() {
                        Some(val) if val.is_ascii_digit() => {
                            repr.push(e as char);
                            repr.push(sign as char);
                            true
                        },
                        _ =>  {
                            tokenizer.reconsume(2);
                            false
                        }
                    }
                },
                Some(val) if val.is_ascii_digit() => {
                    repr.push(e as char);
                    true
                },
                _ => {
                    tokenizer.reconsume(1);
                    false
                }
            };
            if numbers {
                while !tokenizer.is_eof() {
                    let mut byte = tokenizer.next_byte_unchecked();
                    if !byte.is_ascii_digit() {
                        break;
                    }
                    repr.push(byte as char);
                    tokenizer.consume(1);
                }
            }
        },
        _ => {}
    }

    match repr.parse::<f64>() {
        Ok(val) => Ok((val, ty)),
        _ => Err(()),
    }
}

pub fn consume_ident<'i>(tokenizer: &mut Tokenizer<'i>) -> &'i str {
    let start = tokenizer.position();

    while !tokenizer.is_eof() {
        let byte = tokenizer.next_byte_unchecked();
        if is_ident(byte) {
            tokenizer.consume(1);
        } else if tokenizer.starts_with_escape() {
            consume_escape(tokenizer);
        } else {
            break;
        }
    }

    tokenizer.slice(start, tokenizer.position())
}

pub fn consume_hex<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<u32, ()> {
    let start = tokenizer.position();
    while !tokenizer.is_eof() && tokenizer.position() - start < 6 {
        let byte = tokenizer.next_byte_unchecked();
        match byte {
            b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
                tokenizer.consume(1);
            }
            b' ' => {
                tokenizer.consume(1);
                break;
            }
            _ => break,
        }
    }

    let hex = match u32::from_str_radix(tokenizer.slice(start, tokenizer.position()), 16) {
        Ok(hex) => hex,
        Err(err) => {
            return Err(());
        }
    };
    u32::from_str_radix(tokenizer.slice(start, tokenizer.position()), 16).map_err(|_| ())
}

pub fn consume_escape<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<char, ()> {
    tokenizer.consume(1);
    let start = tokenizer.position();

    while !tokenizer.is_eof() {
        match tokenizer.next_byte_unchecked() {
            b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
                let hex = consume_hex(tokenizer).map(|v| v)?;
                return if hex == 0 || hex > 0x10FFFF {
                    Ok('�')
                } else {
                    char::from_u32(hex).ok_or(())
                };
            }
            _ => {
                tokenizer.consume(1);
                return Ok(tokenizer.next_byte_unchecked() as char);
            }
        }
    }
    Err(())
}

pub fn consume_string<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<Token<'i>, ()> {
    let tt = tokenizer.next_byte_unchecked();
    tokenizer.consume(1);
    let start = tokenizer.position();
    let mut result = String::new();

    while !tokenizer.is_eof() {
        match tokenizer.next_byte_unchecked() {
            b'"' => {
                tokenizer.consume(1);
                if tt == b'"' {
                    break;
                }
            }
            b'\'' => {
                tokenizer.consume(1);
                if tt == b'\'' {
                    break;
                }
            }
            b'\n' | b'\r' | b'\x0C' => {
                tokenizer.reconsume(1);
                return Err(());
            }
            b'\\' => {
                if tokenizer.starts_with_escape() {
                    result.push(consume_escape(tokenizer)?);
                }
            }
            ch => {
                tokenizer.consume(1);
                result.push(ch as char);
            }
        }
    }
    Ok(Token::String(result))
}

pub fn consume_whitespace<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<Token<'i>, ()> {
    let start = tokenizer.position();
    while !tokenizer.is_eof() {
        match tokenizer.next_byte_unchecked() {
            b'\n' | b'\r' | b'\x0C' => {
                tokenizer.consume_newline();
            }
            b' ' | b'\t' => {
                tokenizer.consume(1);
            }
            _ => {
                break;
            }
        }
    }
    Ok(Token::Whitespace(
        tokenizer.slice(start, tokenizer.position()),
    ))
}

pub fn consume_comment<'i>(tokenizer: &mut Tokenizer<'i>) -> Result<&'i str, ()> {
    // Consume `/*` prefix
    tokenizer.consume(2);
    let start = tokenizer.position();

    while !tokenizer.is_eof() {
        match tokenizer.next_byte_unchecked() {
            b'*' => {
                let end = tokenizer.position();
                tokenizer.consume(1);
                if tokenizer.next_byte() == Some(b'/') {
                    tokenizer.consume(1);
                    return Ok(tokenizer.slice(start, end));
                }
            }
            // TODO: Continuation and Leading bytes?
            _ => {
                // Consume any other character
                tokenizer.consume(1);
            }
        }
    }

    Err(())
}
