use crate::error::{DeserializeError, DeserializeFailure};
use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}};
use std::io::{BufRead, Seek, Write};
use std::cmp;
use std::ops::{Rem, Div, Sub};
use super::*;

// JsError can't be used by non-wasm targets so we use this macro to expose
// either a DeserializeError or a JsError error depending on if we're on a
// wasm or a non-wasm target where JsError is not available (it panics!).
// Note: wasm-bindgen doesn't support macros inside impls, so we have to wrap these
//       in their own impl and invoke the invoke the macro from global scope.
// TODO: possibly write s generic version of this for other usages (e.g. PrivateKey, etc)
#[macro_export]
macro_rules! from_bytes {
    // Custom from_bytes() code
    ($name:ident, $data: ident, $body:block) => {
        // wasm-exposed JsError return - JsError panics when used outside wasm
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        #[wasm_bindgen]
        impl $name {
            pub fn from_bytes($data: Vec<u8>) -> Result<$name, JsError> {
                Ok($body?)
            }
        }
        // non-wasm exposed DeserializeError return
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        impl $name {
            pub fn from_bytes($data: Vec<u8>) -> Result<$name, DeserializeError> $body
        }
    };
    // Uses Deserialize trait to auto-generate one
    ($name:ident) => {
        from_bytes!($name, bytes, {
            let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
            Self::deserialize(&mut raw)
        });
    };
}

// There's no need to do wasm vs non-wasm as this call can't fail but
// this is here just to provide a default Serialize-based impl
// Note: Once again you can't use macros in impls with wasm-bindgen
//       so make sure you invoke this outside of one
#[macro_export]
macro_rules! to_bytes {
    ($name:ident) => {
        #[wasm_bindgen]
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                let mut buf = Serializer::new_vec();
                self.serialize(&mut buf).unwrap();
                buf.finalize()
            }
        }
    }
}

#[macro_export]
macro_rules! to_from_bytes {
    ($name:ident) => {
        to_bytes!($name);
        from_bytes!($name);
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionUnspentOutput {
    input: TransactionInput,
    output: TransactionOutput
}

to_from_bytes!(TransactionUnspentOutput);

#[wasm_bindgen]
impl TransactionUnspentOutput {
    pub fn new(input: &TransactionInput, output: &TransactionOutput) -> TransactionUnspentOutput {
        Self {
            input: input.clone(),
            output: output.clone()
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
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
                    })().map_err(|e| e.annotate("input"))?;
                    let output = (|| -> Result<_, DeserializeError> {
                        Ok(TransactionOutput::deserialize(raw)?)
                    })().map_err(|e| e.annotate("output"))?;
                    let ret = Ok(Self {
                        input,
                        output
                    });
                    match len {
                        cbor_event::Len::Len(n) => match n {
                            2 => /* it's ok */(),
                            n => return Err(DeserializeFailure::DefiniteLenMismatch(n, Some(2)).into()),
                        },
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => /* it's ok */(),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    ret
                },
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("TransactionUnspentOutput"))
    }
}

// Generic u64 wrapper for platforms that don't support u64 or BigInt/etc
// This is an unsigned type - no negative numbers.
// Can be converted to/from plain rust 
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigNum(u64);

to_from_bytes!(BigNum);

#[wasm_bindgen]
impl BigNum {
    // Create a BigNum from a standard rust string representation
    pub fn from_str(string: &str) -> Result<BigNum, JsError> {
        string.parse::<u64>()
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
}

impl cbor_event::se::Serialize for BigNum {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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

pub fn to_bignum(val: u64) -> BigNum {
    BigNum(val)
}
pub fn from_bignum(val: &BigNum) -> u64 {
    val.0
}

// Specifies an amount of ADA in terms of lovelace
pub type Coin = BigNum;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, /*Hash,*/ Ord, PartialEq)]
pub struct Value {
    pub (crate) coin: Coin,
    pub (crate) multiasset: Option<MultiAsset>,
}

