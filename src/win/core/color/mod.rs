mod brush;
pub use brush::Brush;

pub fn hex(value: &str) -> u32 {
    let mut hex = value.to_owned();
    if hex.starts_with("#") {
        hex = hex.strip_prefix("#").unwrap().to_owned();
    }

    if hex.len() < 3 || hex.len() > 6 {
        panic!("Invalid hex value; expected length of 3 or 6")
    }

    if hex.len() == 3 {
        hex = hex
            .chars()
            .flat_map(|c| std::iter::repeat(c).take(2))
            .collect::<String>();
    }

    
    let r = &hex[..2];
    let g = &hex[2..4];
    let b = &hex[4..6];

    let mut hex = String::new();
    hex.push_str(b);
    hex.push_str(g);
    hex.push_str(r);

    u32::from_str_radix(hex.as_str(), 16).unwrap()
}

pub fn rgb(red: u8, green: u8, blue: u8) -> u32 {
    u32::from_be_bytes([0, blue, green, red])
}
