extern crate cypress_css;

use cypress_css::parser::tokenizer::{Tokenizer};

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
    let mut tokenizer = Tokenizer::new(src);

    while !tokenizer.is_eof() {
        match tokenizer.next() {
            Err(err) => {
                eprintln!("[cypress]: {:?}", err);
                break;
            },
            Ok(token) => println!("{:?}", token),
        }
    }
}