to_from_bytes!(Value);

#[wasm_bindgen]
impl Value {
    pub fn new(coin: &Coin) -> Value {
        Self {
            coin: coin.clone(),
            multiasset: None,
        }
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
            },
            (None, None) => None, 
            (Some(ma), None) => Some(ma.clone()),
            (None, Some(ma)) => Some(ma.clone()),
        };

        Ok(Value {
            coin, 
            multiasset
        })
    }

    pub fn checked_sub(&self, rhs_value: &Value) -> Result<Value, JsError> {
        let coin = self.coin.checked_sub(&rhs_value.coin)?;
        let multiasset = match(&self.multiasset, &rhs_value.multiasset) {
            (Some(lhs_ma), Some(rhs_ma)) => {
                match (lhs_ma.sub(rhs_ma).len()) {
                    0 => None,
                    _ => Some(lhs_ma.sub(rhs_ma))
                }
            },
            (Some(lhs_ma), None) => Some(lhs_ma.clone()),
            (None, Some(_rhs_ma)) => None,
            (None, None) => None
        };

        Ok(Value { coin, multiasset })
    }

    pub fn clamped_sub(&self, rhs_value: &Value) -> Value {
        let coin = self.coin.clamped_sub(&rhs_value.coin);
        let multiasset = match(&self.multiasset, &rhs_value.multiasset) {
            (Some(lhs_ma), Some(rhs_ma)) => {
                match (lhs_ma.sub(rhs_ma).len()) {
                    0 => None,
                    _ => Some(lhs_ma.sub(rhs_ma))
                }
            },
            (Some(lhs_ma), None) => Some(lhs_ma.clone()),
            (None, Some(_rhs_ma)) => None,
            (None, None) => None
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

        fn compare_assets(lhs: &Option<MultiAsset>, rhs: &Option<MultiAsset>) -> Option<std::cmp::Ordering> {
            match (lhs, rhs) {
                (None, None) => Some(Equal),
                (None, Some(rhs_assets)) => MultiAsset::new().partial_cmp(&rhs_assets),
                (Some(lhs_assets), None) => lhs_assets.partial_cmp(&MultiAsset::new()),
                (Some(lhs_assets), Some(rhs_assets)) => lhs_assets.partial_cmp(&rhs_assets),
            }
        }

        compare_assets(&self.multiasset(), &other.multiasset())
            .and_then(|assets_match| {
                let coin_cmp = self.coin.cmp(&other.coin);

                match (coin_cmp, assets_match) {
                    (coin_order, Equal) => Some(coin_order),
                    (Equal, Less) => Some(Less),
                    (Less, Less) => Some(Less),
                    (Equal, Greater) => Some(Greater),
                    (Greater, Greater) => Some(Greater),
                    (_, _) => None
                }
            })
    }
}

impl cbor_event::se::Serialize for Value {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match &self.multiasset {
            Some(multiasset) => {
                serializer.write_array(cbor_event::Len::Len(2))?;
                self.coin.serialize(serializer)?;
                multiasset.serialize(serializer)
            },
            None => self.coin.serialize(serializer)
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
                    let coin = (|| -> Result<_, DeserializeError> {
                        Ok(Coin::deserialize(raw)?)
                    })().map_err(|e| e.annotate("coin"))?;
                    let multiasset = (|| -> Result<_, DeserializeError> {
                        Ok(MultiAsset::deserialize(raw)?)
                    })().map_err(|e| e.annotate("multiasset"))?;
                    let ret = Ok(Self {
                        coin,
                        multiasset: Some(multiasset),
                    });
                    match len {
                        cbor_event::Len::Len(n) => match n {
                            2 => /* it's ok */(),
                            n => return Err(DeserializeFailure::DefiniteLenMismatch(n, Some(2)).into()),
                        },
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => /* it's ok */(),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    ret
                },
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("Value"))
    }
}

