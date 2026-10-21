use crate::Source;
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Clone, Debug)]
pub struct Definition {
    #[new(into)]
    pub term: String,
    #[new(into)]
    pub meaning: String,
    pub definitions: &'static [&'static LazyLock<Self>],
    #[new(into)]
    pub source: Option<Source>,
}

#[macro_export]
macro_rules! definition {
    ($name:ident, $term:expr, $meaning:expr, [$($definition:path),* $(,)?], $source:expr) => {
        $crate::lazy_lock!($name, $crate::Definition, $crate::Definition::new($term, $meaning, const { &[$(&$definition),*] }, $source));
    };
    ($name:ident, $term:expr, $meaning:expr, [$($definition:path),* $(,)?]) => {
        $crate::definition!($name, $term, $meaning, [$($definition),*], ::core::option::Option::None);
    };
    ($name:ident, $term:expr, $meaning:expr) => {
        $crate::definition!($name, $term, $meaning, []);
    };
}
