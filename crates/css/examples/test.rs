extern crate cypress_css;

use std::fmt::{Debug, Display};

use cypress_css::parser::stylesheet::StyleSheet;

fn main() {
    let src = r#"
    @import "other.css";

    div > p ~ a + input,
    #some-id[data-value~='foo' i]::before,
    div.some-class#of-pop:where(p > a, :pseudo-invalid) {
        color: red;
    }
    "#;
    //
    // let stylesheet = StyleSheet::source(src);
    // println!("{:?}", stylesheet);

    let stylesheet = StyleSheet::path("examples/styles.css");
    println!("{:?}", stylesheet);
    println!("{}", stylesheet);
}
