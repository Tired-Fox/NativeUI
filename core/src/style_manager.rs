use std::sync::RwLock;

use once_cell::sync::Lazy;
use style::Stylesheet;

// TODO: Convert to theme manager. Automatically gets system defaults and uses what is set
#[derive(Debug)]
pub struct StyleManager(pub RwLock<Stylesheet>);

pub static STYLESHEET: Lazy<StyleManager> =
    Lazy::new(|| StyleManager(RwLock::new(Stylesheet::default())));

impl StyleManager {
    pub fn get(
        &self,
    ) -> std::sync::RwLockWriteGuard<Stylesheet> {
        self.0.write().unwrap()
    }
}
