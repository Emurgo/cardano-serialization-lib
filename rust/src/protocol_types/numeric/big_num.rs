use std::convert::TryFrom;
use std::ops::Div;

use crate::*;

// Generic u64 wrapper for platforms that don't support u64 or BigInt/etc
// This is an unsigned type - no negative numbers.
// Can be converted to/from plain rust
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default)]
pub struct BigNum(pub(crate) u64);

// Specifies an amount of ADA in terms of lovelace
pub type Coin = BigNum;

impl_to_from!(BigNum);
impl_num_from!(BigNum, u8, u16, u32, u64);
impl_num_into!(BigNum, u64, u128, i128);
impl_num_ops!(BigNum, u64);
impl_num_ops!(@saturating BigNum, u64);

impl std::fmt::Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[wasm_bindgen]
impl BigNum {
    // Create a BigNum from a standard rust string representation
    pub fn from_str(string: &str) -> Result<BigNum, JsError> {
        <Self as std::str::FromStr>::from_str(string)
    }

    // String representation of the BigNum value for use from environments that don't support BigInt
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn one() -> Self {
        Self(1)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn div_floor(&self, other: &BigNum) -> BigNum {
        // same as (a / b)
        let res = self.0.div(&other.0);
        Self(res)
    }

    pub fn checked_mul(&self, other: &BigNum) -> Result<BigNum, JsError> {
        <Self as num::CheckedMul>::checked_mul(&self, other)
            .ok_or_else(|| JsError::from_str("overflow"))
    }

    pub fn checked_add(&self, other: &BigNum) -> Result<BigNum, JsError> {
        <Self as num::CheckedAdd>::checked_add(&self, other)
            .ok_or_else(|| JsError::from_str("overflow"))
    }

    pub fn checked_sub(&self, other: &BigNum) -> Result<BigNum, JsError> {
        <Self as num::CheckedSub>::checked_sub(&self, other)
            .ok_or_else(|| JsError::from_str("underflow"))
    }

    /// returns 0 if it would otherwise underflow
    pub fn clamped_sub(&self, other: &BigNum) -> BigNum {
        <Self as num_traits::SaturatingSub>::saturating_sub(self, other)
    }

    pub fn compare(&self, rhs_value: &BigNum) -> i8 {
        match self.cmp(&rhs_value) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
        }
    }

    pub fn less_than(&self, rhs_value: &BigNum) -> bool {
        self.compare(rhs_value) < 0
    }

    pub fn max_value() -> BigNum {
        BigNum(u64::max_value())
    }

    pub fn max(a: &BigNum, b: &BigNum) -> BigNum {
        if a.less_than(b) {
            b.clone()
        } else {
            a.clone()
        }
    }
}

impl TryFrom<BigNum> for u32 {
    type Error = JsError;

    fn try_from(value: BigNum) -> Result<Self, Self::Error> {
        if value.0 > u32::MAX.into() {
            Err(JsError::from_str(&format!(
                "Value {} is bigger than max u32 {}",
                value.0,
                u32::MAX
            )))
        } else {
            Ok(value.0 as u32)
        }
    }
}

impl From<BigNum> for usize {
    fn from(value: BigNum) -> Self {
        value.0 as usize
    }
}

impl From<usize> for BigNum {
    fn from(value: usize) -> Self {
        return BigNum(value as u64);
    }
}

impl std::str::FromStr for BigNum {
    type Err = JsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map_err(|e| JsError::from_str(&format!("{:?}", e)))
            .map(BigNum)
    }
}

impl serde::Serialize for BigNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for BigNum {
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

impl JsonSchema for BigNum {
    fn schema_name() -> String {
        String::from("BigNum")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}
