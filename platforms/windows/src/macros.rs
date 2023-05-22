pub mod controls {
    use crate::{
        control::{Control, ScrollBar, Text},
        core::constants::SBS,
    };
    use std::{cell::RefCell, rc::Rc};

    pub fn build_text_control(text: &str, classes: Vec<&'static str>) -> Rc<RefCell<dyn Control>> {
        let child = Rc::new(RefCell::new(Text::new(text)));
        child.borrow_mut().classes(classes);
        child
    }

    pub fn build_scrollbar_control(size: i32, direction: &str) -> ScrollBar {
        match direction {
            "h" => ScrollBar::new(size, SBS::HORZ),
            "v" => ScrollBar::new(size, SBS::VERT),
            _ => ScrollBar::default(),
        }
    }

    /// Creates a text Control.
    ///
    /// # Args
    /// The first argument is the text to display. The remaining
    /// arguments are styles formatted `"style": Prop::<type>("value")` and
    /// can be unlimited.
    #[macro_export]
    macro_rules! text {
        ($text: literal) => {
            $crate::core::ChildType::Control(
                $crate::macros::controls::build_text_control($text, Vec::new())
            )
        };
        ($text: literal $(, $class: literal)*) => {
            $crate::core::ChildType::Control(
                $crate::macros::controls::build_text_control(
                    $text,
                    vec![$($class,)*]
                )
            )
        };
    }

    /// Creates a scrollbar Control.
    ///
    /// # Args
    /// The first argument is the size of the scrollbar. The
    /// second argument is the direction of the scrollbar.
    #[macro_export]
    macro_rules! scrollbar {
        ($size: literal, $direction: literal) => {
            $crate::macros::controls::build_scrollbar_control($size, $direction)
        };
    }

    pub use scrollbar;
    pub use text;
}
