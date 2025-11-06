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

impl std::fmt::Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[wasm_bindgen]
impl BigNum {
    // Create a BigNum from a standard rust string representation
    pub fn from_str(string: &str) -> Result<BigNum, JsError> {
        string
            .parse::<u64>()
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            .map(BigNum)
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
        match self.0.checked_mul(other.0) {
            Some(value) => Ok(BigNum(value)),
            None => Err(JsError::from_str("overflow")),
        }
    }

    pub fn checked_add(&self, other: &BigNum) -> Result<BigNum, JsError> {
        match self.0.checked_add(other.0) {
            Some(value) => Ok(BigNum(value)),
            None => Err(JsError::from_str("overflow")),
        }
    }

    pub fn checked_sub(&self, other: &BigNum) -> Result<BigNum, JsError> {
        match self.0.checked_sub(other.0) {
            Some(value) => Ok(BigNum(value)),
            None => Err(JsError::from_str("underflow")),
        }
    }

    /// returns 0 if it would otherwise underflow
    pub fn clamped_sub(&self, other: &BigNum) -> BigNum {
        match self.0.checked_sub(other.0) {
            Some(value) => BigNum(value),
            None => BigNum(0),
        }
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

impl From<BigNum> for u64 {
    fn from(value: BigNum) -> Self {
        value.0
    }
}

impl From<&BigNum> for u64 {
    fn from(value: &BigNum) -> Self {
        value.0
    }
}

impl From<BigNum> for usize {
    fn from(value: BigNum) -> Self {
        value.0 as usize
    }
}

impl From<u64> for BigNum {
    fn from(value: u64) -> Self {
        return BigNum(value);
    }
}

impl From<usize> for BigNum {
    fn from(value: usize) -> Self {
        return BigNum(value as u64);
    }
}

impl From<u32> for BigNum {
    fn from(value: u32) -> Self {
        return BigNum(value.into());
    }
}

impl From<u16> for BigNum {
    fn from(value: u16) -> Self {
        return BigNum(value.into());
    }
}

impl From<u8> for BigNum {
    fn from(value: u8) -> Self {
        return BigNum(value.into());
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