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
    coin: Coin,
    multiasset: Option<MultiAsset>,
}

#[wasm_bindgen]
impl Value {
    pub fn new(coin: Coin) -> Value {
        Self {
            coin,
            multiasset: None,
        }
    }

    pub fn coin(&self) -> Coin {
        self.coin
    }

    pub fn set_coin(&mut self, coin: Coin) {
        self.coin = coin;
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
                let mut lhs_ma = lhs_ma.clone();
                for (policy, assets) in &rhs_ma.0 {
                    for (asset_name, amount) in &assets.0 {
                        match lhs_ma.0.get_mut(policy) {
                            Some(assets) => match assets.0.get_mut(asset_name) {
                                Some(current) => match current.checked_sub(&amount) {
                                    Ok(new) => *current = new,
                                    Err(_) => {
                                        assets.0.remove(asset_name);
                                    }
                                },
                                None => {
                                    return Err(JsError::from_str("underflow when substracting native asset amount"));
                                }
                            },
                            None => {
                                return Err(JsError::from_str("policy id missing from left hand side"));
                            }
                        }
                    }
                }

                Some(lhs_ma)
        },
            (Some(lhs_ma), None) => Some(lhs_ma.clone()),
            (None, Some(rhs_ma)) => Some(rhs_ma.clone()),
            (None, None) => None
        };

        Ok(Value { coin, multiasset })
    }
}

// deriving PartialOrd doesn't work in a way that's useful , as the
// implementation of PartialOrd for BTreeMap compares keys by their order,
// i.e, is equivalent to comparing the iterators of (pid, Assets).
// that would mean that: v1 < v2 if the min_pid(v1) < min_pid(v2)
// this function instead compares amounts, assuming that if a pair (pid, aname)
// is not in the Value then it has an amount of 0
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        fn for_all(ma: &MultiAsset, f: impl Fn(&PolicyID, &AssetName, &Coin) -> bool) -> bool {
            ma.0.iter()
                .all(|(pid, assets)| assets.0.iter().all(|(aname, amount)| f(pid, aname, amount)))
        }

        fn rhs_amount(rhs_ma: &Option<MultiAsset>, pid: &PolicyID, aname: &AssetName) -> Coin {
            rhs_ma
                .as_ref()
                .and_then(|rhs_ma| rhs_ma.get(&pid).and_then(|assets| assets.get(aname)))
                .unwrap_or(to_bignum(0u64))
        }

        match self.coin.cmp(&other.coin) {
            Less => {
                let le = self.multiasset.iter().all(|ma| {
                    for_all(&ma, |pid, aname, amount| {
                        amount < &rhs_amount(&other.multiasset, pid, aname)
                    })
                });

                Some(Less).filter(|_| le)
            }
            Equal => {
                let eq = self.multiasset.iter().all(|ma| {
                    for_all(&ma, |pid, aname, amount| {
                        amount == &rhs_amount(&other.multiasset, pid, aname)
                    })
                });

                Some(Equal).filter(|_| eq)
            }
            Greater => {
                let ge = self.multiasset.iter().all(|ma| {
                    for_all(&ma, |pid, aname, amount| {
                        amount > &rhs_amount(&other.multiasset, pid, aname)
                    })
                });

                Some(Greater).filter(|_| ge)
            }
        }
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
                cbor_event::Type::UnsignedInteger => Ok(Value::new(Coin::deserialize(raw)?)),
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
pub fn hash_metadata(metadata: &TransactionMetadata) -> MetadataHash {
  MetadataHash::from(blake2b256(&metadata.to_bytes()))
}
#[wasm_bindgen]
pub fn hash_transaction(tx_body: &TransactionBody) -> TransactionHash {
    TransactionHash::from(crypto::blake2b256(tx_body.to_bytes().as_ref()))
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

    withdrawal_sum
        .checked_add(&certificate_refund)
        .map(Value::new)
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
}