use crate::Source;
use derive_new::new;

#[derive(new, Clone, Debug)]
pub struct AxiomProp {
    #[new(into)]
    pub content: String,
    pub source: Option<Source>,
}

#[macro_export]
macro_rules! axiom_prop {
    ($name:ident, $content:expr, $source:expr) => {
        $crate::__declare_lazy!($name, $crate::AxiomProp, $crate::AxiomProp::new($content, $source));
    };
    ($name:ident, $content:expr) => {
        $crate::axiom_prop!($name, $content, ::core::option::Option::None);
    };
}
