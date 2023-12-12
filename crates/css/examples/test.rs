extern crate cypress_css;

use std::fs;

use cypress_css::parser::stylesheet::StyleSheet;

fn main() {
    let src = r#"
    #some-id > div.some-class {
        color: red;
    }
    "#;

    let file = fs::read_to_string("examples/styles.css").unwrap();
    let stylesheet = StyleSheet::source(src);
    println!("{:?}", stylesheet);

    let stylesheet = StyleSheet::path("examples/styles.css");
    println!("{:?}", stylesheet);
}
