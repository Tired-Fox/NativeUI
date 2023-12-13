extern crate cypress_css;

use std::fs;

use cypress_css::parser::stylesheet::StyleSheet;

fn main() {
    let src = r#"
    #some-id[data-value~='foo' i] > div.some-class#of-pop:pseudo-woodo(3) {
        color: red;
    }
    "#;

    let stylesheet = StyleSheet::source(src);
    println!("{:?}", stylesheet);

    let stylesheet = StyleSheet::path("examples/styles.css");
    println!("{:?}", stylesheet);
}
