use std::fmt::Display;

pub struct Color {
    pub default: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: f32,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        assert!(alpha < 1.0 && alpha >= 0.0);
        Color {
            default: format!("rgba({}, {}, {}, {})", red, green, blue, alpha),
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "Color {{ r: {}, g: {}, b: {}, a: {}}}",
                self.red, self.green, self.blue, self.alpha
            )
        )
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        if value.starts_with("rgb") || value.starts_with("rgba") {
            rgb_to_color(value)
        } else {
            hex(value)
        }
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        u32::from_be_bytes([0, value.blue, value.green, value.red])
    }
}

fn rgb_to_color(mut value: &str) -> Color {
    let default = String::from(value);
    if value.starts_with("rgb(") {
        value = value.strip_prefix("rgb(").unwrap();
    }

    if value.starts_with("rgba(") {
        value = value.strip_prefix("rgba(").unwrap();
    }

    value = value.strip_suffix(")").unwrap();
    let colors = value.split(",");
    let colors = colors.into_iter().map(|s| s.trim()).collect::<Vec<&str>>();
    match colors.len() {
        3 => Color {
            default,
            red: colors[0].parse::<u8>().unwrap(),
            green: colors[1].parse::<u8>().unwrap(),
            blue: colors[2].parse::<u8>().unwrap(),
            alpha: 1f32,
        },
        4 => Color {
            default,
            red: colors[0].parse::<u8>().unwrap(),
            green: colors[1].parse::<u8>().unwrap(),
            blue: colors[2].parse::<u8>().unwrap(),
            alpha: colors[3].parse::<f32>().unwrap(),
        },
        _ => panic!("Invalid rgb color syntax"),
    }
}

pub fn rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
    Color {
        default: format!("rgba({}, {}, {}, {})", red, green, blue, alpha),
        red,
        green,
        blue,
        alpha
    }
}

pub fn hex(mut code: &str) -> Color {
    if code.starts_with("#") {
        code = code.strip_prefix("#").unwrap();
    }

    let mut hex = code.to_owned();

    let (red, green, blue, alpha) = match hex.len() {
        3 => {
            hex = hex
                .chars()
                .flat_map(|c| std::iter::repeat(c).take(2))
                .collect::<String>();

            (
                u8::from_str_radix(&hex[..2], 16).unwrap(),
                u8::from_str_radix(&hex[2..4], 16).unwrap(),
                u8::from_str_radix(&hex[4..6], 16).unwrap(),
                1f32,
            )
        }
        4 => {
            hex = hex
                .chars()
                .flat_map(|c| std::iter::repeat(c).take(2))
                .collect::<String>();
            (
                u8::from_str_radix(&hex[..2], 16).unwrap(),
                u8::from_str_radix(&hex[2..4], 16).unwrap(),
                u8::from_str_radix(&hex[4..6], 16).unwrap(),

                (&hex[6..8]).parse::<f32>().unwrap(),
            )
        }
        6 => (
            u8::from_str_radix(&hex[..2], 16).unwrap(),
            u8::from_str_radix(&hex[2..4], 16).unwrap(),
            u8::from_str_radix(&hex[4..6], 16).unwrap(),
            1f32,
        ),
        8 => (
            u8::from_str_radix(&hex[..2], 16).unwrap(),
            u8::from_str_radix(&hex[2..4], 16).unwrap(),
            u8::from_str_radix(&hex[4..6], 16).unwrap(),
            (&hex[6..8]).parse::<f32>().unwrap(),
        ),
        _ => panic!("Invalid hex code syntax"),
    };

    return Color {
        default: format!("#{}", code),
        red: red.to_owned(),
        green: green.to_owned(),
        blue: blue.to_owned(),
        alpha: alpha.to_owned(),
    };
}
