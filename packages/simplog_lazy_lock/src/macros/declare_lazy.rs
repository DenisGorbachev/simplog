#[doc(hidden)]
#[macro_export]
macro_rules! __declare_lazy {
    ($name:ident, $type:ty, $initializer:expr) => {
        pub static $name: ::std::sync::LazyLock<$type> = ::std::sync::LazyLock::new(|| $initializer);
    };
}
