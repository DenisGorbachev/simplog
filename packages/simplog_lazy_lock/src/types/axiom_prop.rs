use crate::{AxiomKind, Definition, Source};
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Clone, Debug)]
pub struct AxiomProp {
    #[new(into)]
    pub statement: String,
    #[new(into)]
    pub source: Option<Source>,
    pub kind: AxiomKind,
    pub definitions: &'static [&'static LazyLock<Definition>],
}
