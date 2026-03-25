macro_rules! impl_num_from {
    ($wrapper:ident, $($source:ty),+ $(,)?) => {
        $(
            impl From<$source> for $wrapper {
                fn from(x: $source) -> Self {
                    Self(x.into())
                }
            }
        )+
    };
}

macro_rules! impl_num_into {
    ($wrapper:ident, $($target:ty),+ $(,)?) => {
        $(
        impl From<$wrapper> for $target {
            fn from(value: $wrapper) -> Self {
                value.0.into()
            }
        }

        impl From<&$wrapper> for $target {
            fn from(value: &$wrapper) -> Self {
                value.0.into()
            }
        }
        )+
    };
}