// CBOR has int = uint / nint
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int(pub (crate) i128);

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
        return self.0 >= 0
    }

    pub fn as_positive(&self) -> Option<BigNum> {
        if self.is_positive() {
            Some(to_bignum(self.0 as u64))
        } else {
            None
        }
    }

    pub fn as_negative(&self) -> Option<BigNum> {
        if !self.is_positive() {
            Some(to_bignum((-self.0) as u64))
        } else {
            None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        use std::convert::TryFrom;
        i32::try_from(self.0).ok()
    }
}

impl cbor_event::se::Serialize for Int {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
                cbor_event::Type::NegativeInteger => Ok(Self(raw.negative_integer()? as i128)),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("Int"))
    }
}

const BOUNDED_BYTES_CHUNK_SIZE: usize = 64;

pub (crate) fn write_bounded_bytes<'se, W: Write>(serializer: &'se mut Serializer<W>, bytes: &[u8]) -> cbor_event::Result<&'se mut Serializer<W>> {
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

pub (crate) fn read_bounded_bytes<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Vec<u8>, DeserializeError> {
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
                return Err(DeserializeFailure::OutOfRange{
                    min: 0,
                    max: BOUNDED_BYTES_CHUNK_SIZE,
                    found: bytes.len(),
                }.into());
            }
            Ok(bytes)
        },
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
                    cbor_event::Len::Indefinite => return Err(cbor_event::Error::CustomError(String::from("Illegal CBOR: Indefinite string found inside indefinite string")).into()),
                    cbor_event::Len::Len(len) => {
                        if chunk_len_sz > BOUNDED_BYTES_CHUNK_SIZE {
                            return Err(DeserializeFailure::OutOfRange{
                                min: 0,
                                max: BOUNDED_BYTES_CHUNK_SIZE,
                                found: chunk_len_sz,
                            }.into());
                        }
                        raw.advance(1 + chunk_len_sz)?;
                        raw
                            .as_mut_ref()
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
        },
    }

}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BigInt(num_bigint::BigInt);

to_from_bytes!(BigInt);

#[wasm_bindgen]
impl BigInt {
    pub fn as_u64(&self) -> Option<BigNum> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        if sign == num_bigint::Sign::Minus {
            return None;
        }
        match u64_digits.len() {
            1 => Some(to_bignum(*u64_digits.first().unwrap())),
            _ => None,
        }
    }

    pub fn from_str(text: &str) -> Result<BigInt, JsError> {
        use std::str::FromStr;
        num_bigint::BigInt::from_str(text)
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            .map(BigInt)
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }
}

impl cbor_event::se::Serialize for BigInt {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        // we use the uint/nint encodings to use a minimum of space
        if u64_digits.len() == 1 {
            match sign {
                // uint
                num_bigint::Sign::Plus |
                num_bigint::Sign::NoSign => serializer.write_unsigned_integer(*u64_digits.first().unwrap())?,
                // nint
                num_bigint::Sign::Minus => serializer.write_negative_integer(-(*u64_digits.first().unwrap() as i128) as i64)?,
            };
        } else {
            let (sign, bytes) = self.0.to_bytes_be();
            match sign {
                // positive bigint
                num_bigint::Sign::Plus |
                num_bigint::Sign::NoSign => {
                    serializer.write_tag(2u64)?;
                    write_bounded_bytes(serializer, &bytes)?;
                },
                // negative bigint
                num_bigint::Sign::Minus => {
                    serializer.write_tag(3u64)?;
                    use std::ops::Neg;
                    // CBOR RFC defines this as the bytes of -n -1
                    let adjusted = self.0.clone().neg().checked_sub(&num_bigint::BigInt::from(1u32)).unwrap().to_biguint().unwrap();
                    write_bounded_bytes(serializer, &adjusted.to_bytes_be())?;
                },
            }
        }
        Ok(serializer)
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
                        2 => Ok(Self(num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes))),
                        // negative bigint
                        3 => {
                            // CBOR RFC defines this as the bytes of -n -1
                            let initial = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);
                            use std::ops::Neg;
                            let adjusted = initial.checked_add(&num_bigint::BigInt::from(1u32)).unwrap().neg();
                            Ok(Self(adjusted))
                        },
                        _ => return Err(DeserializeFailure::TagMismatch{ found: tag, expected: 2 }.into()),
                    }
                },
                // uint
                CBORType::UnsignedInteger => Ok(Self(num_bigint::BigInt::from(raw.unsigned_integer()?))),
                // nint
                CBORType::NegativeInteger => Ok(Self(num_bigint::BigInt::from(raw.negative_integer()?))),
                _ => return Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("BigInt"))
    }
}

