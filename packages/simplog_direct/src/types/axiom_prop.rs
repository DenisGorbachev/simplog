use crate::Source;

#[derive(Copy, Clone, Debug)]
pub struct AxiomProp {
    pub content: &'static str,
    pub source: Option<Source>,
}
