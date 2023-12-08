extern crate cypress_css;

use cypress_css::parser::tokenizer::{Token, Tokenizer};

fn main() {
    let src = r#"
    /* This is a comment */
    "String here \00BC"
    +123.3e-16em
    +123.3e-16%
    +123.3e-16
    -123.3e-16em
    -123.3e-16%
    -123.3e-16
    -function("Some string param")
    -ident
    --ident
    .3e-16em
    .4e-16%
    .5e-16
    <!-- "String" -->
    .
    @
    @import
    url("")
    url()
    min(3, 4)

    #some-id > div.some-class {
        color: red;
    }
    "#;
    // TODO: Tests
    let mut tokenizer = Tokenizer::new(src);

    loop {
        match tokenizer.next() {
            Err(_) => {
                println!("ERROR");
                break;
            },
            Ok(token) => {
                if let Token::String(value) = token {
                    println!("{}", value);
                } else {
                    println!("{:?}", token);
                }
            }
        }
    }
}
