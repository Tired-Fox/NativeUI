pub mod controls {
    use crate::control::{Control, Text};
    use std::{cell::RefCell, rc::Rc};

    pub fn build_text_control(
        text: &str,
        styles: Vec<(&str, Prop)>,
    ) -> Rc<RefCell<dyn Control>> {
        let child = Rc::new(RefCell::new(Text::new(text)));
        child.borrow_mut().style(styles);
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
        ($text: literal $(, $style: literal : $value: literal)*) => {
            $crate::core::ChildType::Control(
                $crate::macros::controls::build_text_control(
                    $text,
                    vec![$(($style, $crate::style::Prop::from($value)),)*]
                )
            )
        };
    }

    use style::Prop;
    pub use text;
}
