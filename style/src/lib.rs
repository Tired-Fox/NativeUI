use std::{fs::File, path::Path, io::Read, collections::HashMap};

use cssparser::{Parser, ParserInput, RuleListParser};

pub mod styles;
pub mod color;
mod parser;
mod rules;

pub use parser::{RuleParser, StyleParser, Rule};
pub use rules::*;

#[derive(Debug)]
pub struct Stylesheet(HashMap<String, Vec<Style>>);

impl Stylesheet {
    pub fn parse(src: &str) -> Self {
        let mut input = ParserInput::new(src);
        let mut input = Parser::new(&mut input);
        let rules: Vec<Rule> = RuleListParser::new_for_stylesheet(&mut input, RuleParser {})
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|rule| rule.ok())
            .collect();

        let mut styles = HashMap::new();
        for rule in rules {
            styles.insert(rule.key, rule.styles);
        }

        Stylesheet(styles)
    }

    pub fn file(path: &str) -> Self{
        let path = Path::new(path);
        let mut file = match File::open(path) {
            Err(error) => panic!("Couldn't open file '{}': {}", path.display(), error),
            Ok(file) => file
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(error) => panic!("Couldn't read file '{}': {}", path.display(), error),
            Ok(_) => ()
        }

        Stylesheet::parse(s.as_str())
    }
}
