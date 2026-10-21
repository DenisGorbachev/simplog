use crate::{Definition, Prop};
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Clone, Debug)]
pub struct Question {
    #[new(into)]
    pub statement: String,
    pub definitions: &'static [&'static LazyLock<Definition>],
    pub context: &'static [&'static LazyLock<Prop>],
}

#[macro_export]
macro_rules! question {
    ($name:ident, $statement:expr, [$($definition:path),* $(,)?], [$($context:path),* $(,)?]) => {
        $crate::lazy_lock!($name, $crate::Question, $crate::Question::new($statement, const { &[$(&$definition),*] }, const { &[$(&$context),*] }));
    };
    ($name:ident, $statement:expr, [$($definition:path),* $(,)?]) => {
        $crate::question!($name, $statement, [$($definition),*], []);
    };
    ($name:ident, $statement:expr) => {
        $crate::question!($name, $statement, []);
    };
}
