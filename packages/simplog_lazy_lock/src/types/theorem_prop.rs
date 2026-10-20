use crate::Prop;
use std::sync::LazyLock;

#[derive(Clone, Debug)]
pub struct TheoremProp {
    pub content: String,
    pub dependencies: &'static [&'static LazyLock<Prop>],
}
