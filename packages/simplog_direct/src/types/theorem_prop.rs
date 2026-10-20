use crate::Prop;

#[derive(Copy, Clone, Debug)]
pub struct TheoremProp {
    pub statement: &'static str,
    pub dependencies: &'static [&'static Prop],
}
