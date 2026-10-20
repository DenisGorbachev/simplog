use crate::{Prop, impl_const_new};

#[derive(Copy, Clone, Debug)]
pub struct TheoremProp {
    pub content: &'static str,
    pub dependencies: &'static [&'static Prop],
}

impl_const_new!(TheoremProp, content: &'static str, dependencies: &'static [&'static Prop]);

#[macro_export]
macro_rules! theorem_prop {
    ($name:ident, $content:expr, [$($dependency:path),* $(,)?] $(,)?) => {
        pub static $name: $crate::TheoremProp = $crate::TheoremProp::new($content, &[$(&$dependency),*]);
    };
}
