use std::fmt::Display;

pub trait Unit {}

#[derive(Debug)]
pub enum Length {
    Percent(f32),
    Cm(f32),
    Mm(f32),
    Q(f32),
    In(f32),
    Pc(f32),
    Pt(f32),
    Px(f32),
}
impl Unit for Length {}

#[derive(Debug)]
pub enum Angle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
    Turn(f32),
}
impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Angle::Deg(value) => format!("{}deg", value),
            Angle::Grad(value) => format!("{}grad", value),
            Angle::Rad(value) => format!("{}rad", value),
            Angle::Turn(value) => format!("{}turn", value),
        })
    }
}
impl Unit for Angle {}

#[derive(Debug)]
pub enum Relative {
    Em(f32),
    Ex(f32),
    Ch(f32),
    Rem(f32),
    Lh(f32),
    Rlh(f32),
    Vw(f32),
    Vh(f32),
    VMin(f32),
    VMax(f32),
    Vb(f32),
    Vi(f32),
    Svw(f32),
    Svh(f32),
    Lvw(f32),
    Lvh(f32),
    Dvw(f32),
    Dvh(f32),
}
impl Unit for Relative {}
