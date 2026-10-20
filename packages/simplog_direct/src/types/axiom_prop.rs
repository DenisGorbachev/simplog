use crate::Source;

#[derive(Copy, Clone, Debug)]
pub struct AxiomProp {
    pub statement: &'static str,
    pub source: Option<Source>,
}
