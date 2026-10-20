// derive-new 0.7 cannot generate the const constructors required by declaration macros.
macro_rules! impl_const_new {
    ($type:ident, $($field:ident: $field_type:ty),* $(,)?) => {
        impl $type {
            pub const fn new($($field: $field_type),*) -> Self {
                Self { $($field),* }
            }
        }
    };
}

pub(crate) use impl_const_new;
