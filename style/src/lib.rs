use std::{collections::HashMap, fs::File, io::Read, path::Path};

use cssparser::{Parser, ParserInput, RuleListParser};

use color::Color;
mod parser;
mod size;
mod rules;

pub mod color;
pub use size::Size;
pub use parser::{Rule, RuleParser, StyleParser};
pub use rules::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub position: Position,
    pub min_width: Unit,
    pub width: Unit,
    pub max_width: Unit,
    pub min_height: Unit,
    pub height: Unit,
    pub max_height: Unit,

    pub padding: Size,
    pub margin: Size,
    pub inset: Size,
    pub overflow_x: Overflow,
    pub overflow_y: Overflow,
}

impl Default for Dimensions {
    fn default() -> Self {
        Dimensions {
            position: Position::default(),
            min_width: Unit::Default,
            width: Unit::Default,
            max_width: Unit::Default,
            min_height: Unit::Default,
            height: Unit::Default,
            max_height: Unit::Default,

            padding: Size::default(),
            margin: Size::default(),
            inset: Size::default(),
            overflow_x: Overflow::default(),
            overflow_y: Overflow::default(),
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

unsafe impl Send for Stylesheet {}
unsafe impl Sync for Stylesheet {}

impl Stylesheet {
    pub fn dup(&mut self, src: Stylesheet) {
        self.0 = src.0.clone();
    }

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
                        Style::MinWidth(min_width) => dimensions.min_width = *min_width,
                        Style::Width(width) => dimensions.width = *width,
                        Style::MaxWidth(max_width) => dimensions.max_width = *max_width,
                        Style::MinHeight(min_height) => dimensions.min_height = *min_height,
                        Style::Height(height) => dimensions.height = *height,
                        Style::MaxHeight(max_height) => dimensions.max_height = *max_height,

                        Style::FontStyle(font_style) => appearance.font_style = *font_style,

                        Style::BackgroundColor(color) => appearance.background_color = *color,
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
                        Style::Overflow(overflow) => {
                            dimensions.overflow_x = overflow.clone();
                            dimensions.overflow_y = *overflow
                        },
                        Style::OverflowX(overflow) => {
                            dimensions.overflow_x = *overflow;
                        },
                        Style::OverflowY(overflow) => {
                            dimensions.overflow_y = *overflow;
                        }
                    };
                }
            }
        }

        (dimensions, appearance)
    }
}
