use cssparser::CowRcStr;

#[derive(Debug)]
pub enum Decleration {
    Simple(String, String)
}

#[derive(Debug)]
pub struct Block(Vec<Decleration>);
