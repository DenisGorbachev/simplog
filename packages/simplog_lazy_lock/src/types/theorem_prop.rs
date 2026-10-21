use crate::{Definition, Prop};
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Clone, Debug)]
pub struct TheoremProp {
    #[new(into)]
    pub statement: String,
    pub dependencies: &'static [&'static LazyLock<Prop>],
    pub definitions: &'static [&'static LazyLock<Definition>],
}
