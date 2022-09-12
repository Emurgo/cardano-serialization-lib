use cbor_event::{
    self,
    de::Deserializer,
    se::{Serialize, Serializer},
};
use hex::FromHex;
use num_bigint::Sign;
use serde_json;
use std::convert::TryFrom;
use std::ops::Div;
use std::{
    collections::HashMap,
    io::{BufRead, Seek, Write},
    ops::{Rem, Sub},
};
use std::fmt::Display;
use itertools::Itertools;

use super::*;
use crate::error::{DeserializeError, DeserializeFailure};
use crate::fakes::fake_data_hash;
use schemars::JsonSchema;

pub fn to_bytes<T: cbor_event::se::Serialize>(data_item: &T) -> Vec<u8> {
    let mut buf = Serializer::new_vec();
    data_item.serialize(&mut buf).unwrap();
    buf.finalize()
}

pub fn from_bytes<T: Deserialize>(data: &Vec<u8>) -> Result<T, DeserializeError> {
    let mut raw = Deserializer::from(std::io::Cursor::new(data));
    T::deserialize(&mut raw)
}



#[wasm_bindgen]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, JsonSchema,)]
pub struct TransactionUnspentOutput {
    pub(crate) input: TransactionInput,
    pub(crate) output: TransactionOutput,
}

impl_to_from!(TransactionUnspentOutput);

#[wasm_bindgen]
impl TransactionUnspentOutput {
    pub fn new(input: &TransactionInput, output: &TransactionOutput) -> TransactionUnspentOutput {
        Self {
            input: input.clone(),
            output: output.clone(),
        }
    }

    pub fn input(&self) -> TransactionInput {
        self.input.clone()
    }

    pub fn output(&self) -> TransactionOutput {
        self.output.clone()
    }
}

impl cbor_event::se::Serialize for TransactionUnspentOutput {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.input.serialize(serializer)?;
        self.output.serialize(serializer)
    }
}

impl Deserialize for TransactionUnspentOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::Array => {
                    let len = raw.array()?;
                    let input = (|| -> Result<_, DeserializeError> {
                        Ok(TransactionInput::deserialize(raw)?)
                    })()
                        .map_err(|e| e.annotate("input"))?;
                    let output = (|| -> Result<_, DeserializeError> {
                        Ok(TransactionOutput::deserialize(raw)?)
                    })()
                        .map_err(|e| e.annotate("output"))?;
                    let ret = Ok(Self { input, output });
                    match len {
                        cbor_event::Len::Len(n) => match n {
                            2 =>
                            /* it's ok */
                                {
                                    ()
                                }
                            n => {
                                return Err(
                                    DeserializeFailure::DefiniteLenMismatch(n, Some(2)).into()
                                );
                            }
                        },
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break =>
                            /* it's ok */
                                {
                                    ()
                                }
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    ret
                }
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("TransactionUnspentOutput"))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionUnspentOutputs(pub(crate) Vec<TransactionUnspentOutput>);

#[wasm_bindgen]
impl TransactionUnspentOutputs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionUnspentOutput {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionUnspentOutput) {
        self.0.push(elem.clone());
    }
}

// Generic u64 wrapper for platforms that don't support u64 or BigInt/etc
// This is an unsigned type - no negative numbers.
// Can be converted to/from plain rust
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigNum(u64);

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

    pub fn max(a: &BigNum, b: &BigNum) -> BigNum {
        if a.less_than(b) { b.clone() } else { a.clone() }
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

impl cbor_event::se::Serialize for BigNum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(self.0)
    }
}

impl Deserialize for BigNum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        match raw.unsigned_integer() {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(DeserializeError::new("BigNum", DeserializeFailure::CBOR(e))),
        }
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

pub fn to_bignum(val: u64) -> BigNum {
    BigNum(val)
}

pub fn from_bignum(val: &BigNum) -> u64 {
    val.0
}

pub fn to_bigint(val: u64) -> BigInt {
    BigInt::from_str(&val.to_string()).unwrap()
}

// Specifies an amount of ADA in terms of lovelace
pub type Coin = BigNum;

#[wasm_bindgen]
#[derive(
Clone,
Debug,
Eq,
/*Hash,*/ Ord,
PartialEq,
serde::Serialize,
serde::Deserialize,
JsonSchema,
)]
pub struct Value {
    pub(crate) coin: Coin,
    pub(crate) multiasset: Option<MultiAsset>,
}

impl_to_from!(Value);

#[wasm_bindgen]
impl Value {
    pub fn new(coin: &Coin) -> Value {
        Self {
            coin: coin.clone(),
            multiasset: None,
        }
    }

    pub fn new_from_assets(multiasset: &MultiAsset) -> Value {
        Value::new_with_assets(&Coin::zero(), multiasset)
    }

    pub fn new_with_assets(coin: &Coin, multiasset: &MultiAsset) -> Value {
        match multiasset.0.is_empty() {
            true => Value::new(coin),
            false => Self {
                coin: coin.clone(),
                multiasset: Some(multiasset.clone()),
            },
        }
    }

    pub fn zero() -> Value {
        Value::new(&Coin::zero())
    }

    pub fn is_zero(&self) -> bool {
        self.coin.is_zero()
            && self
            .multiasset
            .as_ref()
            .map(|m| m.len() == 0)
            .unwrap_or(true)
    }

    pub fn coin(&self) -> Coin {
        self.coin
    }

    pub fn set_coin(&mut self, coin: &Coin) {
        self.coin = coin.clone();
    }

    pub fn multiasset(&self) -> Option<MultiAsset> {
        self.multiasset.clone()
    }

    pub fn set_multiasset(&mut self, multiasset: &MultiAsset) {
        self.multiasset = Some(multiasset.clone());
    }

    pub fn checked_add(&self, rhs: &Value) -> Result<Value, JsError> {
        use std::collections::btree_map::Entry;
        let coin = self.coin.checked_add(&rhs.coin)?;

        let multiasset = match (&self.multiasset, &rhs.multiasset) {
            (Some(lhs_multiasset), Some(rhs_multiasset)) => {
                let mut multiasset = MultiAsset::new();

                for ma in &[lhs_multiasset, rhs_multiasset] {
                    for (policy, assets) in &ma.0 {
                        for (asset_name, amount) in &assets.0 {
                            match multiasset.0.entry(policy.clone()) {
                                Entry::Occupied(mut assets) => {
                                    match assets.get_mut().0.entry(asset_name.clone()) {
                                        Entry::Occupied(mut assets) => {
                                            let current = assets.get_mut();
                                            *current = current.checked_add(&amount)?;
                                        }
                                        Entry::Vacant(vacant_entry) => {
                                            vacant_entry.insert(amount.clone());
                                        }
                                    }
                                }
                                Entry::Vacant(entry) => {
                                    let mut assets = Assets::new();
                                    assets.0.insert(asset_name.clone(), amount.clone());
                                    entry.insert(assets);
                                }
                            }
                        }
                    }
                }

                Some(multiasset)
            }
            (None, None) => None,
            (Some(ma), None) => Some(ma.clone()),
            (None, Some(ma)) => Some(ma.clone()),
        };

        Ok(Value { coin, multiasset })
    }

    pub fn checked_sub(&self, rhs_value: &Value) -> Result<Value, JsError> {
        let coin = self.coin.checked_sub(&rhs_value.coin)?;
        let multiasset = match (&self.multiasset, &rhs_value.multiasset) {
            (Some(lhs_ma), Some(rhs_ma)) => match lhs_ma.sub(rhs_ma).len() {
                0 => None,
                _ => Some(lhs_ma.sub(rhs_ma)),
            },
            (Some(lhs_ma), None) => Some(lhs_ma.clone()),
            (None, Some(_rhs_ma)) => None,
            (None, None) => None,
        };

        Ok(Value { coin, multiasset })
    }

    pub fn clamped_sub(&self, rhs_value: &Value) -> Value {
        let coin = self.coin.clamped_sub(&rhs_value.coin);
        let multiasset = match (&self.multiasset, &rhs_value.multiasset) {
            (Some(lhs_ma), Some(rhs_ma)) => match lhs_ma.sub(rhs_ma).len() {
                0 => None,
                _ => Some(lhs_ma.sub(rhs_ma)),
            },
            (Some(lhs_ma), None) => Some(lhs_ma.clone()),
            (None, Some(_rhs_ma)) => None,
            (None, None) => None,
        };

        Value { coin, multiasset }
    }

