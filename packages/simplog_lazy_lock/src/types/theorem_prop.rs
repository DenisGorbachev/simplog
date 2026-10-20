use crate::Prop;
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Clone, Debug)]
pub struct TheoremProp {
    #[new(into)]
    pub content: String,
    pub dependencies: &'static [&'static LazyLock<Prop>],
}

#[macro_export]
macro_rules! theorem_prop {
    ($name:ident, $content:expr, [$($dependency:path),* $(,)?] $(,)?) => {
        $crate::__declare_lazy!($name, $crate::TheoremProp, $crate::TheoremProp::new($content, const { &[$(&$dependency),*] }));
    };
}