// we use the cbor_event::Serialize trait directly

// This is only for use for plain cddl groups who need to be embedded within outer groups.
pub (crate) trait SerializeEmbeddedGroup {
    fn serialize_as_embedded_group<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

// same as cbor_event::de::Deserialize but with our DeserializeError
pub trait Deserialize {
    fn deserialize<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> where Self: Sized;
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
    ) -> Result<Self, DeserializeError> where Self: Sized;
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
            },
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
            },
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
    let signature = Ed25519Signature::from_bytes(key.0.sign(&tx_body_hash.to_bytes()).as_ref().to_vec()).unwrap();

    BootstrapWitness::new(
        &vkey,
        &signature,
        chain_code,
        addr.attributes(),
    )
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

    BootstrapWitness::new(
        &vkey,
        &signature,
        chain_code,
        addr.attributes(),
    )
}

#[wasm_bindgen]
pub fn make_vkey_witness(
    tx_body_hash: &TransactionHash,
    sk: &PrivateKey
) -> Vkeywitness {
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
pub fn hash_script_data(redeemers: &Redeemers, cost_models: &Costmdls, datums: Option<PlutusList>) -> ScriptDataHash {
    /*
    ; script data format:
    ; [ redeemers | datums | language views ]
    ; The redeemers are exactly the data present in the transaction witness set.
    ; Similarly for the datums, if present. If no datums are provided, the middle
    ; field is an empty string.
    */
    let mut buf = Vec::new();
    buf.extend(redeemers.to_bytes());
     if let Some(d) = &datums {
        buf.extend(d.to_bytes());
    } else {
        let empty_string = "";
        buf.extend(empty_string.as_bytes());
    }
    buf.extend(cost_models.to_bytes());
    ScriptDataHash::from(blake2b256(&buf))
}

// wasm-bindgen can't accept Option without clearing memory, so we avoid exposing this in WASM
pub fn internal_get_implicit_input(
    withdrawals: &Option<Withdrawals>,
    certs: &Option<Certificates>,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Value, JsError> {
    let withdrawal_sum = match &withdrawals {
        None => to_bignum(0),
        Some(x) => x.0
            .values()
            .try_fold(
                to_bignum(0),
                |acc, ref withdrawal_amt| acc.checked_add(&withdrawal_amt)
            )?,
    };
    let certificate_refund = match &certs {
        None => to_bignum(0),
        Some(certs) => certs.0
            .iter()
            .try_fold(
                to_bignum(0),
                |acc, ref cert| match &cert.0 {
                    CertificateEnum::PoolRetirement(_cert) => acc.checked_add(&pool_deposit),
                    CertificateEnum::StakeDeregistration(_cert) => acc.checked_add(&key_deposit),
                    _ => Ok(acc),
                }
            )?
    };

    Ok(Value::new(&withdrawal_sum.checked_add(&certificate_refund)?))
}
pub fn internal_get_deposit(
    certs: &Option<Certificates>,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Coin, JsError> {
    let certificate_refund = match &certs {
        None => to_bignum(0),
        Some(certs) => certs.0
            .iter()
            .try_fold(
                to_bignum(0),
                |acc, ref cert| match &cert.0 {
                    CertificateEnum::PoolRegistration(_cert) => acc.checked_add(&pool_deposit),
                    CertificateEnum::StakeRegistration(_cert) => acc.checked_add(&key_deposit),
                    _ => Ok(acc),
                }
            )?
    };
    Ok(certificate_refund)
}


#[wasm_bindgen]
pub fn get_implicit_input(
    txbody: &TransactionBody,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
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
    key_deposit: &BigNum, // protocol parameter
) -> Result<Coin, JsError> {
    internal_get_deposit(
        &txbody.certs,
        &pool_deposit,
        &key_deposit,
    )
}

struct OutputSizeConstants {
    k0: usize,
    k1: usize,
    k2: usize,
}

fn quot<T>(a: T, b: T) -> T
where T: Sub<Output=T> + Rem<Output=T> + Div<Output=T> + Copy + Clone + std::fmt::Display {
    (a - (a % b)) / b
}

fn bundle_size(
    assets: &Value,
    constants: &OutputSizeConstants,
) -> usize {
    // based on https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo.rst
    match &assets.multiasset {
        None => 1, // Haskell codebase considers these size 1
        Some (assets) => {
            let num_assets = assets.0
                .values()
                .fold(
                    0,
                    | acc, next| acc + next.len()
                );
            let sum_asset_name_lengths = assets.0
                .values()
                .flat_map(|assets| assets.0.keys())
                .fold(
                    0,
                    | acc, next| acc + next.0.len()
                );
            let sum_policy_id_lengths = assets.0
                .keys()
                .fold(
                    0,
                    | acc, next| acc + next.0.len()
                );
            // converts bytes to 8-byte long words, rounding up
            fn roundup_bytes_to_words(b: usize) -> usize {
                quot(b + 7, 8)
            };
            constants.k0 + roundup_bytes_to_words(
                (num_assets * constants.k1) + sum_asset_name_lengths +
                (constants.k2 * sum_policy_id_lengths)
            )
        }
    }
}

#[wasm_bindgen]
pub fn min_ada_required(
    assets: &Value,
    minimum_utxo_val: &BigNum, // protocol parameter
) -> BigNum {
    // based on https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo.rst
    match &assets.multiasset {
        None => minimum_utxo_val.clone(),
        Some(_assets) => {
            // NOTE: should be 2, but a bug in Haskell set this to 0
            let coin_size: u64 = 0;
            let tx_out_len_no_val = 14;
            let tx_in_len = 7;
            let utxo_entry_size_without_val: u64 = 6 + tx_out_len_no_val + tx_in_len; // 27

            // NOTE: should be 29 but a bug in Haskell set this to 27
            let ada_only_utxo_size: u64 = utxo_entry_size_without_val + coin_size;

            let size = bundle_size(
                &assets,
                &OutputSizeConstants {
                    k0: 6,
                    k1: 12,
                    k2: 1,
                },
            );
            BigNum(cmp::max(
                minimum_utxo_val.0,
                quot(minimum_utxo_val.0, ada_only_utxo_size) * (utxo_entry_size_without_val + (size as u64))
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // this is what is used in mainnet
    static MINIMUM_UTXO_VAL: u64 = 1_000_000;

    #[test]
    fn no_token_minimum() {
        
        let assets = Value {
            coin: BigNum(0),
            multiasset: None,
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            MINIMUM_UTXO_VAL
        );
    }

    #[test]
    fn one_policy_one_smallest_name() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1407406),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1407406
        );
    }

    #[test]
    fn one_policy_one_small_name() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![1]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1444443),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1444443
        );
    }

    #[test]
    fn one_policy_one_largest_name() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            // The largest asset names have length thirty-two
            &AssetName([1; 32].to_vec()),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1555554
        );
    }

    #[test]
    fn one_policy_three_small_names() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![1]),
            &BigNum(1)
        );
        asset_list.insert(
            &AssetName(vec![2]),
            &BigNum(1)
        );
        asset_list.insert(
            &AssetName(vec![3]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1555554
        );
    }

    #[test]
    fn one_policy_three_largest_names() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            // The largest asset names have length thirty-two
            &AssetName([1; 32].to_vec()),
            &BigNum(1)
        );
        asset_list.insert(
            // The largest asset names have length thirty-two
            &AssetName([2; 32].to_vec()),
            &BigNum(1)
        );
        asset_list.insert(
            // The largest asset names have length thirty-two
            &AssetName([3; 32].to_vec()),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1962961),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1962961
        );
    }

    #[test]
    fn two_policies_one_smallest_name() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        token_bundle.insert(
            &PolicyID::from([1; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1592591),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1592591
        );
    }

    #[test]
    fn two_policies_two_small_names() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        token_bundle.insert(
            &PolicyID::from([1; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1592591),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1592591
        );
    }

    #[test]
    fn three_policies_99_small_names() {
        
        let mut token_bundle = MultiAsset::new();
        fn add_policy(token_bundle: &mut MultiAsset, index: u8) -> () {
            let mut asset_list = Assets::new();

            for i in 0..33 {
                asset_list.insert(
                    &AssetName(vec![i]),
                    &BigNum(1)
                );
            }
            token_bundle.insert(
                &PolicyID::from([index; ScriptHash::BYTE_COUNT]),
                &asset_list
            );
        }
        add_policy(&mut token_bundle, 1);
        add_policy(&mut token_bundle, 2);
        add_policy(&mut token_bundle, 3);
        let assets = Value {
            coin: BigNum(7592585),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            7592585
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
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            asset_list1.insert(
                &asset2,
                &BigNum(1)
            );
            asset_list1.insert(
                &asset3,
                &BigNum(1)
            );
            asset_list1.insert(
                &asset4,
                &BigNum(2)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );

            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy2,
                &asset_list2
            );
        }
        let assets1 = Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        {
            let mut asset_list2 = Assets::new();
            // more than asset1 bundle
            asset_list2.insert(
                &asset1,
                &BigNum(2)
            );
            // exactly equal to asset1 bundle
            asset_list2.insert(
                &asset2,
                &BigNum(1)
            );
            // skip asset 3
            // less than in asset1 bundle
            asset_list2.insert(
                &asset4,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );

            // this policy should be removed entirely
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy2,
                &asset_list2
            );
        }

        let assets2 = Value {
            coin: BigNum(2555554),
            multiasset: Some(token_bundle2),
        };

        let result = assets1.clamped_sub(&assets2);
        assert_eq!(
            result.coin().to_str(),
            "0"
        );
        assert_eq!(
            result.multiasset().unwrap().len(),
            1 // policy 2 was deleted successfully
        );
        let policy1_content = result.multiasset().unwrap().get(&policy1).unwrap();
        assert_eq!(
            policy1_content.len(),
            2
        );
        assert_eq!(
            policy1_content.get(&asset3).unwrap().to_str(),
            "1"
        );
        assert_eq!(
            policy1_content.get(&asset4).unwrap().to_str(),
            "1"
        );
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
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
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
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
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
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Equal);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b), None);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b), None);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset2,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
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
        assert_eq!(hex::decode("c349010000000000000000").unwrap(), BigInt::from_str("-18446744073709551617").unwrap().to_bytes());
        // positive big int
        assert_eq!(hex::decode("c249010000000000000000").unwrap(), BigInt::from_str("18446744073709551616").unwrap().to_bytes());
        // uint
        assert_eq!(hex::decode("1b000000e8d4a51000").unwrap(), BigInt::from_str("1000000000000").unwrap().to_bytes());
        // nint
        // we can't use this due to cbor_event actually not supporting the full NINT spectrum as it uses an i64 for some reason...
        //assert_eq!(hex::decode("3bffffffffffffffff").unwrap(), BigInt::from_str("-18446744073709551616").unwrap().to_bytes());
        // this one fits in an i64 though
        assert_eq!(hex::decode("3903e7").unwrap(), BigInt::from_str("-1000").unwrap().to_bytes());


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
        let chunks = vec![
            chunk_64,
            vec![0x44, 0x01, 0x02, 0x03, 0x04],
        ];
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
}
