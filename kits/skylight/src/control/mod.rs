pub mod button;
pub mod text;

use crate::{core::Rect, window::Window};
use style::Stylesheet;

trait Control {
    /// Update the components rect and redraw when finished
    fn update(&mut self, window: Window, previous: impl Control);

    /// Get styles
    fn stylesheet(&self) -> Stylesheet;

    /// Get components rect
    fn rect(&self) -> Rect;
}
