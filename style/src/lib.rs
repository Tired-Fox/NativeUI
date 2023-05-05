use std::{fs::File, path::Path, io::Read};

use cssparser::{Parser, ParserInput, RuleListParser};

pub mod color;
mod parser;
mod rules;

pub use parser::{RuleParser, StyleParser, Rule};
pub use rules::*;

pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    pub fn parse(src: &str) -> Self {
        let mut input = ParserInput::new(src);
        let mut input = Parser::new(&mut input);
        let rules = RuleListParser::new_for_stylesheet(&mut input, RuleParser {})
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|rule| rule.ok())
            .collect();

        Stylesheet { rules }
    }

    pub fn file(path: &str) -> Self{
        let path = Path::new(path);
        let mut file = match File::open(path) {
            Err(why) => panic!("Couldn't open file '{}': {}", path.display(), why),
            Ok(file) => file
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("Couldn't read file '{}': {}", path.display(), why),
            Ok(_) => ()
        }

        Stylesheet::parse(s.as_str())
    }
}
