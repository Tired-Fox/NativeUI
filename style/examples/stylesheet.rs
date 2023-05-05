extern crate style;
use style::Stylesheet;

fn main() {
    let stylesheet = Stylesheet::parse("div {
    font-style: normal;
}");
    
    println!("{:?}", stylesheet.rules);

    let stylesheet = Stylesheet::file("test.css");
    println!("{:?}", stylesheet.rules);
}
