use crate::Source;

#[derive(Clone, Debug)]
pub struct AxiomProp {
    pub content: String,
    pub source: Option<Source>,
}
