pub mod Controls {
    #[macro_export]
    macro_rules! text {
        ($($text:tt)*) => {
            $crate::core::ChildType::Control(
                $crate::core::ControlType::Text($crate::control::Text::new(
                        $($text)*
                ))
            )
        };
    }

    pub use text;
}
