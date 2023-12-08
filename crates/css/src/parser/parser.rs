use crate::parser::tokenizer::{Token, Tokenizer, TokenizerErrorKind};

pub struct Parser<'i> {
    tokenizer: Tokenizer<'i>,
    current: Token<'i>,
    next: Result<Token<'i>, TokenizerErrorKind>,
    reconsume: bool
}

impl<'i> Parser<'i> {
    pub fn new(input: &'i str) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        Self {
            next: tokenizer.next(),
            tokenizer,
            current: Token::Whitespace(""),
            reconsume: false
        }
    }

    #[inline]
    pub fn current(&self) -> &Token<'i> {
        &self.current
    }

    #[inline]
    pub fn next(&mut self) -> Result<&Token<'i>, &TokenizerErrorKind> {
        if self.reconsume {
            Ok(&self.current)
        } else {
            if let Ok(token) = &self.next {
                self.current = token.clone();
            }
            self.next.as_ref()
        }
    }

    #[inline]
    pub fn peek(&self) -> Result<&Token<'i>, &TokenizerErrorKind> {
        self.next.as_ref()
    }
}