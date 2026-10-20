use crate::Prop;

#[derive(Copy, Clone, Debug)]
pub struct TheoremProp {
    pub content: &'static str,
    pub dependencies: &'static [&'static Prop],
}
