use std::{collections::HashMap, fs::File, io::Read, path::Path};

use cssparser::{Parser, ParserInput, RuleListParser};

pub mod color;
use color::Color;
mod parser;
mod size;
use size::Size;
mod rules;

pub use parser::{Rule, RuleParser, StyleParser};
pub use rules::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub position: Position,
    pub width: Unit,
    pub height: Unit,

    pub padding: Size,
    pub margin: Size,
    pub inset: Size,
}

impl Default for Dimensions {
    fn default() -> Self {
        Dimensions {
            position: Position::default(),
            width: Unit::default(),
            height: Unit::default(),

            padding: Size::default(),
            margin: Size::default(),
            inset: Size::default(),
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
                        Style::Position(position) => dimensions.position = *position,

                        Style::Padding(size) => dimensions.padding = *size,
                        Style::PaddingInline(inline) => {
                            dimensions.padding.left = inline.clone();
                            dimensions.padding.right = *inline;
                        }
                        Style::PaddingBlock(block) => {
                            dimensions.padding.top = block.clone();
                            dimensions.padding.bottom = *block;
                        }
                        Style::PaddingLeft(left) => dimensions.padding.left = *left,
                        Style::PaddingTop(top) => dimensions.padding.top = *top,
                        Style::PaddingRight(right) => dimensions.padding.right = *right,
                        Style::PaddingBottom(bottom) => dimensions.padding.bottom = *bottom,

                        Style::Margin(size) => dimensions.margin = *size,
                        Style::MarginInline(inline) => {
                            dimensions.margin.left = inline.clone();
                            dimensions.margin.right = *inline;
                        }
                        Style::MarginBlock(block) => {
                            dimensions.margin.top = block.clone();
                            dimensions.margin.bottom = *block;
                        }
                        Style::MarginTop(top) => dimensions.margin.top = *top,
                        Style::MarginLeft(left) => dimensions.margin.left = *left,
                        Style::MarginRight(right) => dimensions.margin.right = *right,
                        Style::MarginBottom(bottom) => dimensions.margin.bottom = *bottom,

                        Style::Inset(inset) => dimensions.inset = *inset,
                        Style::InsetBlock(block) => {
                            dimensions.inset.left = block.clone();
                            dimensions.inset.left = *block;
                        },
                        Style::InsetInline(inline) => {
                            dimensions.inset.top = inline.clone();
                            dimensions.inset.bottom = *inline;
                        },
                        Style::Top(top) => dimensions.inset.top = *top,
                        Style::Left(left) => dimensions.inset.left = *left,
                        Style::Right(right) => dimensions.inset.right = *right,
                        Style::Bottom(bottom) => dimensions.inset.bottom = *bottom,
                    };
                }
            }
        }

        (dimensions, appearance)
    }
}
