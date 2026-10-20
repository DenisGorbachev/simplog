use crate::{AxiomProp, TheoremProp};
use derive_more::From;

#[derive(From, Clone, Debug)]
pub enum Prop {
    Axiom(AxiomProp),
    Theorem(TheoremProp),
}

#[macro_export]
macro_rules! axiom {
    ($name:ident, $content:expr, $source:expr) => {
        $crate::lazy_lock!($name, $crate::Prop, $crate::Prop::Axiom($crate::AxiomProp::new($content, $source)));
    };
    ($name:ident, $content:expr) => {
        $crate::axiom!($name, $content, ::core::option::Option::None);
    };
}

#[macro_export]
macro_rules! theorem {
    ($name:ident, $content:expr, [$($dependency:path),* $(,)?]) => {
        $crate::lazy_lock!($name, $crate::Prop, $crate::Prop::Theorem($crate::TheoremProp::new($content, const { &[$(&$dependency),*] })));
    };
}
