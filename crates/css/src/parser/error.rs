use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default)]
pub enum StyleParseError {
    NotImplemented,
    #[default]
    Unkown,
    UnkownSyntax,
    UnkownAtRule,
    UnkownPseudoClass,
    UnkownPseudoElement,
    UnkownProperty,
    EndOfStream,
    InvalidHexFormat,
    InvalidArgument,
    ExpectedZero,
    InvalidSelector(&'static str),
    ExpectedSelector,
    Expected(&'static str),
    RangeAllowedItems{min: usize, max: usize},
    ExpectedKeyword(&'static str),
    ExpectedKeywords(Vec<&'static str>),
    ExpectedPattern(Link),
    ExpectedFunction(&'static str),
    ExpectedFunctions(Vec<&'static str>),
    ExpectedAngle,
    ExpectedNumber,
    ExpectedAngleOrNumber,
    ExpectedInteger,
    ExpectedNumberOrPercent,
    ExpectedLengthOrPercent,
    ExpectedCombinator,
    ExpectedString,
    ExpectedIdentOrString,
    ExpectedEqualSign,
    DuplicateIDSelector,
    DuplicateElementSelector,
    ExpectedPercent,
    InvalidPseudoSelector,
    InvalidColor,
    InvalidNthFormat,
    InvalidColorKeyword,
    UnexpectedCombinator,
    ExpectedArguments,
}

pub struct Link {
    pub title: &'static str,
    pub url: &'static str
}
impl Link {
    pub fn new(title: &'static str, url: &'static str) -> Self {
        Self {
            title,
            url
        }
    }
}
impl Debug for Link {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", self.url, self.title)
    }
}

#[derive(Debug, Default)]
pub struct Error {
    pub kind: StyleParseError,
    pub line: u32,
    pub column: u32,
    pub src: String,
}

impl Error {
    pub fn set_src(&mut self, src: &str) {
       self.src = src.to_string();
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //╘ │─║╚⮥╚
        let line_num= format!("{}", self.line+1);
        let message = format!("[\x1b[1;31mERROR\x1b[22;39m:{}:{}]: {:?}", line_num, self.column, self.kind);
        let code = format!("{} │ {}", line_num, self.src);
        let pointer = format!("\x1b[31m{}  {}▴\x1b[39m", " ".repeat(line_num.len()), " ".repeat((self.column as usize)));
        write!(f, "{}\n{}\n{}\n", message, code, pointer)
    }
}

impl std::error::Error for Error {}