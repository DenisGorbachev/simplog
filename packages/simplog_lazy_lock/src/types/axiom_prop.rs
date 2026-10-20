use crate::Source;
use derive_new::new;

#[derive(new, Clone, Debug)]
pub struct AxiomProp {
    #[new(into)]
    pub content: String,
    #[new(into)]
    pub source: Option<Source>,
}
