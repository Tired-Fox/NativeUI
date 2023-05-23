use std::sync::RwLock;

use style::Stylesheet;
use once_cell::sync::Lazy;

// TODO: Convert to theme manager. Automatically gets system defaults and uses what is set
#[derive(Debug)]
pub struct StyleManager(pub RwLock<Stylesheet>);

pub static STYLESHEET: Lazy<StyleManager> = Lazy::new(|| {
    StyleManager(RwLock::new(Stylesheet::default()))
});
