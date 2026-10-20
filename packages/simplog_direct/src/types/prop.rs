use crate::{AxiomProp, TheoremProp};
use derive_more::From;

#[derive(From, Copy, Clone, Debug)]
pub enum Prop {
    Axiom(AxiomProp),
    Theorem(TheoremProp),
}

#[macro_export]
macro_rules! axiom {
    ($name:ident, $statement:expr, $source:expr) => {
        pub static $name: $crate::Prop = $crate::Prop::Axiom($crate::AxiomProp {
            statement: $statement,
            source: $source,
        });
    };
    ($name:ident, $statement:expr) => {
        $crate::axiom!($name, $statement, ::core::option::Option::None);
    };
}

#[macro_export]
macro_rules! theorem {
    ($name:ident, $statement:expr, [$($dependency:path),* $(,)?]) => {
        pub static $name: $crate::Prop = $crate::Prop::Theorem($crate::TheoremProp { statement: $statement, dependencies: &[$(&$dependency),*] });
    };
}