    /// note: values are only partially comparable
    pub fn compare(&self, rhs_value: &Value) -> Option<i8> {
        match self.partial_cmp(&rhs_value) {
            None => None,
            Some(std::cmp::Ordering::Equal) => Some(0),
            Some(std::cmp::Ordering::Less) => Some(-1),
            Some(std::cmp::Ordering::Greater) => Some(1),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        fn compare_assets(
            lhs: &Option<MultiAsset>,
            rhs: &Option<MultiAsset>,
        ) -> Option<std::cmp::Ordering> {
            match (lhs, rhs) {
                (None, None) => Some(Equal),
                (None, Some(rhs_assets)) => MultiAsset::new().partial_cmp(&rhs_assets),
                (Some(lhs_assets), None) => lhs_assets.partial_cmp(&MultiAsset::new()),
                (Some(lhs_assets), Some(rhs_assets)) => lhs_assets.partial_cmp(&rhs_assets),
            }
        }

        compare_assets(&self.multiasset(), &other.multiasset()).and_then(|assets_match| {
            let coin_cmp = self.coin.cmp(&other.coin);

            match (coin_cmp, assets_match) {
                (coin_order, Equal) => Some(coin_order),
                (Equal, Less) => Some(Less),
                (Less, Less) => Some(Less),
                (Equal, Greater) => Some(Greater),
                (Greater, Greater) => Some(Greater),
                (_, _) => None,
            }
        })
    }
}

impl cbor_event::se::Serialize for Value {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match &self.multiasset {
            Some(multiasset) => {
                serializer.write_array(cbor_event::Len::Len(2))?;
                self.coin.serialize(serializer)?;
                multiasset.serialize(serializer)
            }
            None => self.coin.serialize(serializer),
        }
    }
}

impl Deserialize for Value {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::UnsignedInteger => Ok(Value::new(&Coin::deserialize(raw)?)),
                cbor_event::Type::Array => {
                    let len = raw.array()?;
                    let coin =
                        (|| -> Result<_, DeserializeError> { Ok(Coin::deserialize(raw)?) })()
                            .map_err(|e| e.annotate("coin"))?;
                    let multiasset =
                        (|| -> Result<_, DeserializeError> { Ok(MultiAsset::deserialize(raw)?) })()
                            .map_err(|e| e.annotate("multiasset"))?;
                    let ret = Ok(Self {
                        coin,
                        multiasset: Some(multiasset),
                    });
                    match len {
                        cbor_event::Len::Len(n) => match n {
                            2 =>
                            /* it's ok */
                                {
                                    ()
                                }
                            n => {
                                return Err(
                                    DeserializeFailure::DefiniteLenMismatch(n, Some(2)).into()
                                );
                            }
                        },
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break =>
                            /* it's ok */
                                {
                                    ()
                                }
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    ret
                }
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("Value"))
    }
}

// CBOR has int = uint / nint
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int(pub(crate) i128);

impl_to_from!(Int);

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
        return self.0 >= 0;
    }

    /// BigNum can only contain unsigned u64 values
    ///
    /// This function will return the BigNum representation
    /// only in case the underlying i128 value is positive.
    ///
    /// Otherwise nothing will be returned (undefined).
    pub fn as_positive(&self) -> Option<BigNum> {
        if self.is_positive() {
            Some(to_bignum(self.0 as u64))
        } else {
            None
        }
    }

    /// BigNum can only contain unsigned u64 values
    ///
    /// This function will return the *absolute* BigNum representation
    /// only in case the underlying i128 value is negative.
    ///
    /// Otherwise nothing will be returned (undefined).
    pub fn as_negative(&self) -> Option<BigNum> {
        if !self.is_positive() {
            Some(to_bignum((-self.0) as u64))
        } else {
            None
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
        use std::convert::TryFrom;
        i32::try_from(self.0).ok()
    }

    /// Returns the underlying value converted to i32 if possible (within limits)
    /// JsError in case of out of boundary overflow
    pub fn as_i32_or_fail(&self) -> Result<i32, JsError> {
        use std::convert::TryFrom;
        i32::try_from(self.0).map_err(|e| JsError::from_str(&format!("{}", e)))
    }

    /// Returns string representation of the underlying i128 value directly.
    /// Might contain the minus sign (-) in case of negative value.
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    // Create an Int from a standard rust string representation
    pub fn from_str(string: &str) -> Result<Int, JsError> {
        let x = string
            .parse::<i128>()
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))?;
        if x.abs() > u64::MAX as i128 {
            return Err(JsError::from_str(&format!(
                "{} out of bounds. Value (without sign) must fit within 4 bytes limit of {}",
                x,
                u64::MAX
            )));
        }
        Ok(Self(x))
    }
}

impl cbor_event::se::Serialize for Int {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.0 < 0 {
            serializer.write_negative_integer(self.0 as i64)
        } else {
            serializer.write_unsigned_integer(self.0 as u64)
        }
    }
}

impl Deserialize for Int {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::UnsignedInteger => Ok(Self(raw.unsigned_integer()? as i128)),
                cbor_event::Type::NegativeInteger => Ok(Self(read_nint(raw)?)),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("Int"))
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

/// TODO: this function can be removed in case `cbor_event` library ever gets a fix on their side
/// See https://github.com/Emurgo/cardano-serialization-lib/pull/392
fn read_nint<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<i128, DeserializeError> {
    let found = raw.cbor_type()?;
    if found != cbor_event::Type::NegativeInteger {
        return Err(cbor_event::Error::Expected(cbor_event::Type::NegativeInteger, found).into());
    }
    let (len, len_sz) = raw.cbor_len()?;
    match len {
        cbor_event::Len::Indefinite => Err(cbor_event::Error::IndefiniteLenNotSupported(
            cbor_event::Type::NegativeInteger,
        )
            .into()),
        cbor_event::Len::Len(v) => {
            raw.advance(1 + len_sz)?;
            Ok(-(v as i128) - 1)
        }
    }
}

const BOUNDED_BYTES_CHUNK_SIZE: usize = 64;

pub(crate) fn write_bounded_bytes<'se, W: Write>(
    serializer: &'se mut Serializer<W>,
    bytes: &[u8],
) -> cbor_event::Result<&'se mut Serializer<W>> {
    if bytes.len() <= BOUNDED_BYTES_CHUNK_SIZE {
        serializer.write_bytes(bytes)
    } else {
        // to get around not having access from outside the library we just write the raw CBOR indefinite byte string code here
        serializer.write_raw_bytes(&[0x5f])?;
        for chunk in bytes.chunks(BOUNDED_BYTES_CHUNK_SIZE) {
            serializer.write_bytes(chunk)?;
        }
        serializer.write_special(CBORSpecial::Break)
    }
}

