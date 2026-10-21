use crate::{Definition, Source};
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Clone, Debug)]
pub struct AxiomProp {
    #[new(into)]
    pub statement: String,
    #[new(into)]
    pub source: Option<Source>,
    pub definitions: &'static [&'static LazyLock<Definition>],
}
