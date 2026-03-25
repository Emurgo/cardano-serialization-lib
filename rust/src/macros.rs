macro_rules! impl_num_from {
    ($wrapper:ident, $($source:ty),+ $(,)?) => {
        $(
            impl From<$source> for $wrapper {
                fn from(value: $source) -> Self {
                    Self(value.into())
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

macro_rules! impl_num_op {
    ($wrapper:ident, $inner:ty, $trait:ident, $method:ident, $op:tt) => {
        impl std::ops::$trait for $wrapper {
            type Output = $wrapper;

            fn $method(self, rhs: Self) -> Self::Output {
                Self(self.0 $op rhs.0)
            }
        }

        impl std::ops::$trait<$inner> for $wrapper {
            type Output = $wrapper;

            fn $method(self, rhs: $inner) -> Self::Output {
                Self(self.0 $op rhs)
            }
        }
    };
}

macro_rules! impl_num_ops {
    ($wrapper:ident, $inner:ty) => {
        impl std::iter::Sum for $wrapper {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self(iter.into_iter().map(|v| v.0).sum())
            }
        }

        impl_num_op!($wrapper, $inner, Add, add, +);
        impl_num_op!($wrapper, $inner, Sub, sub, -);
        impl_num_op!($wrapper, $inner, Mul, mul, *);
        impl_num_op!($wrapper, $inner, Div, div, /);
        impl_num_op!($wrapper, $inner, Rem, rem, %);

        impl num_traits::Zero for $wrapper {
            fn zero() -> Self {
                Self(num_traits::Zero::zero())
            }

            fn is_zero(&self) -> bool {
                num_traits::Zero::is_zero(&self.0)
            }
        }

        impl num_traits::One for $wrapper {
            fn one() -> Self {
                Self(num_traits::One::one())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    mod impl_num {
        #[derive(Debug, PartialEq)]
        struct TestNum(u64);
        impl_num_from!(TestNum, u32, u16, u8);
        impl_num_into!(TestNum, u128, u64);
        impl_num_ops!(TestNum, u64);

        #[quickcheck]
        fn from(inner: u32) {
            assert_eq!(TestNum::from(inner), TestNum(inner.into()));
        }

        #[quickcheck]
        fn into(inner: u64) {
            assert_eq!(u128::from(TestNum(inner)), inner as u128);
            assert_eq!(u128::from(&TestNum(inner)), inner as u128);
        }

        #[quickcheck]
        fn add(a: u32, b: u32) {
            let a = a as u64;
            let b = b as u64;
            assert_eq!(TestNum(a) + TestNum(b), TestNum(a + b));
            assert_eq!(TestNum(a) + b, TestNum(a + b));
        }

        #[quickcheck]
        fn sub(a: u64, b: u64) {
            if a >= b {
                assert_eq!(TestNum(a) - TestNum(b), TestNum(a - b));
                assert_eq!(TestNum(a) - b, TestNum(a - b));
            }
        }

        #[quickcheck]
        fn mul(a: u32, b: u32) {
            let a = a as u64;
            let b = b as u64;
            assert_eq!(TestNum(a) * TestNum(b), TestNum(a * b));
            assert_eq!(TestNum(a) * b, TestNum(a * b));
        }

        #[quickcheck]
        fn div(a: u64, b: u64) {
            if b != 0 {
                assert_eq!(TestNum(a) / TestNum(b), TestNum(a / b));
                assert_eq!(TestNum(a) / b, TestNum(a / b));
            }
        }

        #[quickcheck]
        fn rem(a: u64, b: u64) {
            if b != 0 {
                assert_eq!(TestNum(a) % TestNum(b), TestNum(a % b));
                assert_eq!(TestNum(a) % b, TestNum(a % b));
            }
        }

        #[quickcheck]
        fn sum(values: Vec<u32>) {
            let total: u64 = values.iter().cloned().map(u64::from).sum();
            let wrapped: TestNum = values.into_iter().map(|n| TestNum(n.into())).sum();
            assert_eq!(wrapped, TestNum(total));
        }

        #[test]
        fn zero() {
            use num_traits::Zero;
            assert_eq!(TestNum::zero(), TestNum(0));
            assert!(TestNum::zero().is_zero());
            assert!(!TestNum(1).is_zero());
        }

        #[test]
        fn one() {
            use num_traits::One;
            assert_eq!(TestNum::one(), TestNum(1));
        }
    }
}
