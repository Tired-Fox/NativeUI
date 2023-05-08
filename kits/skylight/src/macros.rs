pub mod controls {
    use crate::control::{Control, Text};
    use std::{cell::RefCell, rc::Rc};

    pub fn build_text_control(text: &str, classes: Vec<&'static str>) -> Rc<RefCell<dyn Control>> {
        let child = Rc::new(RefCell::new(Text::new(text)));
        child.borrow_mut().classes(classes);
        child
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

    pub use text;
}
