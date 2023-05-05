use super::color::Color;
use std::fmt::Display;

pub enum Pseudo {
    Class(String),
    Element(String),
    None,
}

impl Display for Pseudo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class(class) => write!(f, ":{}", class),
            Self::Element(element) => write!(f, "::{}", element),
            Self::None => write!(f, ""),
        }
    }
}

pub struct Element {
    pub tag: Option<String>,
    pub classes: Vec<String>,
    pub id: Option<String>,
    pub attributes: Vec<Attribute>,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tag = "";
        if let Some(t) = &self.tag {
            tag = t.as_str();
        }

        let mut id = String::new();
        if let Some(i) = &self.id {
            id = format!("#{}", i.to_owned());
        }

        let mut classes = self
            .classes
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(".");

        if self.classes.len() > 0 {
            classes = format!(".{}", classes);
        }

        let attributes = self
            .attributes
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{}{}{}{}", tag, id, classes, attributes)
    }
}

pub struct Attribute {
    pub name: String,
    pub compare: AttributeCompare,
    pub value: Option<String>,
    pub case_sensitive: bool,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value = "";
        if let Some(v) = &self.value {
            value = v.as_str();
        }

        write!(f, "[{}{}{}]", self.name, self.compare, value)
    }
}

pub enum AttributeCompare {
    Exists,                // ``
    Equal,                 // `=`
    StartsWith,            // `^=`
    EndsWith,              // `$=`
    Contains,              // `*=`
    EqualOrStartsWithDash, // `|=`
    InList,                // `~=`
}

impl Display for AttributeCompare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Exists => "",
            Self::Equal => "=",
            Self::StartsWith => "^=",
            Self::EndsWith => "$=",
            Self::Contains => "*=",
            Self::EqualOrStartsWithDash => "|=",
            Self::InList => "~=",
        };
        write!(f, "{}", symbol)
    }
}

pub enum Select {
    Element(Element, Pseudo),

    Group(Vec<Select>),
    Decendant(Box<Select>, Box<Select>),
    Child(Box<Select>, Box<Select>),
    GeneralSibling(Box<Select>, Box<Select>),
    AdjacentSibling(Box<Select>, Box<Select>),
}

impl Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(element, pseudo) => write!(f, "{}{}", element, pseudo),
            Self::Group(selectors) => write!(
                f,
                "{}",
                selectors
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Decendant(s1, s2) => write!(f, "{} {}", *s1, *s2),
            Self::Child(s1, s2) => write!(f, "{} > {}", *s1, *s2),
            Self::GeneralSibling(s1, s2) => write!(f, "{} ~ {}", *s1, *s2),
            Self::AdjacentSibling(s1, s2) => write!(f, "{} + {}", *s1, *s2),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BorderStyle {
    Solid,
    Dotted,
    Dashed,
    None,
}

/// Background Style. Right now only applies to windows hatch brush
#[derive(Clone, Debug)]
pub enum BS {
    DCROSS,
    CROSS,
    VERTICAL,
    HORIZONTAL,
    TANGENT,
    DIAGNOL,
    SOLID,
}

impl From<&str> for BS {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "dcross" => BS::DCROSS,
            "cross" => BS::CROSS,
            "vertical" => BS::VERTICAL,
            "horizontal" => BS::HORIZONTAL,
            "tangent" => BS::TANGENT,
            "diagnol" => BS::DIAGNOL,
            "solid" => BS::SOLID,
            _ => BS::SOLID,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Size {
    PX(i32),
    Percent(f32),
}

#[derive(Clone, Debug)]
pub enum Prop {
    Size(Size),
    Color(Color),
    Border(i32, Option<BorderStyle>, Option<Color>),
    BorderStyle(BorderStyle),
    BackgroundStyle(BS),
    Background(Color, Option<BS>),
    Padding(Size, Size, Size, Size),
}

impl Prop {
    pub fn get_size(size: Size, total: i32) -> i32 {
        match size {
            Size::PX(px) => px,
            Size::Percent(percent) => (total as f32 * percent) as i32,
        }
    }
}

impl From<i32> for Prop {
    fn from(value: i32) -> Self {
        Prop::Size(Size::PX(value))
    }
}

impl From<f32> for Prop {
    fn from(value: f32) -> Self {
        Prop::Size(Size::Percent(value))
    }
}

impl From<&str> for Prop {
    fn from(value: &str) -> Self {
        if value.ends_with("px") {
            Prop::Size(Size::PX(
                value
                    .strip_suffix("px")
                    .unwrap()
                    .trim()
                    .parse::<i32>()
                    .unwrap(),
            ))
        } else if value.ends_with("%") {
            Prop::Size(Size::Percent(
                value
                    .strip_suffix("%")
                    .unwrap()
                    .trim()
                    .parse::<f32>()
                    .unwrap(),
            ))
        } else if vec![
            "dcross",
            "cross",
            "vertical",
            "horizontal",
            "tangent",
            "diagnol",
            "solid",
        ]
        .contains(&value.to_lowercase().as_str())
        {
            Prop::BackgroundStyle(BS::from(value))
        } else {
            Prop::Color(value.into())
        }
    }
}

impl Display for Prop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prop::Size(size) => {
                match size {
                    Size::PX(px) => write!(f, "{}px", px),
                    Size::Percent(percent) => write!(f, "{}%", percent)
                }
            } ,
            Prop::Color(color) => write!(f, "{}", color.default),
            _ => write!(f, "{}", "Unkown"),
        }
    }
}

pub struct Property {
    pub name: &'static str,
    pub value: Prop,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {};", self.name, self.value)
    }
}

pub struct Style {
    pub selector: Select,
    pub properties: Vec<Property>,
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let styles = self
            .properties
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n  ");

        write!(f, "{}\n{{\n  {}\n}}", self.selector, styles)
    }
}

pub struct Stylesheet {
    pub styles: Vec<Style>,
}

impl Display for Stylesheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blocks = self
            .styles
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", blocks)
    }
}
