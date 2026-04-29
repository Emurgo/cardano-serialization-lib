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

        impl num_traits::CheckedAdd for $wrapper {
            fn checked_add(&self, v: &Self) -> Option<Self> {
                num_traits::CheckedAdd::checked_add(&self.0, &v.0).map(Self)
            }
        }

        impl num_traits::CheckedSub for $wrapper {
            fn checked_sub(&self, v: &Self) -> Option<Self> {
                num_traits::CheckedSub::checked_sub(&self.0, &v.0).map(Self)
            }
        }

        impl num_traits::CheckedMul for $wrapper {
            fn checked_mul(&self, v: &Self) -> Option<Self> {
                num_traits::CheckedMul::checked_mul(&self.0, &v.0).map(Self)
            }
        }

        impl num_traits::CheckedDiv for $wrapper {
            fn checked_div(&self, v: &Self) -> Option<Self> {
                num_traits::CheckedDiv::checked_div(&self.0, &v.0).map(Self)
            }
        }
    };
    (@saturating $wrapper:ident, $inner:ty) => {
        impl num_traits::SaturatingAdd for $wrapper {
            fn saturating_add(&self, v: &Self) -> Self {
                Self(num_traits::SaturatingAdd::saturating_add(&self.0, &v.0))
            }
        }

        impl num_traits::SaturatingSub for $wrapper {
            fn saturating_sub(&self, v: &Self) -> Self {
                Self(num_traits::SaturatingSub::saturating_sub(&self.0, &v.0))
            }
        }
    };
}

