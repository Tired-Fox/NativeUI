extern crate cypress_css;

use std::fmt::{Debug, Display};

use cypress_css::parser::stylesheet::StyleSheet;

fn main() {
    let stylesheet = StyleSheet::path("examples/styles.css");
    for error in stylesheet.errors.iter() {
        eprintln!("{}", error);
    }
    println!("{}", stylesheet);
}
