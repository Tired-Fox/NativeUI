
pub mod Controls {
    use std::{rc::Rc, cell::RefCell};
    use crate::control::{Control, Text};

    pub fn build_text_control(text: &str) -> Rc<RefCell<dyn Control>> {
        Rc::new(RefCell::new(Text::new(text)))
    }

    #[macro_export]
    macro_rules! text {
        ($($text:tt)*) => {
            $crate::core::ChildType::Control(
                $crate::macros::Controls::build_text_control($($text)*)
            )
        };
    }

    pub use text;

}
