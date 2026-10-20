use crate::Prop;
use derive_more::{AsRef, Deref};
use derive_new::new;
use std::sync::LazyLock;

#[derive(new, Deref, AsRef, Copy, Clone, Debug)]
pub struct Value<T> {
    #[deref]
    #[as_ref]
    pub inner: T,
    pub dependencies: &'static [&'static LazyLock<Prop>],
}
