use std::{collections::HashMap, fs::File, io::Read, path::Path};

use cssparser::{Parser, ParserInput, RuleListParser};

pub mod color;
use color::Color;
mod parser;
mod rules;

pub use parser::{Rule, RuleParser, StyleParser};
pub use rules::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub width: Unit,
    pub height: Unit,
}

impl Default for Dimensions {
    fn default() -> Self {
        Dimensions {
            width: Unit::default(),
            height: Unit::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Appearance {
    pub font_style: FontStyle,

    pub background_color: Color,
}

impl Default for Appearance {
    fn default() -> Self {
        Appearance {
            font_style: FontStyle::default(),

            background_color: Color::new(255, 255, 255, 1.),
        }
    }
}

#[derive(Debug, Default)]
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

    pub fn file(path: &str) -> Self {
        let path = Path::new(path);
        let mut file = match File::open(path) {
            Err(error) => panic!("Couldn't open file '{}': {}", path.display(), error),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(error) => panic!("Couldn't read file '{}': {}", path.display(), error),
            Ok(_) => (),
        }

        Stylesheet::parse(s.as_str())
    }

    pub fn get_styles(&self, rules: Vec<String>) -> (Dimensions, Appearance) {
        let mut dimensions = Dimensions::default();
        let mut appearance = Appearance::default();

        for rule in rules.iter() {
            if self.0.contains_key(rule) {
                for style in self.0.get(rule).unwrap().iter() {
                    match style {
                        Style::Width(width) => dimensions.width = width.clone(),
                        Style::Height(height) => dimensions.height = height.clone(),

                        Style::FontStyle(font_style) => appearance.font_style = font_style.clone(),

                        Style::BackgroundColor(color) => appearance.background_color = color.to_owned(),
                    };
                }
            }
        }

        (dimensions, appearance)
    }
}
