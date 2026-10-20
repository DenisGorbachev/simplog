use crate::{AxiomProp, TheoremProp};
use derive_more::From;

#[derive(From, Copy, Clone, Debug)]
pub enum Prop {
    Axiom(AxiomProp),
    Theorem(TheoremProp),
}

#[macro_export]
macro_rules! axiom {
    ($name:ident, $content:expr, $source:expr) => {
        pub static $name: $crate::Prop = $crate::Prop::Axiom($crate::AxiomProp::new($content, $source));
    };
    ($name:ident, $content:expr) => {
        pub static $name: $crate::Prop = $crate::Prop::Axiom($crate::AxiomProp::new($content, ::core::option::Option::None));
    };
}

#[macro_export]
macro_rules! theorem {
    ($name:ident, $content:expr, [$($dependency:path),* $(,)?] $(,)?) => {
        pub static $name: $crate::Prop = $crate::Prop::Theorem($crate::TheoremProp::new($content, &[$(&$dependency),*]));
    };
}
