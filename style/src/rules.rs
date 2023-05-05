
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique
}

#[derive(Debug)]
pub enum Properties {
    FontStyle(FontStyle)
}
