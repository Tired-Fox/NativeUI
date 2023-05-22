use style::Stylesheet;
use once_cell::sync::Lazy;

// TODO: Convert to theme manager. Automatically gets system defaults and uses what is set
#[derive(Debug)]
pub struct StyleManager(Stylesheet);

impl StyleManager {
    pub fn set_styles(&mut self, styles: Stylesheet) {
        self.0 = styles;
    }
}

pub static STYLESHEET: Lazy<StyleManager> = Lazy::new(|| {
    StyleManager(Stylesheet::default())
});