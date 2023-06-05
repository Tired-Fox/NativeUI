use crate::core::error::Error;
use native_core::layout::{Child, Layout};
use windows::Win32::Foundation::{HMODULE, HWND};

pub mod component {
    use crate::{core::error::Error, ui::component::Text};
    use native_core::layout::Child;
    use std::{cell::RefCell, sync::Arc};
    use windows::Win32::Foundation::{HMODULE, HWND};

    pub fn build_text(
        text: &str,
        id: Option<&str>,
        classes: Vec<&str>,
    ) -> Child<(HWND, HMODULE), Error> {
        let mut text =
            Text::builder(text).classes(classes.iter().map(|c| format!(".{}", c)).collect());

        if let Some(id) = id {
            text = text.id(format!("#{}", id).as_str());
        }

        Child::Component(Arc::new(RefCell::new(text.build())))
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
            $crate::prelude::component::build_text($text, None, Vec::new())
        };
        ($text: literal, $id: literal) => {
            $crate::prelude::component::build_text($text, Some($id), Vec::new())
        };
        ($text: literal, [$($class: literal),*]) => {
            $crate::prelude::component::build_text(
                $text,
                None,
                vec![$($class,)*]
            )
        };
        ($text: literal, $id: literal, [$($class: literal),*]) => {
            $crate::prelude::component::build_text(
                $text,
                Some($id),
                vec![$($class,)*]
            )
        };
    }
    pub use text;
}

pub fn build_layout(
    children: Vec<Child<(HWND, HMODULE), Error>>,
) -> Layout<(HWND, HMODULE), Error> {
    Layout::from(children)
}

#[macro_export]
macro_rules! layout {
    [$($child: expr),*] => {
       $crate::prelude::build_layout(vec![$($child,)*])
    };
}

pub use layout;
