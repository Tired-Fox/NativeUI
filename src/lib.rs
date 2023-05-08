use proc_macro_hack::proc_macro_hack;
pub use style;

#[cfg(feature="windows")]
pub use skylight;


#[proc_macro_hack]
pub use macros::styles;
