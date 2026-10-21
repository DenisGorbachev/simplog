#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AxiomKind {
    Unspecified,
    Fact,
    Assumption,
    Requirement,
}
