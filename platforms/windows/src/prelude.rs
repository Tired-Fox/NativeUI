use native_core::{Child, Layout};
use windows::Win32::Foundation::{HWND, HMODULE};

pub mod component {
    use crate::{ui::component::{Text, ScrollBar}, core::constants::SBS};
    use native_core::Child;
    use windows::Win32::Foundation::{HWND, HMODULE};
    use std::{cell::RefCell, sync::Arc};

    pub fn build_text(text: &str, id: Option<&str>, classes: Vec<&str>) -> Child<(HWND, HMODULE)> {
        let mut text = Text::builder(text).classes(classes.iter().map(|c| format!(".{}", c)).collect());
        
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

    pub fn build_scrollbar_control(size: i32, direction: &str) -> ScrollBar {
        match direction {
            "h" => ScrollBar::new(size, SBS::HORZ),
            "v" => ScrollBar::new(size, SBS::VERT),
            _ => ScrollBar::default(),
        }
    }

    /// Creates a scrollbar Control.
    ///
    /// # Args
    /// The first argument is the size of the scrollbar. The
    /// second argument is the direction of the scrollbar.
    #[macro_export]
    macro_rules! scrollbar {
        ($size: literal, $direction: literal) => {
            $crate::prelude::component::build_scrollbar_control($size, $direction)
        };
    }

    pub use scrollbar;
}

pub fn build_layout(children: Vec<Child<(HWND, HMODULE)>>) -> Layout<(HWND, HMODULE)> {
    Layout::from(children)
}

#[macro_export]
macro_rules! layout {
    [$($child: expr),*] => {
       $crate::prelude::build_layout(vec![$($child,)*]) 
    };
}

pub use layout;
