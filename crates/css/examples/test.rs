extern crate cypress_css;

use cypress_css::parser::tokenizer::{Token, Tokenizer};

fn main() {
    let src = r#"
    /* This is a comment */
    "String here \00BC"
    "#;
    let mut tokenizer = Tokenizer::new(src);

    while let Ok(token) = tokenizer.next() {
        if let Token::String(value) = token {
            println!("{}", value);
        } else {
            println!("{:?}", token);
        }
    }
}