pub(crate) fn read_bounded_bytes<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<Vec<u8>, DeserializeError> {
    use std::io::Read;
    let t = raw.cbor_type()?;
    if t != CBORType::Bytes {
        return Err(cbor_event::Error::Expected(CBORType::Bytes, t).into());
    }
    let (len, len_sz) = raw.cbor_len()?;
    match len {
        cbor_event::Len::Len(_) => {
            let bytes = raw.bytes()?;
            if bytes.len() > BOUNDED_BYTES_CHUNK_SIZE {
                return Err(DeserializeFailure::OutOfRange {
                    min: 0,
                    max: BOUNDED_BYTES_CHUNK_SIZE,
                    found: bytes.len(),
                }
                    .into());
            }
            Ok(bytes)
        }
        cbor_event::Len::Indefinite => {
            // this is CBOR indefinite encoding, but we must check that each chunk
            // is at most 64 big so we can't just use cbor_event's implementation
            // and check after the fact.
            // This is a slightly adopted version of what I made internally in cbor_event
            // but with the extra checks and not having access to non-pub methods.
            let mut bytes = Vec::new();
            raw.advance(1 + len_sz)?;
            // TODO: also change this + check at end of loop to the following after we update cbor_event
            //while raw.cbor_type()? != CBORType::Special || !raw.special_break()? {
            while raw.cbor_type()? != CBORType::Special {
                let chunk_t = raw.cbor_type()?;
                if chunk_t != CBORType::Bytes {
                    return Err(cbor_event::Error::Expected(CBORType::Bytes, chunk_t).into());
                }
                let (chunk_len, chunk_len_sz) = raw.cbor_len()?;
                match chunk_len {
                    // TODO: use this error instead once that PR is merged into cbor_event
                    //cbor_event::Len::Indefinite => return Err(cbor_event::Error::InvalidIndefiniteString.into()),
                    cbor_event::Len::Indefinite => {
                        return Err(cbor_event::Error::CustomError(String::from(
                            "Illegal CBOR: Indefinite string found inside indefinite string",
                        ))
                            .into());
                    }
                    cbor_event::Len::Len(len) => {
                        if chunk_len_sz > BOUNDED_BYTES_CHUNK_SIZE {
                            return Err(DeserializeFailure::OutOfRange {
                                min: 0,
                                max: BOUNDED_BYTES_CHUNK_SIZE,
                                found: chunk_len_sz,
                            }
                                .into());
                        }
                        raw.advance(1 + chunk_len_sz)?;
                        raw.as_mut_ref()
                            .by_ref()
                            .take(len)
                            .read_to_end(&mut bytes)
                            .map_err(|e| cbor_event::Error::IoError(e))?;
                    }
                }
            }
            if raw.special()? != CBORSpecial::Break {
                return Err(DeserializeFailure::EndingBreakMissing.into());
            }
            Ok(bytes)
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BigInt(num_bigint::BigInt);

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
        if sign == num_bigint::Sign::Minus {
            return None;
        }
        match u64_digits.len() {
            0 => Some(to_bignum(0)),
            1 => Some(to_bignum(*u64_digits.first().unwrap())),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        let u64_digit = match u64_digits.len() {
            0 => Some(to_bignum(0)),
            1 => Some(to_bignum(*u64_digits.first().unwrap())),
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

impl cbor_event::se::Serialize for BigInt {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        match u64_digits.len() {
            0 => serializer.write_unsigned_integer(0),
            // we use the uint/nint encodings to use a minimum of space
            1 => match sign {
                // uint
                num_bigint::Sign::Plus | num_bigint::Sign::NoSign => {
                    serializer.write_unsigned_integer(*u64_digits.first().unwrap())
                }
                // nint
                num_bigint::Sign::Minus => serializer
                    .write_negative_integer(-(*u64_digits.first().unwrap() as i128) as i64),
            },
            _ => {
                // Small edge case: nint's minimum is -18446744073709551616 but in this bigint lib
                // that takes 2 u64 bytes so we put that as a special case here:
                if sign == num_bigint::Sign::Minus && u64_digits == vec![0, 1] {
                    serializer.write_negative_integer(-18446744073709551616i128 as i64)
                } else {
                    let (sign, bytes) = self.0.to_bytes_be();
                    match sign {
                        // positive bigint
                        num_bigint::Sign::Plus | num_bigint::Sign::NoSign => {
                            serializer.write_tag(2u64)?;
                            write_bounded_bytes(serializer, &bytes)
                        }
                        // negative bigint
                        num_bigint::Sign::Minus => {
                            serializer.write_tag(3u64)?;
                            use std::ops::Neg;
                            // CBOR RFC defines this as the bytes of -n -1
                            let adjusted = self
                                .0
                                .clone()
                                .neg()
                                .checked_sub(&num_bigint::BigInt::from(1u32))
                                .unwrap()
                                .to_biguint()
                                .unwrap();
                            write_bounded_bytes(serializer, &adjusted.to_bytes_be())
                        }
                    }
                }
            }
        }
    }
}

impl Deserialize for BigInt {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                // bigint
                CBORType::Tag => {
                    let tag = raw.tag()?;
                    let bytes = read_bounded_bytes(raw)?;
                    match tag {
                        // positive bigint
                        2 => Ok(Self(num_bigint::BigInt::from_bytes_be(
                            num_bigint::Sign::Plus,
                            &bytes,
                        ))),
                        // negative bigint
                        3 => {
                            // CBOR RFC defines this as the bytes of -n -1
                            let initial =
                                num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);
                            use std::ops::Neg;
                            let adjusted = initial
                                .checked_add(&num_bigint::BigInt::from(1u32))
                                .unwrap()
                                .neg();
                            Ok(Self(adjusted))
                        }
                        _ => {
                            return Err(DeserializeFailure::TagMismatch {
                                found: tag,
                                expected: 2,
                            }
                                .into());
                        }
                    }
                }
                // uint
                CBORType::UnsignedInteger => {
                    Ok(Self(num_bigint::BigInt::from(raw.unsigned_integer()?)))
                }
                // nint
                CBORType::NegativeInteger => Ok(Self(num_bigint::BigInt::from(read_nint(raw)?))),
                _ => return Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("BigInt"))
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

// we use the cbor_event::Serialize trait directly

// This is only for use for plain cddl groups who need to be embedded within outer groups.
pub(crate) trait SerializeEmbeddedGroup {
    fn serialize_as_embedded_group<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

// same as cbor_event::de::Deserialize but with our DeserializeError
pub trait Deserialize {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError>
        where
            Self: Sized;
}

// auto-implement for all cbor_event Deserialize implementors
impl<T: cbor_event::de::Deserialize> Deserialize for T {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<T, DeserializeError> {
        T::deserialize(raw).map_err(|e| DeserializeError::from(e))
    }
}

// This is only for use for plain cddl groups who need to be embedded within outer groups.
pub trait DeserializeEmbeddedGroup {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError>
        where
            Self: Sized;
}

pub struct CBORReadLen {
    deser_len: cbor_event::Len,
    read: u64,
}

impl CBORReadLen {
    pub fn new(len: cbor_event::Len) -> Self {
        Self {
            deser_len: len,
            read: 0,
        }
    }

    // Marks {n} values as being read, and if we go past the available definite length
    // given by the CBOR, we return an error.
    pub fn read_elems(&mut self, count: usize) -> Result<(), DeserializeFailure> {
        match self.deser_len {
            cbor_event::Len::Len(n) => {
                self.read += count as u64;
                if self.read > n {
                    Err(DeserializeFailure::DefiniteLenMismatch(n, None))
                } else {
                    Ok(())
                }
            }
            cbor_event::Len::Indefinite => Ok(()),
        }
    }

    pub fn finish(&self) -> Result<(), DeserializeFailure> {
        match self.deser_len {
            cbor_event::Len::Len(n) => {
                if self.read == n {
                    Ok(())
                } else {
                    Err(DeserializeFailure::DefiniteLenMismatch(n, Some(self.read)))
                }
            }
            cbor_event::Len::Indefinite => Ok(()),
        }
    }
}

#[wasm_bindgen]
pub fn make_daedalus_bootstrap_witness(
    tx_body_hash: &TransactionHash,
    addr: &ByronAddress,
    key: &LegacyDaedalusPrivateKey,
) -> BootstrapWitness {
    let chain_code = key.chaincode();

    let pubkey = Bip32PublicKey::from_bytes(&key.0.to_public().as_ref()).unwrap();
    let vkey = Vkey::new(&pubkey.to_raw_key());
    let signature =
        Ed25519Signature::from_bytes(key.0.sign(&tx_body_hash.to_bytes()).as_ref().to_vec())
            .unwrap();

    BootstrapWitness::new(&vkey, &signature, chain_code, addr.attributes())
}

#[wasm_bindgen]
pub fn make_icarus_bootstrap_witness(
    tx_body_hash: &TransactionHash,
    addr: &ByronAddress,
    key: &Bip32PrivateKey,
) -> BootstrapWitness {
    let chain_code = key.chaincode();

    let raw_key = key.to_raw_key();
    let vkey = Vkey::new(&raw_key.to_public());
    let signature = raw_key.sign(&tx_body_hash.to_bytes());

    BootstrapWitness::new(&vkey, &signature, chain_code, addr.attributes())
}

#[wasm_bindgen]
pub fn make_vkey_witness(tx_body_hash: &TransactionHash, sk: &PrivateKey) -> Vkeywitness {
    let sig = sk.sign(tx_body_hash.0.as_ref());
    Vkeywitness::new(&Vkey::new(&sk.to_public()), &sig)
}

#[wasm_bindgen]
pub fn hash_auxiliary_data(auxiliary_data: &AuxiliaryData) -> AuxiliaryDataHash {
    AuxiliaryDataHash::from(blake2b256(&auxiliary_data.to_bytes()))
}

#[wasm_bindgen]
pub fn hash_transaction(tx_body: &TransactionBody) -> TransactionHash {
    TransactionHash::from(crypto::blake2b256(tx_body.to_bytes().as_ref()))
}

#[wasm_bindgen]
pub fn hash_plutus_data(plutus_data: &PlutusData) -> DataHash {
    DataHash::from(blake2b256(&plutus_data.to_bytes()))
}

#[wasm_bindgen]
pub fn hash_script_data(
    redeemers: &Redeemers,
    cost_models: &Costmdls,
    datums: Option<PlutusList>,
) -> ScriptDataHash {
    let mut buf = Vec::new();
    if redeemers.len() == 0 && datums.is_some() {
        /*
        ; Finally, note that in the case that a transaction includes datums but does not
        ; include any redeemers, the script data format becomes (in hex):
        ; [ 80 | datums | A0 ]
        ; corresponding to a CBOR empty list and an empty map (our apologies).
        */
        buf.push(0x80);
        if let Some(d) = &datums {
            buf.extend(d.to_bytes());
        }
        buf.push(0xA0);
    } else {
        /*
        ; script data format:
        ; [ redeemers | datums | language views ]
        ; The redeemers are exactly the data present in the transaction witness set.
        ; Similarly for the datums, if present. If no datums are provided, the middle
        ; field is an empty string.
        */
        buf.extend(redeemers.to_bytes());
        if let Some(d) = &datums {
            buf.extend(d.to_bytes());
        }
        buf.extend(cost_models.language_views_encoding());
    }
    ScriptDataHash::from(blake2b256(&buf))
}

// wasm-bindgen can't accept Option without clearing memory, so we avoid exposing this in WASM
pub fn internal_get_implicit_input(
    withdrawals: &Option<Withdrawals>,
    certs: &Option<Certificates>,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum,  // protocol parameter
) -> Result<Value, JsError> {
    let withdrawal_sum = match &withdrawals {
        None => to_bignum(0),
        Some(x) => {
            x.0.values()
                .try_fold(to_bignum(0), |acc, ref withdrawal_amt| {
                    acc.checked_add(&withdrawal_amt)
                })?
        }
    };
    let certificate_refund = match &certs {
        None => to_bignum(0),
        Some(certs) => certs
            .0
            .iter()
            .try_fold(to_bignum(0), |acc, ref cert| match &cert.0 {
                CertificateEnum::PoolRetirement(_cert) => acc.checked_add(&pool_deposit),
                CertificateEnum::StakeDeregistration(_cert) => acc.checked_add(&key_deposit),
                _ => Ok(acc),
            })?,
    };

    Ok(Value::new(
        &withdrawal_sum.checked_add(&certificate_refund)?,
    ))
}

pub fn internal_get_deposit(
    certs: &Option<Certificates>,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum,  // protocol parameter
) -> Result<Coin, JsError> {
    let certificate_refund = match &certs {
        None => to_bignum(0),
        Some(certs) => certs
            .0
            .iter()
            .try_fold(to_bignum(0), |acc, ref cert| match &cert.0 {
                CertificateEnum::PoolRegistration(_cert) => acc.checked_add(&pool_deposit),
                CertificateEnum::StakeRegistration(_cert) => acc.checked_add(&key_deposit),
                _ => Ok(acc),
            })?,
    };
    Ok(certificate_refund)
}

#[wasm_bindgen]
pub fn get_implicit_input(
    txbody: &TransactionBody,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum,  // protocol parameter
) -> Result<Value, JsError> {
    internal_get_implicit_input(
        &txbody.withdrawals,
        &txbody.certs,
        &pool_deposit,
        &key_deposit,
    )
}

#[wasm_bindgen]
pub fn get_deposit(
    txbody: &TransactionBody,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum,  // protocol parameter
) -> Result<Coin, JsError> {
    internal_get_deposit(&txbody.certs, &pool_deposit, &key_deposit)
}

// <TODO:REMOVE_AFTER_BABBAGE>
struct OutputSizeConstants {
    k0: usize,
    k1: usize,
    k2: usize,
}

// <TODO:REMOVE_AFTER_BABBAGE>
fn quot<T>(a: T, b: T) -> T
    where T: Sub<Output=T> + Rem<Output=T> + Div<Output=T> + Copy + Clone + std::fmt::Display {
    (a - (a % b)) / b
}

// <TODO:REMOVE_AFTER_BABBAGE>
fn bundle_size(
    assets: &Value,
    constants: &OutputSizeConstants,
) -> usize {
    // based on https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo-alonzo.rst
    match &assets.multiasset {
        None => 2, // coinSize according the minimum value function
        Some(assets) => {
            let num_assets = assets.0
                .values()
                .fold(
                    0,
                    |acc, next| acc + next.len(),
                );
            let sum_asset_name_lengths = assets.0
                .values()
                .flat_map(|assets| assets.0.keys())
                .unique_by(|asset| asset.name())
                .fold(
                    0,
                    |acc, next| acc + next.0.len(),
                );
            let sum_policy_id_lengths = assets.0
                .keys()
                .fold(
                    0,
                    |acc, next| acc + next.0.len(),
                );
            // converts bytes to 8-byte long words, rounding up
            fn roundup_bytes_to_words(b: usize) -> usize {
                quot(b + 7, 8)
            }
            constants.k0 + roundup_bytes_to_words(
                (num_assets * constants.k1) + sum_asset_name_lengths +
                    (constants.k2 * sum_policy_id_lengths)
            )
        }
    }
}

// <TODO:REMOVE_AFTER_BABBAGE>
fn _min_ada_required_legacy(
    assets: &Value,
    has_data_hash: bool, // whether the output includes a data hash
    coins_per_utxo_word: &BigNum, // protocol parameter (in lovelace)
) -> Result<BigNum, JsError> {
    // based on https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo-alonzo.rst
    let data_hash_size = if has_data_hash { 10 } else { 0 }; // in words
    let utxo_entry_size_without_val = 27; // in words

    let size = bundle_size(
        &assets,
        &OutputSizeConstants {
            k0: 6,
            k1: 12,
            k2: 1,
        },
    );
    let words = to_bignum(utxo_entry_size_without_val)
        .checked_add(&to_bignum(size as u64))?
        .checked_add(&to_bignum(data_hash_size))?;
    coins_per_utxo_word.checked_mul(&words)
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MinOutputAdaCalculator {
    output: TransactionOutput,
    data_cost: DataCost,
}

impl MinOutputAdaCalculator {
    pub fn new(output: &TransactionOutput, data_cost: &DataCost) -> Self {
        Self {
            output: output.clone(),
            data_cost: data_cost.clone(),
        }
    }

    pub fn new_empty(data_cost: &DataCost) -> Result<MinOutputAdaCalculator, JsError> {
        Ok(Self {
            output: MinOutputAdaCalculator::create_fake_output()?,
            data_cost: data_cost.clone(),
        })
    }

    pub fn set_address(&mut self, address: &Address) {
        self.output.address = address.clone();
    }

    pub fn set_plutus_data(&mut self, data: &PlutusData) {
        self.output.plutus_data = Some(DataOption::Data(data.clone()));
    }

    pub fn set_data_hash(&mut self, data_hash: &DataHash) {
        self.output.plutus_data = Some(DataOption::DataHash(data_hash.clone()));
    }

    pub fn set_amount(&mut self, amount: &Value) {
        self.output.amount = amount.clone();
    }

    pub fn set_script_ref(&mut self, script_ref: &ScriptRef) {
        self.output.script_ref = Some(script_ref.clone());
    }

    pub fn calculate_ada(&self) -> Result<BigNum, JsError> {
        let coins_per_byte = self.data_cost.coins_per_byte();
        // <TODO:REMOVE_AFTER_BABBAGE>
        let coins_per_word = self.data_cost.coins_per_word()?;
        let mut output: TransactionOutput = self.output.clone();
        fn calc_required_coin(
            output: &TransactionOutput,
            coins_per_byte: &Coin,
            coins_per_word: &Coin,
        ) -> Result<Coin, JsError> {
            // <TODO:REMOVE_AFTER_BABBAGE>
            let legacy_coin = _min_ada_required_legacy(
                &output.amount(),
                output.has_data_hash(),
                coins_per_word,
            )?;
            //according to https://hydra.iohk.io/build/15339994/download/1/babbage-changes.pdf
            //See on the page 9 getValue txout
            let result = BigNum::from(output.to_bytes().len())
                .checked_add(&to_bignum(160))?
                .checked_mul(&coins_per_byte)?;
            Ok(BigNum::max(&result, &legacy_coin))
        }
        for _ in 0..3 {
            let required_coin = calc_required_coin(&output, &coins_per_byte, &coins_per_word)?;
            if output.amount.coin.less_than(&required_coin) {
                output.amount.coin = required_coin.clone();
            } else {
                return Ok(required_coin);
            }
        }
        output.amount.coin = to_bignum(u64::MAX);
        calc_required_coin(&output, &coins_per_byte, &coins_per_word)
    }

    fn create_fake_output() -> Result<TransactionOutput, JsError> {
        let fake_base_address: Address = Address::from_bech32("addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w")?;
        let fake_value: Value = Value::new(&to_bignum(1000000));
        Ok(TransactionOutput::new(&fake_base_address, &fake_value))
    }
}

///returns minimal amount of ada for the output for case when the amount is included to the output
#[wasm_bindgen]
pub fn min_ada_for_output(
    output: &TransactionOutput,
    data_cost: &DataCost,
) -> Result<BigNum, JsError> {
    MinOutputAdaCalculator::new(output, data_cost).calculate_ada()
}

/// !!! DEPRECATED !!!
/// This function uses outdated set of arguments.
/// Use `min_ada_for_output` instead
#[wasm_bindgen]
#[deprecated(since = "11.0.0", note = "Use `min_ada_for_output` instead")]
pub fn min_ada_required(
    assets: &Value,
    has_data_hash: bool,          // whether the output includes a data hash
    coins_per_utxo_word: &BigNum, // protocol parameter (in lovelace)
) -> Result<BigNum, JsError> {
    let data_cost = DataCost::new_coins_per_word(coins_per_utxo_word);
    let mut calc = MinOutputAdaCalculator::new_empty(&data_cost)?;
    calc.set_amount(assets);
    if has_data_hash {
        calc.set_data_hash(&fake_data_hash(0));
    }
    calc.calculate_ada()
}

/// Used to choosed the schema for a script JSON string
#[wasm_bindgen]
pub enum ScriptSchema {
    Wallet,
    Node,
}

/// Receives a script JSON string
/// and returns a NativeScript.
/// Cardano Wallet and Node styles are supported.
///
/// * wallet: https://github.com/input-output-hk/cardano-wallet/blob/master/specifications/api/swagger.yaml
/// * node: https://github.com/input-output-hk/cardano-node/blob/master/doc/reference/simple-scripts.md
///
/// self_xpub is expected to be a Bip32PublicKey as hex-encoded bytes
#[wasm_bindgen]
pub fn encode_json_str_to_native_script(
    json: &str,
    self_xpub: &str,
    schema: ScriptSchema,
) -> Result<NativeScript, JsError> {
    let value: serde_json::Value =
        serde_json::from_str(&json).map_err(|e| JsError::from_str(&e.to_string()))?;

    let native_script = match schema {
        ScriptSchema::Wallet => encode_wallet_value_to_native_script(value, self_xpub)?,
        ScriptSchema::Node => todo!(),
    };

    Ok(native_script)
}

fn encode_wallet_value_to_native_script(
    value: serde_json::Value,
    self_xpub: &str,
) -> Result<NativeScript, JsError> {
    match value {
        serde_json::Value::Object(map)
        if map.contains_key("cosigners") && map.contains_key("template") =>
            {
                let mut cosigners = HashMap::new();

                if let serde_json::Value::Object(cosigner_map) = map.get("cosigners").unwrap() {
                    for (key, value) in cosigner_map.iter() {
                        if let serde_json::Value::String(xpub) = value {
                            if xpub == "self" {
                                cosigners.insert(key.to_owned(), self_xpub.to_owned());
                            } else {
                                cosigners.insert(key.to_owned(), xpub.to_owned());
                            }
                        } else {
                            return Err(JsError::from_str("cosigner value must be a string"));
                        }
                    }
                } else {
                    return Err(JsError::from_str("cosigners must be a map"));
                }

                let template = map.get("template").unwrap();

                let template_native_script = encode_template_to_native_script(template, &cosigners)?;

                Ok(template_native_script)
            }
        _ => Err(JsError::from_str(
            "top level must be an object. cosigners and template keys are required",
        )),
    }
}

fn encode_template_to_native_script(
    template: &serde_json::Value,
    cosigners: &HashMap<String, String>,
) -> Result<NativeScript, JsError> {
    match template {
        serde_json::Value::String(cosigner) => {
            if let Some(xpub) = cosigners.get(cosigner) {
                let bytes = Vec::from_hex(xpub).map_err(|e| JsError::from_str(&e.to_string()))?;

                let public_key = Bip32PublicKey::from_bytes(&bytes)?;

                Ok(NativeScript::new_script_pubkey(&ScriptPubkey::new(
                    &public_key.to_raw_key().hash(),
                )))
            } else {
                Err(JsError::from_str(&format!(
                    "cosigner {} not found",
                    cosigner
                )))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("all") => {
            let mut all = NativeScripts::new();

            if let serde_json::Value::Array(array) = map.get("all").unwrap() {
                for val in array {
                    all.add(&encode_template_to_native_script(val, cosigners)?);
                }
            } else {
                return Err(JsError::from_str("all must be an array"));
            }

            Ok(NativeScript::new_script_all(&ScriptAll::new(&all)))
        }
        serde_json::Value::Object(map) if map.contains_key("any") => {
            let mut any = NativeScripts::new();

            if let serde_json::Value::Array(array) = map.get("any").unwrap() {
                for val in array {
                    any.add(&encode_template_to_native_script(val, cosigners)?);
                }
            } else {
                return Err(JsError::from_str("any must be an array"));
            }

            Ok(NativeScript::new_script_any(&ScriptAny::new(&any)))
        }
        serde_json::Value::Object(map) if map.contains_key("some") => {
            if let serde_json::Value::Object(some) = map.get("some").unwrap() {
                if some.contains_key("at_least") && some.contains_key("from") {
                    let n = if let serde_json::Value::Number(at_least) =
                    some.get("at_least").unwrap()
                    {
                        if let Some(n) = at_least.as_u64() {
                            n as u32
                        } else {
                            return Err(JsError::from_str("at_least must be an integer"));
                        }
                    } else {
                        return Err(JsError::from_str("at_least must be an integer"));
                    };

                    let mut from_scripts = NativeScripts::new();

                    if let serde_json::Value::Array(array) = some.get("from").unwrap() {
                        for val in array {
                            from_scripts.add(&encode_template_to_native_script(val, cosigners)?);
                        }
                    } else {
                        return Err(JsError::from_str("from must be an array"));
                    }

                    Ok(NativeScript::new_script_n_of_k(&ScriptNOfK::new(
                        n,
                        &from_scripts,
                    )))
                } else {
                    Err(JsError::from_str("some must contain at_least and from"))
                }
            } else {
                Err(JsError::from_str("some must be an object"))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("active_from") => {
            if let serde_json::Value::Number(active_from) = map.get("active_from").unwrap() {
                if let Some(n) = active_from.as_u64() {
                    let slot: SlotBigNum = n.into();

                    let time_lock_start = TimelockStart::new_timelockstart(&slot);

                    Ok(NativeScript::new_timelock_start(&time_lock_start))
                } else {
                    Err(JsError::from_str(
                        "active_from slot must be an integer greater than or equal to 0",
                    ))
                }
            } else {
                Err(JsError::from_str("active_from slot must be a number"))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("active_until") => {
            if let serde_json::Value::Number(active_until) = map.get("active_until").unwrap() {
                if let Some(n) = active_until.as_u64() {
                    let slot: SlotBigNum = n.into();

                    let time_lock_expiry = TimelockExpiry::new_timelockexpiry(&slot);

                    Ok(NativeScript::new_timelock_expiry(&time_lock_expiry))
                } else {
                    Err(JsError::from_str(
                        "active_until slot must be an integer greater than or equal to 0",
                    ))
                }
            } else {
                Err(JsError::from_str("active_until slot must be a number"))
            }
        }
        _ => Err(JsError::from_str("invalid template format")),
    }
}

pub(crate) fn opt64<T>(o: &Option<T>) -> u64 {
    o.is_some() as u64
}

pub struct ValueShortage {
    pub(crate) ada_shortage: Option<(Coin, Coin, Coin)>,
    pub(crate) asset_shortage: Vec<(PolicyID, AssetName, Coin, Coin)>,
}

impl Display for ValueShortage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shortage: {{")?;
        if let Some((input_data, out_data, fee)) = self.ada_shortage {
            writeln!(f, "ada in inputs: {}, ada in outputs: {}, fee {}", input_data, out_data, fee)?;
            writeln!(f, "NOTE! \"ada in inputs\" must be >= (\"ada in outputs\" + fee) before adding change")?;
            writeln!(f, "and  \"ada in inputs\" must be == (\"ada in outputs\" + fee) after adding change")?;
        }
        for (policy_id, asset_name, asset_shortage, asset_available) in
            &self.asset_shortage
        {
            write!(f, "policy id: \"{}\", asset name: \"{}\" ", policy_id, asset_name)?;
            writeln!(f, "coins in inputs: {}, coins in outputs: {}", asset_shortage, asset_available)?;
        }
        write!(f, " }}")
    }
}

pub(crate) fn get_input_shortage(all_inputs_value: &Value, all_outputs_value: &Value, fee: &Coin)
    -> Result<Option<ValueShortage>, JsError> {
    let mut shortage = ValueShortage{
        ada_shortage: None,
        asset_shortage: Vec::new()};
    if all_inputs_value.coin < all_outputs_value.coin.checked_add(fee)? {
        shortage.ada_shortage = Some((
            all_inputs_value.coin.clone(),
            all_outputs_value.coin.clone(),
            fee.clone()));
    }

    if let Some(policies) = &all_outputs_value.multiasset {
        for (policy_id, assets) in &policies.0 {
            for (asset_name, coins) in &assets.0 {
                let inputs_coins = match &all_inputs_value.multiasset {
                    Some(multiasset) => multiasset.get_asset(policy_id, asset_name),
                    None => Coin::zero()
                };

                if inputs_coins < *coins {
                    shortage.asset_shortage.push((policy_id.clone(), asset_name.clone(), inputs_coins, coins.clone()));
                }
            }
        }
    }

    if shortage.ada_shortage.is_some() || shortage.asset_shortage.len() > 0 {
        Ok(Some(shortage))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tx_builder_constants::TxBuilderConstants;

    // this is what is used in mainnet
    const COINS_PER_UTXO_WORD: u64 = 34_482;

    // taken from https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo-alonzo.rst
    fn one_policy_one_0_char_asset() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(&AssetName(vec![]), &BigNum(1));
        token_bundle.insert(&PolicyID::from([0; ScriptHash::BYTE_COUNT]), &asset_list);
        Value {
            coin: BigNum(0),
            multiasset: Some(token_bundle),
        }
    }

    fn one_policy_one_1_char_asset() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(&AssetName(vec![1]), &BigNum(1));
        token_bundle.insert(&PolicyID::from([0; ScriptHash::BYTE_COUNT]), &asset_list);
        Value {
            coin: BigNum(1407406),
            multiasset: Some(token_bundle),
        }
    }

    fn one_policy_three_1_char_assets() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(&AssetName(vec![1]), &BigNum(1));
        asset_list.insert(&AssetName(vec![2]), &BigNum(1));
        asset_list.insert(&AssetName(vec![3]), &BigNum(1));
        token_bundle.insert(&PolicyID::from([0; ScriptHash::BYTE_COUNT]), &asset_list);
        Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle),
        }
    }

    fn two_policies_one_0_char_asset() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(&AssetName(vec![]), &BigNum(1));
        token_bundle.insert(&PolicyID::from([0; ScriptHash::BYTE_COUNT]), &asset_list);
        token_bundle.insert(&PolicyID::from([1; ScriptHash::BYTE_COUNT]), &asset_list);
        Value {
            coin: BigNum(1592591),
            multiasset: Some(token_bundle),
        }
    }

    fn two_policies_one_1_char_asset() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(&AssetName(vec![1]), &BigNum(1));
        token_bundle.insert(&PolicyID::from([0; ScriptHash::BYTE_COUNT]), &asset_list);
        token_bundle.insert(&PolicyID::from([1; ScriptHash::BYTE_COUNT]), &asset_list);
        Value {
            coin: BigNum(1592591),
            multiasset: Some(token_bundle),
        }
    }

    fn three_policies_96_1_char_assets() -> Value {
        let mut token_bundle = MultiAsset::new();
        fn add_policy(token_bundle: &mut MultiAsset, index: u8) -> () {
            let mut asset_list = Assets::new();

            for i in 0..32 {
                asset_list.insert(&AssetName(vec![index * 32 + i]), &BigNum(1));
            }
            token_bundle.insert(
                &PolicyID::from([index; ScriptHash::BYTE_COUNT]),
                &asset_list,
            );
        }
        add_policy(&mut token_bundle, 1);
        add_policy(&mut token_bundle, 2);
        add_policy(&mut token_bundle, 3);
        Value {
            coin: BigNum(7592585),
            multiasset: Some(token_bundle),
        }
    }

    fn one_policy_three_32_char_assets() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(&AssetName(vec![1; 32]), &BigNum(1));
        asset_list.insert(&AssetName(vec![2; 32]), &BigNum(1));
        asset_list.insert(&AssetName(vec![3; 32]), &BigNum(1));
        token_bundle.insert(&PolicyID::from([0; ScriptHash::BYTE_COUNT]), &asset_list);
        Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle),
        }
    }

    #[ignore]
    #[test]
    fn min_ada_value_no_multiasset() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &Value::new(&Coin::zero()),
                    false,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            969750,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_one_policy_one_0_char_asset() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &one_policy_one_0_char_asset(),
                    false,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            1_120_600,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_one_policy_one_1_char_asset() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &one_policy_one_1_char_asset(),
                    false,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            1_124_910,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_one_policy_three_1_char_assets() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &one_policy_three_1_char_assets(),
                    false,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            1_150_770,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_two_policies_one_0_char_asset() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &two_policies_one_0_char_asset(),
                    false,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            1_262_830,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_two_policies_one_1_char_asset() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &two_policies_one_1_char_asset(),
                    false,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            1_271_450,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_three_policies_96_1_char_assets() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &three_policies_96_1_char_assets(),
                    false,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            2_633_410,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_one_policy_one_0_char_asset_datum_hash() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &one_policy_one_0_char_asset(),
                    true,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            1_267_140,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_one_policy_three_32_char_assets_datum_hash() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &one_policy_three_32_char_assets(),
                    true,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            1_711_070,
        );
    }

    #[ignore]
    #[test]
    fn min_ada_value_two_policies_one_0_char_asset_datum_hash() {
        assert_eq!(
            from_bignum(
                &min_ada_required(
                    &two_policies_one_0_char_asset(),
                    true,
                    &to_bignum(COINS_PER_UTXO_WORD),
                )
                    .unwrap()
            ),
            1_409_370,
        );
    }

    #[test]
    fn subtract_values() {
        let policy1 = PolicyID::from([0; ScriptHash::BYTE_COUNT]);
        let policy2 = PolicyID::from([1; ScriptHash::BYTE_COUNT]);

        let asset1 = AssetName(vec![1]);
        let asset2 = AssetName(vec![2]);
        let asset3 = AssetName(vec![3]);
        let asset4 = AssetName(vec![4]);

        let mut token_bundle1 = MultiAsset::new();
        {
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            asset_list1.insert(&asset2, &BigNum(1));
            asset_list1.insert(&asset3, &BigNum(1));
            asset_list1.insert(&asset4, &BigNum(2));
            token_bundle1.insert(&policy1, &asset_list1);

            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy2, &asset_list2);
        }
        let assets1 = Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        {
            let mut asset_list2 = Assets::new();
            // more than asset1 bundle
            asset_list2.insert(&asset1, &BigNum(2));
            // exactly equal to asset1 bundle
            asset_list2.insert(&asset2, &BigNum(1));
            // skip asset 3
            // less than in asset1 bundle
            asset_list2.insert(&asset4, &BigNum(1));
            token_bundle2.insert(&policy1, &asset_list2);

            // this policy should be removed entirely
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(1));
            token_bundle2.insert(&policy2, &asset_list2);
        }

        let assets2 = Value {
            coin: BigNum(2555554),
            multiasset: Some(token_bundle2),
        };

        let result = assets1.clamped_sub(&assets2);
        assert_eq!(result.coin().to_str(), "0");
        assert_eq!(
            result.multiasset().unwrap().len(),
            1 // policy 2 was deleted successfully
        );
        let policy1_content = result.multiasset().unwrap().get(&policy1).unwrap();
        assert_eq!(policy1_content.len(), 2);
        assert_eq!(policy1_content.get(&asset3).unwrap().to_str(), "1");
        assert_eq!(policy1_content.get(&asset4).unwrap().to_str(), "1");
    }

    #[test]
    fn compare_values() {
        let policy1 = PolicyID::from([0; ScriptHash::BYTE_COUNT]);

        let asset1 = AssetName(vec![1]);
        let asset2 = AssetName(vec![2]);

        // testing cases with no assets
        {
            let a = Value::new(&to_bignum(1));
            let b = Value::new(&to_bignum(1));
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Equal);
        }
        {
            let a = Value::new(&to_bignum(2));
            let b = Value::new(&to_bignum(1));
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let a = Value::new(&to_bignum(1));
            let b = Value::new(&to_bignum(2));
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        // testing case where one side has assets
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };
            let b = Value::new(&to_bignum(1));
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value::new(&to_bignum(1));
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        // testing case where both sides has assets
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(1));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Equal);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(1));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(1));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(2));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(1));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(2));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(1));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(2));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(1));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b), None);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(2));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(2));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset1, &BigNum(2));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b), None);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(&asset1, &BigNum(1));
            token_bundle1.insert(&policy1, &asset_list1);
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(&asset2, &BigNum(1));
            token_bundle2.insert(&policy1, &asset_list2);
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b), None);
        }
    }

    #[test]
    fn bigint_serialization() {
        let zero = BigInt::from_str("0").unwrap();
        let zero_rt = BigInt::from_bytes(zero.to_bytes()).unwrap();
        assert_eq!(zero.to_str(), zero_rt.to_str());
        assert_eq!(zero.to_bytes(), vec![0x00]);

        let pos_small = BigInt::from_str("100").unwrap();
        let pos_small_rt = BigInt::from_bytes(pos_small.to_bytes()).unwrap();
        assert_eq!(pos_small.to_str(), pos_small_rt.to_str());

        let pos_big = BigInt::from_str("123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890").unwrap();
        let pos_big_rt = BigInt::from_bytes(pos_big.to_bytes()).unwrap();
        assert_eq!(pos_big.to_str(), pos_big_rt.to_str());

        let neg_small = BigInt::from_str("-100").unwrap();
        let neg_small_rt = BigInt::from_bytes(neg_small.to_bytes()).unwrap();
        assert_eq!(neg_small.to_str(), neg_small_rt.to_str());

        let neg_big = BigInt::from_str("-123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890").unwrap();
        let neg_big_rt = BigInt::from_bytes(neg_big.to_bytes()).unwrap();
        assert_eq!(neg_big.to_str(), neg_big_rt.to_str());

        // taken from CBOR RFC examples
        // negative big int
        assert_eq!(
            hex::decode("c349010000000000000000").unwrap(),
            BigInt::from_str("-18446744073709551617")
                .unwrap()
                .to_bytes()
        );
        // positive big int
        assert_eq!(
            hex::decode("c249010000000000000000").unwrap(),
            BigInt::from_str("18446744073709551616").unwrap().to_bytes()
        );
        // uint
        assert_eq!(
            hex::decode("1b000000e8d4a51000").unwrap(),
            BigInt::from_str("1000000000000").unwrap().to_bytes()
        );
        // nint (lowest possible - used to be unsupported but works now)
        assert_eq!(
            hex::decode("3bffffffffffffffff").unwrap(),
            BigInt::from_str("-18446744073709551616")
                .unwrap()
                .to_bytes()
        );
        // this one fits in an i64 though
        assert_eq!(
            hex::decode("3903e7").unwrap(),
            BigInt::from_str("-1000").unwrap().to_bytes()
        );

        let x = BigInt::from_str("-18446744073709551617").unwrap();
        let x_rt = BigInt::from_bytes(x.to_bytes()).unwrap();
        assert_eq!(x.to_str(), x_rt.to_str());
    }

    #[test]
    fn bounded_bytes_read_chunked() {
        use std::io::Cursor;
        let chunks = vec![
            vec![
                0x52, 0x73, 0x6F, 0x6D, 0x65, 0x20, 0x72, 0x61, 0x6E, 0x64, 0x6F, 0x6D, 0x20, 0x73,
                0x74, 0x72, 0x69, 0x6E, 0x67,
            ],
            vec![0x44, 0x01, 0x02, 0x03, 0x04],
        ];
        let mut expected = Vec::new();
        for chunk in chunks.iter() {
            expected.extend_from_slice(&chunk[1..]);
        }
        let mut vec = vec![0x5f];
        for mut chunk in chunks {
            vec.append(&mut chunk);
        }
        vec.push(0xff);
        let mut raw = Deserializer::from(Cursor::new(vec.clone()));
        let found = read_bounded_bytes(&mut raw).unwrap();
        assert_eq!(found, expected);
    }

    #[test]
    fn bounded_bytes_write_chunked() {
        let mut chunk_64 = vec![0x58, BOUNDED_BYTES_CHUNK_SIZE as u8];
        chunk_64.extend(std::iter::repeat(37).take(BOUNDED_BYTES_CHUNK_SIZE));
        let chunks = vec![chunk_64, vec![0x44, 0x01, 0x02, 0x03, 0x04]];
        let mut input = Vec::new();
        input.extend_from_slice(&chunks[0][2..]);
        input.extend_from_slice(&chunks[1][1..]);
        let mut serializer = cbor_event::se::Serializer::new_vec();
        write_bounded_bytes(&mut serializer, &input).unwrap();
        let written = serializer.finalize();
        let mut expected = vec![0x5f];
        for mut chunk in chunks {
            expected.append(&mut chunk);
        }
        expected.push(0xff);
        assert_eq!(expected, written);
    }

    #[test]
    fn correct_script_data_hash() {
        let mut datums = PlutusList::new();
        datums.add(&PlutusData::new_integer(&BigInt::from_str("1000").unwrap()));
        let mut redeemers = Redeemers::new();
        redeemers.add(&Redeemer::new(
            &RedeemerTag::new_spend(),
            &BigNum::from_str("1").unwrap(),
            &PlutusData::new_integer(&BigInt::from_str("2000").unwrap()),
            &ExUnits::new(
                &BigNum::from_str("0").unwrap(),
                &BigNum::from_str("0").unwrap(),
            ),
        ));
        let plutus_cost_model = CostModel::from_bytes(vec![
            159, 26, 0, 3, 2, 89, 0, 1, 1, 26, 0, 6, 11, 199, 25, 2, 109, 0, 1, 26, 0, 2, 73, 240,
            25, 3, 232, 0, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 37, 206, 168, 25, 113, 247, 4, 25,
            116, 77, 24, 100, 25, 116, 77, 24, 100, 25, 116, 77, 24, 100, 25, 116, 77, 24, 100, 25,
            116, 77, 24, 100, 25, 116, 77, 24, 100, 24, 100, 24, 100, 25, 116, 77, 24, 100, 26, 0,
            2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73,
            240, 25, 3, 232, 0, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 25, 3, 232, 0, 8,
            26, 0, 2, 66, 32, 26, 0, 6, 126, 35, 24, 118, 0, 1, 1, 26, 0, 2, 73, 240, 25, 3, 232,
            0, 8, 26, 0, 2, 73, 240, 26, 0, 1, 183, 152, 24, 247, 1, 26, 0, 2, 73, 240, 25, 39, 16,
            1, 26, 0, 2, 21, 94, 25, 5, 46, 1, 25, 3, 232, 26, 0, 2, 73, 240, 25, 3, 232, 1, 26, 0,
            2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 1, 1, 26, 0,
            2, 73, 240, 1, 26, 0, 2, 73, 240, 4, 26, 0, 1, 148, 175, 24, 248, 1, 26, 0, 1, 148,
            175, 24, 248, 1, 26, 0, 2, 55, 124, 25, 5, 86, 1, 26, 0, 2, 189, 234, 25, 1, 241, 1,
            26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0,
            2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 66,
            32, 26, 0, 6, 126, 35, 24, 118, 0, 1, 1, 25, 240, 76, 25, 43, 210, 0, 1, 26, 0, 2, 73,
            240, 24, 32, 26, 0, 2, 66, 32, 26, 0, 6, 126, 35, 24, 118, 0, 1, 1, 26, 0, 2, 66, 32,
            26, 0, 6, 126, 35, 24, 118, 0, 1, 1, 26, 0, 37, 206, 168, 25, 113, 247, 4, 0, 26, 0, 1,
            65, 187, 4, 26, 0, 2, 73, 240, 25, 19, 136, 0, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 3,
            2, 89, 0, 1, 1, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73,
            240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 2, 73, 240,
            24, 32, 26, 0, 2, 73, 240, 24, 32, 26, 0, 51, 13, 167, 1, 1, 255,
        ])
            .unwrap();
        let mut cost_models = Costmdls::new();
        cost_models.insert(&Language::new_plutus_v1(), &plutus_cost_model);
        let script_data_hash = hash_script_data(&redeemers, &cost_models, Some(datums));

        assert_eq!(
            hex::encode(script_data_hash.to_bytes()),
            "4415e6667e6d6bbd992af5092d48e3c2ba9825200d0234d2470068f7f0f178b3"
        );
    }

    #[test]
    fn native_scripts_from_wallet_json() {
        let cosigner0_hex = "1423856bc91c49e928f6f30f4e8d665d53eb4ab6028bd0ac971809d514c92db11423856bc91c49e928f6f30f4e8d665d53eb4ab6028bd0ac971809d514c92db1";
        let cosigner1_hex = "a48d97f57ce49433f347d44ee07e54a100229b4f8e125d25f7bca9ad66d9707a25cd1331f46f7d6e279451637ca20802a25c441ba9436abf644fe5410d1080e3";
        let self_key_hex = "6ce83a12e9d4c783f54c0bb511303b37160a6e4f3f96b8e878a7c1f7751e18c4ccde3fb916d330d07f7bd51fb6bd99aa831d925008d3f7795033f48abd6df7f6";
        let native_script = encode_json_str_to_native_script(
            &format!(
                r#"
        {{
            "cosigners": {{
                "cosigner#0": "{}",
                "cosigner#1": "{}",
                "cosigner#2": "self"
            }},
            "template": {{
                "some": {{
                    "at_least": 2,
                    "from": [
                        {{
                            "all": [
                                "cosigner#0",
                                {{ "active_from": 120 }}
                            ]
                        }},
                        {{
                            "any": [
                                "cosigner#1",
                                {{ "active_until": 1000 }}
                            ]
                        }},
                        "cosigner#2"
                    ]
                }}
            }}
        }}"#,
                cosigner0_hex, cosigner1_hex
            ),
            self_key_hex,
            ScriptSchema::Wallet,
        );

        let n_of_k = native_script.unwrap().as_script_n_of_k().unwrap();
        let from = n_of_k.native_scripts();
        assert_eq!(n_of_k.n(), 2);
        assert_eq!(from.len(), 3);
        let all = from.get(0).as_script_all().unwrap().native_scripts();
        assert_eq!(all.len(), 2);
        let all_0 = all.get(0).as_script_pubkey().unwrap();
        assert_eq!(
            all_0.addr_keyhash(),
            Bip32PublicKey::from_bytes(&hex::decode(cosigner0_hex).unwrap())
                .unwrap()
                .to_raw_key()
                .hash()
        );
        let all_1 = all.get(1).as_timelock_start().unwrap();
        assert_eq!(all_1.slot().unwrap(), 120);
        let any = from.get(1).as_script_any().unwrap().native_scripts();
        assert_eq!(all.len(), 2);
        let any_0 = any.get(0).as_script_pubkey().unwrap();
        assert_eq!(
            any_0.addr_keyhash(),
            Bip32PublicKey::from_bytes(&hex::decode(cosigner1_hex).unwrap())
                .unwrap()
                .to_raw_key()
                .hash()
        );
        let any_1 = any.get(1).as_timelock_expiry().unwrap();
        assert_eq!(any_1.slot().unwrap(), 1000);
        let self_key = from.get(2).as_script_pubkey().unwrap();
        assert_eq!(
            self_key.addr_keyhash(),
            Bip32PublicKey::from_bytes(&hex::decode(self_key_hex).unwrap())
                .unwrap()
                .to_raw_key()
                .hash()
        );
    }

    #[test]
    fn int_to_str() {
        assert_eq!(
            Int::new(&BigNum(u64::max_value())).to_str(),
            u64::max_value().to_string()
        );
        assert_eq!(
            Int::new(&BigNum(u64::min_value())).to_str(),
            u64::min_value().to_string()
        );
        assert_eq!(
            Int::new_negative(&BigNum(u64::max_value())).to_str(),
            (-(u64::max_value() as i128)).to_string()
        );
        assert_eq!(
            Int::new_negative(&BigNum(u64::min_value())).to_str(),
            (-(u64::min_value() as i128)).to_string()
        );
        assert_eq!(Int::new_i32(142).to_str(), "142");
        assert_eq!(Int::new_i32(-142).to_str(), "-142");
    }

    #[test]
    fn int_as_i32_or_nothing() {
        let over_pos_i32 = (i32::max_value() as i64) + 1;
        assert!(Int::new(&BigNum(over_pos_i32 as u64))
            .as_i32_or_nothing()
            .is_none());

        let valid_pos_i32 = i32::max_value() as i64;
        assert_eq!(
            Int::new(&BigNum(valid_pos_i32 as u64))
                .as_i32_or_nothing()
                .unwrap(),
            i32::max_value()
        );

        let over_neg_i32 = (i32::min_value() as i64) - 1;
        assert!(Int::new_negative(&BigNum((-over_neg_i32) as u64))
            .as_i32_or_nothing()
            .is_none());

        let valid_neg_i32 = i32::min_value() as i64;
        assert_eq!(
            Int::new_negative(&BigNum((-valid_neg_i32) as u64))
                .as_i32_or_nothing()
                .unwrap(),
            i32::min_value()
        );

        assert!(Int::new(&BigNum(u64::max_value()))
            .as_i32_or_nothing()
            .is_none());
        assert_eq!(
            Int::new(&BigNum(i32::max_value() as u64))
                .as_i32_or_nothing()
                .unwrap(),
            i32::max_value()
        );
        assert_eq!(
            Int::new_negative(&BigNum(i32::max_value() as u64))
                .as_i32_or_nothing()
                .unwrap(),
            -i32::max_value()
        );

        assert_eq!(Int::new_i32(42).as_i32_or_nothing().unwrap(), 42);
        assert_eq!(Int::new_i32(-42).as_i32_or_nothing().unwrap(), -42);
    }

    #[test]
    fn int_as_i32_or_fail() {
        let over_pos_i32 = (i32::max_value() as i64) + 1;
        assert!(Int::new(&BigNum(over_pos_i32 as u64))
            .as_i32_or_fail()
            .is_err());

        let valid_pos_i32 = i32::max_value() as i64;
        assert_eq!(
            Int::new(&BigNum(valid_pos_i32 as u64))
                .as_i32_or_fail()
                .unwrap(),
            i32::max_value()
        );

        let over_neg_i32 = (i32::min_value() as i64) - 1;
        assert!(Int::new_negative(&BigNum((-over_neg_i32) as u64))
            .as_i32_or_fail()
            .is_err());

        let valid_neg_i32 = i32::min_value() as i64;
        assert_eq!(
            Int::new_negative(&BigNum((-valid_neg_i32) as u64))
                .as_i32_or_fail()
                .unwrap(),
            i32::min_value()
        );

        assert!(Int::new(&BigNum(u64::max_value()))
            .as_i32_or_fail()
            .is_err());
        assert_eq!(
            Int::new(&BigNum(i32::max_value() as u64))
                .as_i32_or_fail()
                .unwrap(),
            i32::max_value()
        );
        assert_eq!(
            Int::new_negative(&BigNum(i32::max_value() as u64))
                .as_i32_or_fail()
                .unwrap(),
            -i32::max_value()
        );

        assert_eq!(Int::new_i32(42).as_i32_or_fail().unwrap(), 42);
        assert_eq!(Int::new_i32(-42).as_i32_or_fail().unwrap(), -42);
    }

    #[test]
    fn int_full_range() {
        // cbor_event's nint API worked via i64 but we now have a workaround for it
        // so these tests are here to make sure that workaround works.

        // first nint below of i64::MIN
        let bytes_x = vec![0x3b, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let x = Int::from_bytes(bytes_x.clone()).unwrap();
        assert_eq!(x.to_str(), "-9223372036854775809");
        assert_eq!(bytes_x, x.to_bytes());

        // smallest possible nint which is -u64::MAX - 1
        let bytes_y = vec![0x3b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let y = Int::from_bytes(bytes_y.clone()).unwrap();
        assert_eq!(y.to_str(), "-18446744073709551616");
        assert_eq!(bytes_y, y.to_bytes());
    }

    #[test]
    fn test_bigint_add() {
        assert_eq!(to_bigint(10).add(&to_bigint(20)), to_bigint(30), );
        assert_eq!(to_bigint(500).add(&to_bigint(800)), to_bigint(1300), );
    }

    #[test]
    fn test_bigint_mul() {
        assert_eq!(to_bigint(10).mul(&to_bigint(20)), to_bigint(200), );
        assert_eq!(to_bigint(500).mul(&to_bigint(800)), to_bigint(400000), );
        assert_eq!(to_bigint(12).mul(&to_bigint(22)), to_bigint(264), );
    }

    #[test]
    fn test_bigint_div_ceil() {
        assert_eq!(to_bigint(20).div_ceil(&to_bigint(10)), to_bigint(2), );
        assert_eq!(to_bigint(20).div_ceil(&to_bigint(2)), to_bigint(10), );
        assert_eq!(to_bigint(21).div_ceil(&to_bigint(2)), to_bigint(11), );
        assert_eq!(to_bigint(6).div_ceil(&to_bigint(3)), to_bigint(2), );
        assert_eq!(to_bigint(5).div_ceil(&to_bigint(3)), to_bigint(2), );
        assert_eq!(to_bigint(7).div_ceil(&to_bigint(3)), to_bigint(3), );
    }

    #[test]
    fn test_bignum_div() {
        assert_eq!(to_bignum(10).div_floor(&to_bignum(1)), to_bignum(10), );
        assert_eq!(to_bignum(10).div_floor(&to_bignum(3)), to_bignum(3), );
        assert_eq!(to_bignum(10).div_floor(&to_bignum(4)), to_bignum(2), );
        assert_eq!(to_bignum(10).div_floor(&to_bignum(5)), to_bignum(2), );
        assert_eq!(to_bignum(10).div_floor(&to_bignum(6)), to_bignum(1), );
        assert_eq!(to_bignum(10).div_floor(&to_bignum(12)), to_bignum(0), );
    }

    #[test]
    fn test_vasil_v1_costmodel_hashing() {
        let v1 = Language::new_plutus_v1();
        let v1_cost_model = TxBuilderConstants::plutus_vasil_cost_models()
            .get(&v1)
            .unwrap();
        let mut costmodels = Costmdls::new();
        costmodels.insert(&v1, &v1_cost_model);
        let hash = hash_script_data(
            &Redeemers(vec![Redeemer::new(
                &RedeemerTag::new_spend(),
                &BigNum::zero(),
                &PlutusData::new_integer(&BigInt::from_str("42").unwrap()),
                &ExUnits::new(&to_bignum(1700), &to_bignum(368100)),
            )]),
            &costmodels,
            Some(PlutusList::from(vec![PlutusData::new_integer(
                &BigInt::from_str("42").unwrap(),
            )])),
        );
        assert_eq!(
            hex::encode(hash.to_bytes()),
            "f4e4522ff98b6ba0ab5042d44da2458cd5fa6f97dc42aca1def58193f17a1375"
        );
    }

    #[test]
    fn bigint_as_int() {
        let zero = BigInt::from_str("0").unwrap();
        let zero_int = zero.as_int().unwrap();
        assert_eq!(zero_int.0, 0i128);

        let pos = BigInt::from_str("1024").unwrap();
        let pos_int = pos.as_int().unwrap();
        assert_eq!(pos_int.0, 1024i128);

        let neg = BigInt::from_str("-1024").unwrap();
        let neg_int = neg.as_int().unwrap();
        assert_eq!(neg_int.0, -1024i128);
    }
}
