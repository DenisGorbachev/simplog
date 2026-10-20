use crate::{AxiomProp, TheoremProp};
use derive_more::From;

#[derive(From, Clone, Debug)]
pub enum Prop {
    Axiom(AxiomProp),
    Theorem(TheoremProp),
}

#[macro_export]
macro_rules! axiom {
    ($name:ident, $statement:expr, $source:expr) => {
        $crate::lazy_lock!($name, $crate::Prop, $crate::Prop::Axiom($crate::AxiomProp::new($statement, $source)));
    };
    ($name:ident, $statement:expr) => {
        $crate::axiom!($name, $statement, ::core::option::Option::None);
    };
}

#[macro_export]
macro_rules! theorem {
    ($name:ident, $statement:expr, [$($dependency:path),* $(,)?]) => {
        $crate::lazy_lock!($name, $crate::Prop, $crate::Prop::Theorem($crate::TheoremProp::new($statement, const { &[$(&$dependency),*] })));
    };
}
