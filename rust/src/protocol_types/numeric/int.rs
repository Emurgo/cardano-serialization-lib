use crate::*;
use std::convert::TryFrom;

// CBOR has int = uint / nint. Full range is [-2^64, 2^64-1].
// We store as i128 with an invariant validated by every constructor:
//     Int::MIN <= self.0 <= Int::MAX
// The field is `pub(crate)` to prevent external bypass; internal modules
// must use `Int::new_checked` / typed `From` impls / dedicated constructors.
#[wasm_bindgen]
#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int(pub(crate) i128);

impl_to_from!(Int);

impl Int {
    pub(crate) const MIN_I128: i128 = -(u64::MAX as i128) - 1; // -2^64
    pub(crate) const MAX_I128: i128 = u64::MAX as i128;        //  2^64 - 1

    pub(crate) fn new_checked(x: i128) -> Result<Self, JsError> {
        if x < Self::MIN_I128 || x > Self::MAX_I128 {
            Err(JsError::from_str(&format!(
                "{} out of CBOR int range [{}, {}]",
                x,
                Self::MIN_I128,
                Self::MAX_I128
            )))
        } else {
            Ok(Self(x))
        }
    }
}

#[wasm_bindgen]
impl Int {
    pub fn new(x: &BigNum) -> Self {
        Self(x.0 as i128)
    }

    pub fn new_negative(x: &BigNum) -> Self {
        Self(-(x.0 as i128))
    }

    pub fn new_i32(x: i32) -> Self {
        Self(x as i128)
    }

    pub fn is_positive(&self) -> bool {
        self.0 >= 0
    }

    /// BigNum can only contain unsigned u64 values
    ///
    /// This function will return the BigNum representation
    /// only in case the underlying i128 value is positive.
    ///
    /// Otherwise nothing will be returned (undefined).
    pub fn as_positive(&self) -> Option<BigNum> {
        if self.is_positive() {
            Some(BigNum(self.0 as u64))
        } else {
            None
        }
    }

    /// BigNum can only contain unsigned u64 values
    ///
    /// This function will return the *absolute* BigNum representation
    /// only in case the underlying i128 value is negative AND the
    /// absolute value fits in a u64. The single CBOR-int value -2^64
    /// (i.e. `Int::MIN_I128`) has |x| = 2^64, which does not fit a u64,
    /// so `None` is returned in that case as well.
    pub fn as_negative(&self) -> Option<BigNum> {
        if self.is_positive() {
            return None;
        }
        let abs = -self.0;
        if abs > u64::MAX as i128 {
            None
        } else {
            Some(BigNum(abs as u64))
        }
    }

    /// !!! DEPRECATED !!!
    /// Returns an i32 value in case the underlying original i128 value is within the limits.
    /// Otherwise will just return an empty value (undefined).
    #[deprecated(
    since = "10.0.0",
    note = "Unsafe ignoring of possible boundary error and it's not clear from the function name. Use `as_i32_or_nothing`, `as_i32_or_fail`, or `to_str`"
    )]
    pub fn as_i32(&self) -> Option<i32> {
        self.as_i32_or_nothing()
    }

    /// Returns the underlying value converted to i32 if possible (within limits)
    /// Otherwise will just return an empty value (undefined).
    pub fn as_i32_or_nothing(&self) -> Option<i32> {
        i32::try_from(self.0).ok()
    }

    /// Returns the underlying value converted to i32 if possible (within limits)
    /// JsError in case of out of boundary overflow
    pub fn as_i32_or_fail(&self) -> Result<i32, JsError> {
        i32::try_from(self.0).map_err(|e| JsError::from_str(&format!("{}", e)))
    }

    /// Returns string representation of the underlying i128 value directly.
    /// Might contain the minus sign (-) in case of negative value.
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    // Create an Int from a standard rust string representation
    pub fn from_str(string: &str) -> Result<Int, JsError> {
        <Self as std::str::FromStr>::from_str(string)
    }
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::convert::TryFrom<i128> for Int {
    type Error = JsError;

    fn try_from(x: i128) -> Result<Self, Self::Error> {
        Self::new_checked(x)
    }
}

impl std::str::FromStr for Int {
    type Err = JsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i128>()
            .map_err(|e| JsError::from_str(&format!("{:?}", e)))?
            .try_into()
    }
}

// Infallible From: every input type's range is a subset of [MIN, MAX].
impl From<i32> for Int {
    fn from(x: i32) -> Self {
        Self(x as i128)
    }
}
impl From<u32> for Int {
    fn from(x: u32) -> Self {
        Self(x as i128)
    }
}
impl From<i64> for Int {
    fn from(x: i64) -> Self {
        Self(x as i128)
    }
}
impl From<u64> for Int {
    fn from(x: u64) -> Self {
        Self(x as i128)
    }
}
impl From<BigNum> for Int {
    fn from(x: BigNum) -> Self {
        Self(x.0 as i128)
    }
}
impl From<Int> for i128 {
    fn from(x: Int) -> Self {
        x.0
    }
}
impl From<&Int> for i128 {
    fn from(x: &Int) -> Self {
        x.0
    }
}

// Checked / saturating arithmetic. We deliberately do NOT implement
// std::ops::{Add, Sub, Mul, Div, Rem, Neg} or num_traits::{CheckedAdd, ...}
// (the latter require Sub/Add bounds) on Int — it is a codec/domain wrapper,
// not a general numeric type. Callers that need math should use these
// inherent methods (which validate against [MIN_I128, MAX_I128]) or convert
// via `From<&Int> for i128`.
impl Int {
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        self.0
            .checked_add(other.0)
            .and_then(|x| Self::new_checked(x).ok())
    }
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.0
            .checked_sub(other.0)
            .and_then(|x| Self::new_checked(x).ok())
    }
    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        self.0
            .checked_mul(other.0)
            .and_then(|x| Self::new_checked(x).ok())
    }
    pub fn saturating_add(&self, other: &Self) -> Self {
        Self(
            self.0
                .saturating_add(other.0)
                .clamp(Self::MIN_I128, Self::MAX_I128),
        )
    }
    pub fn saturating_sub(&self, other: &Self) -> Self {
        Self(
            self.0
                .saturating_sub(other.0)
                .clamp(Self::MIN_I128, Self::MAX_I128),
        )
    }
}

impl serde::Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"string rep of a number",
            )
        })
    }
}

impl JsonSchema for Int {
    fn schema_name() -> String {
        String::from("Int")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}
