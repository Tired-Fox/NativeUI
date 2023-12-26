use std::fs;
use std::ops::Index;
use lightningcss::printer::PrinterOptions;

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, StyleSheet};

fn main() {
    let input = fs::read_to_string("examples/styles.css").unwrap();
    let mut stylesheet = StyleSheet::parse(input.as_str(), ParserOptions {
        filename: "styles.css".to_string(),
        ..Default::default()
    }).unwrap();

    let _ = stylesheet.minify(MinifyOptions::default()).unwrap();
    let res = stylesheet.to_css(PrinterOptions {minify: true, ..Default::default()}).unwrap();
    println!("{}", res.code);
}