macro_rules! impl_vec_wrapper {
    ($wrapper:ident, $item:ty, $field:ident) => {
        impl_vec_wrapper!(@inner $wrapper, $item, $field);
    };
    ($wrapper:ident, $item:ty) => {
        impl_vec_wrapper!(@inner $wrapper, $item, 0);

        impl std::convert::From<Vec<$item>> for $wrapper {
            fn from(vec: Vec<$item>) -> $wrapper {
                Self(vec)
            }
        }

        impl Default for $wrapper {
            fn default() -> Self {
                Self(Vec::new())
            }
        }
    };
    (@inner $wrapper:ident, $item:ty, $field:tt) => {
        impl<'a> std::iter::IntoIterator for &'a $wrapper {
            type Item = &'a $item;
            type IntoIter = std::slice::Iter<'a, $item>;

            fn into_iter(self) -> std::slice::Iter<'a, $item> {
                self.$field.iter()
            }
        }

        impl std::iter::IntoIterator for $wrapper {
            type Item = $item;
            type IntoIter = std::vec::IntoIter<$item>;

            fn into_iter(self) -> std::vec::IntoIter<$item> {
                self.$field.into_iter()
            }
        }

        impl std::iter::FromIterator<$item> for $wrapper {
            fn from_iter<I: IntoIterator<Item = $item>>(iter: I) -> Self {
                Self::from(iter.into_iter().collect::<Vec<_>>())
            }
        }

        impl std::iter::Extend<$item> for $wrapper {
            fn extend<I: IntoIterator<Item = $item>>(&mut self, iter: I) {
                self.$field.extend(iter);
            }
        }

        impl std::ops::Index<usize> for $wrapper {
            type Output = $item;

            fn index(&self, index: usize) -> &$item {
                &self.$field[index]
            }
        }

        impl<const N: usize> std::convert::From<[$item; N]> for $wrapper {
            fn from(slice: [$item; N]) -> $wrapper {
                Self::from(Vec::from(slice))
            }
        }

        impl std::ops::Deref for $wrapper {
            type Target = [$item];

            fn deref(&self) -> &[$item] {
                &self.$field
            }
        }

        impl AsRef<[$item]> for $wrapper {
            fn as_ref(&self) -> &[$item] {
                &self.$field
            }
        }

        impl $crate::NoneOrEmpty for $wrapper {
            fn is_none_or_empty(&self) -> bool {
                self.$field.is_empty()
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
        impl_num_ops!(@saturating TestNum, u64);

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

        #[quickcheck]
        fn checked_add(a: u32, b: u32) {
            use num_traits::CheckedAdd;
            let a = a as u64;
            let b = b as u64;
            assert_eq!(TestNum(a).checked_add(&TestNum(b)), Some(TestNum(a + b)));
        }

        #[test]
        fn checked_add_overflow() {
            use num_traits::CheckedAdd;
            assert_eq!(TestNum(u64::MAX).checked_add(&TestNum(1)), None);
        }

        #[quickcheck]
        fn checked_sub(a: u64, b: u64) {
            use num_traits::CheckedSub;
            if a >= b {
                assert_eq!(TestNum(a).checked_sub(&TestNum(b)), Some(TestNum(a - b)));
            }
        }

        #[test]
        fn checked_sub_underflow() {
            use num_traits::CheckedSub;
            assert_eq!(TestNum(0).checked_sub(&TestNum(1)), None);
        }

        #[quickcheck]
        fn checked_mul(a: u32, b: u32) {
            use num_traits::CheckedMul;
            let a = a as u64;
            let b = b as u64;
            assert_eq!(TestNum(a).checked_mul(&TestNum(b)), Some(TestNum(a * b)));
        }

        #[test]
        fn checked_mul_overflow() {
            use num_traits::CheckedMul;
            assert_eq!(TestNum(u64::MAX).checked_mul(&TestNum(2)), None);
        }

        #[quickcheck]
        fn checked_div(a: u64, b: u64) {
            use num_traits::CheckedDiv;
            if b != 0 {
                assert_eq!(TestNum(a).checked_div(&TestNum(b)), Some(TestNum(a / b)));
            }
        }

        #[test]
        fn checked_div_by_zero() {
            use num_traits::CheckedDiv;
            assert_eq!(TestNum(1).checked_div(&TestNum(0)), None);
        }

        #[quickcheck]
        fn saturating_add(a: u64, b: u64) {
            use num_traits::SaturatingAdd;
            if a <= u64::MAX - b {
                assert_eq!(TestNum(a).saturating_add(&TestNum(b)), TestNum(a+b));
            } else {
                assert_eq!(TestNum(a).saturating_add(&TestNum(b)), TestNum(u64::MAX));
            }
        }

        #[quickcheck]
        fn saturating_sub_within_range(a: u64, b: u64) {
            use num_traits::SaturatingSub;
            if a >= b {
                assert_eq!(TestNum(a).saturating_sub(&TestNum(b)), TestNum(a - b));
            } else {
                assert_eq!(TestNum(a).saturating_sub(&TestNum(b)), TestNum(0));
            }
        }
    }

    mod impl_vec_wrapper {
        #[derive(Debug, PartialEq)]
        struct TestWrapper(Vec<u32>);
        impl_vec_wrapper!(TestWrapper, u32);

        #[quickcheck]
        fn uses_vec_into_iterator_owned(vec: Vec<u32>) {
            assert_eq!(
                TestWrapper(vec.clone()).into_iter().collect::<Vec<_>>(),
                vec
            );
        }

        #[quickcheck]
        fn uses_vec_into_iterator_ref(vec: Vec<u32>) {
            assert_eq!(
                (&TestWrapper(vec.clone())).into_iter().collect::<Vec<_>>(),
                vec.iter().collect::<Vec<_>>()
            );
        }

        #[quickcheck]
        fn uses_vec_from_iterator(vec: Vec<u32>) {
            assert_eq!(
                TestWrapper(vec.clone()),
                vec.into_iter().collect::<TestWrapper>()
            )
        }

        #[quickcheck]
        fn uses_vec_extend(vec1: Vec<u32>, vec2: Vec<u32>) {
            let mut wrapper = TestWrapper(vec1.clone());
            wrapper.extend(vec2.iter().cloned());
            assert_eq!(wrapper, TestWrapper([vec1, vec2].concat()))
        }

        #[quickcheck]
        fn uses_vec_index(vec: Vec<u32>, index: usize) {
            let len = vec.len();
            if len != 0 {
                assert_eq!(TestWrapper(vec.clone())[index % len], vec[index % len])
            }
        }

        #[test]
        fn uses_vec_from_slice() {
            let mut arr = [0; 32];
            arr.copy_from_slice(&(0..32).collect::<Vec<_>>());
            assert_eq!(TestWrapper::from(arr.clone()), TestWrapper(arr.to_vec()))
        }

        #[quickcheck]
        fn from_wraps_vec(vec: Vec<u32>) {
            assert_eq!(TestWrapper::from(vec.clone()), TestWrapper(vec))
        }

        #[test]
        fn default_is_empty() {
            let wrapper = TestWrapper::default();
            assert!(wrapper.is_empty());
            assert!(!TestWrapper(vec![1]).is_empty());
        }

        #[test]
        fn deref_to_slice() {
            let wrapper = TestWrapper(vec![1, 2, 3]);
            let slice: &[u32] = &wrapper;
            assert_eq!(slice, &[1, 2, 3]);
            assert_eq!(wrapper.first(), Some(&1));
            assert_eq!(wrapper.last(), Some(&3));
        }

        #[test]
        fn as_ref_slice() {
            let wrapper = TestWrapper(vec![1, 2, 3]);
            let slice: &[u32] = wrapper.as_ref();
            assert_eq!(slice.len(), 3);
        }

        #[test]
        fn none_or_empty() {
            use crate::NoneOrEmpty;
            assert!(TestWrapper(vec![]).is_none_or_empty());
            assert!(!TestWrapper(vec![1]).is_none_or_empty());
        }
    }

    mod impl_vec_wrapper_named_field {
        #[derive(Debug, PartialEq)]
        struct TestNamedWrapper {
            items: Vec<u32>,
            extra: Option<bool>,
        }

        impl_vec_wrapper!(TestNamedWrapper, u32, items);

        impl From<Vec<u32>> for TestNamedWrapper {
            fn from(items: Vec<u32>) -> Self {
                Self { items, extra: None }
            }
        }

        #[quickcheck]
        fn uses_vec_into_iterator_owned(vec: Vec<u32>) {
            let wrapper = TestNamedWrapper { items: vec.clone(), extra: None };
            assert_eq!(wrapper.into_iter().collect::<Vec<_>>(), vec);
        }

        #[quickcheck]
        fn uses_vec_into_iterator_ref(vec: Vec<u32>) {
            let wrapper = TestNamedWrapper { items: vec.clone(), extra: None };
            assert_eq!(
                (&wrapper).into_iter().collect::<Vec<_>>(),
                vec.iter().collect::<Vec<_>>()
            );
        }

        #[quickcheck]
        fn uses_vec_from_iterator(vec: Vec<u32>) {
            let wrapper: TestNamedWrapper = vec.clone().into_iter().collect();
            assert_eq!(wrapper.items, vec);
            assert_eq!(wrapper.extra, None);
        }

        #[quickcheck]
        fn uses_vec_extend(vec1: Vec<u32>, vec2: Vec<u32>) {
            let mut wrapper = TestNamedWrapper { items: vec1.clone(), extra: Some(true) };
            wrapper.extend(vec2.iter().cloned());
            assert_eq!(wrapper.items, [vec1, vec2].concat());
            assert_eq!(wrapper.extra, Some(true));
        }

        #[quickcheck]
        fn uses_vec_index(vec: Vec<u32>, index: usize) {
            let len = vec.len();
            if len != 0 {
                let wrapper = TestNamedWrapper { items: vec.clone(), extra: None };
                assert_eq!(wrapper[index % len], vec[index % len]);
            }
        }

        #[test]
        fn from_slice_delegates_to_from_vec() {
            let wrapper = TestNamedWrapper::from([1, 2, 3]);
            assert_eq!(wrapper.items, vec![1, 2, 3]);
            assert_eq!(wrapper.extra, None);
        }

        #[test]
        fn deref_to_slice() {
            let wrapper = TestNamedWrapper { items: vec![1, 2, 3], extra: None };
            let slice: &[u32] = &wrapper;
            assert_eq!(slice, &[1, 2, 3]);
            assert_eq!(wrapper.first(), Some(&1));
        }

        #[test]
        fn none_or_empty() {
            use crate::NoneOrEmpty;
            let empty = TestNamedWrapper { items: vec![], extra: None };
            let non_empty = TestNamedWrapper { items: vec![1], extra: None };
            assert!(empty.is_none_or_empty());
            assert!(!non_empty.is_none_or_empty());
        }
    }
}
