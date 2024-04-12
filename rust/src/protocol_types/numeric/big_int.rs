use num_bigint::Sign;
use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct BigInt(pub(crate) num_bigint::BigInt);

impl_to_from!(BigInt);

impl serde::Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        BigInt::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"string rep of a big int",
            )
        })
    }
}

impl JsonSchema for BigInt {
    fn schema_name() -> String {
        String::from("BigInt")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[wasm_bindgen]
impl BigInt {
    pub fn is_zero(&self) -> bool {
        self.0.sign() == Sign::NoSign
    }

    pub fn as_u64(&self) -> Option<BigNum> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        if sign == Sign::Minus {
            return None;
        }
        match u64_digits.len() {
            0 => Some(BigNum::zero()),
            1 => Some((*u64_digits.first().unwrap()).into()),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        let u64_digit = match u64_digits.len() {
            0 => Some(BigNum::zero()),
            1 => Some((*u64_digits.first().unwrap()).into()),
            _ => None,
        }?;
        match sign {
            num_bigint::Sign::NoSign | num_bigint::Sign::Plus => Some(Int::new(&u64_digit)),
            num_bigint::Sign::Minus => Some(Int::new_negative(&u64_digit)),
        }
    }

    pub fn from_str(text: &str) -> Result<BigInt, JsError> {
        use std::str::FromStr;
        num_bigint::BigInt::from_str(text)
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            .map(Self)
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }

    pub fn add(&self, other: &BigInt) -> BigInt {
        Self(&self.0 + &other.0)
    }

    pub fn mul(&self, other: &BigInt) -> BigInt {
        Self(&self.0 * &other.0)
    }

    pub fn one() -> BigInt {
        use std::str::FromStr;
        Self(num_bigint::BigInt::from_str("1").unwrap())
    }

    pub fn increment(&self) -> BigInt {
        self.add(&Self::one())
    }

    pub fn div_ceil(&self, other: &BigInt) -> BigInt {
        use num_integer::Integer;
        let (res, rem) = self.0.div_rem(&other.0);
        let result = Self(res);
        if Self(rem).is_zero() {
            result
        } else {
            result.increment()
        }
    }
}

impl<T> std::convert::From<T> for BigInt
    where
        T: std::convert::Into<num_bigint::BigInt>,
{
    fn from(x: T) -> Self {
        Self(x.into())
    }
}

impl From<BigNum> for BigInt {
    fn from(x: BigNum) -> Self {
        Self(x.0.into())
    }
}

impl From<&BigNum> for BigInt {
    fn from(x: &BigNum) -> Self {
        Self(x.0.into())
    }
}

pub fn to_bigint(val: u64) -> BigInt {
    BigInt::from_str(&val.to_string()).unwrap()
}