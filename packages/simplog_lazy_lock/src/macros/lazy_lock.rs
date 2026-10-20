#[doc(hidden)]
#[macro_export]
macro_rules! lazy_lock {
    ($name:ident, $type:ty, $initializer:expr) => {
        pub static $name: ::std::sync::LazyLock<$type> = ::std::sync::LazyLock::new(|| $initializer);
    };
}
