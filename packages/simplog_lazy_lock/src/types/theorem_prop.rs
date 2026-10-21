use crate::{Definition, Prop};
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Clone, Debug)]
pub struct TheoremProp {
    #[new(into)]
    pub statement: String,
    pub dependencies: &'static [&'static LazyLock<Prop>],
    #[new(into)]
    pub inference: String,
    pub definitions: &'static [&'static LazyLock<Definition>],
}
