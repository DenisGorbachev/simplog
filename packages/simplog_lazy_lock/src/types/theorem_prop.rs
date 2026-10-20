use crate::Prop;
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Clone, Debug)]
pub struct TheoremProp {
    #[new(into)]
    pub content: String,
    pub dependencies: &'static [&'static LazyLock<Prop>],
}
