use cssparser::CowRcStr;

#[derive(Debug)]
pub enum Decleration<'i> {
    Simple(CowRcStr<'i>, CowRcStr<'i>)
}