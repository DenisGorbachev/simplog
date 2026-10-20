use crate::{Source, impl_const_new};

#[derive(Copy, Clone, Debug)]
pub struct AxiomProp {
    pub content: &'static str,
    pub source: Option<Source>,
}

impl_const_new!(AxiomProp, content: &'static str, source: Option<Source>);

#[macro_export]
macro_rules! axiom_prop {
    ($name:ident, $content:expr, $source:expr) => {
        pub static $name: $crate::AxiomProp = $crate::AxiomProp::new($content, $source);
    };
    ($name:ident, $content:expr) => {
        pub static $name: $crate::AxiomProp = $crate::AxiomProp::new($content, ::core::option::Option::None);
    };
}
