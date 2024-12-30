use cbor_event::{
    self,
    de::Deserializer,
    se::{Serialize, Serializer},
};
use hex::FromHex;
use serde_json;
use std::fmt::Display;
use std::{
    collections::HashMap,
    io::{BufRead, Seek, Write},
};

use super::*;
use crate::error::{DeserializeError, DeserializeFailure};
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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct TransactionUnspentOutputs(pub(crate) Vec<TransactionUnspentOutput>);

to_from_json!(TransactionUnspentOutputs);

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

impl<'a> IntoIterator for &'a TransactionUnspentOutputs {
    type Item = &'a TransactionUnspentOutput;
    type IntoIter = std::slice::Iter<'a, TransactionUnspentOutput>;

    fn into_iter(self) -> std::slice::Iter<'a, TransactionUnspentOutput> {
        self.0.iter()
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    /*Hash,*/ Ord,
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

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        let self_ma = self.multiasset.as_ref().map(|ma| ma.reduce_empty_to_none()).flatten();
        let other_ma = other.multiasset.as_ref().map(|ma| ma.reduce_empty_to_none()).flatten();
        self.coin == other.coin && self_ma == other_ma
    }
}

impl Eq for Value {}

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
        let multiasset = self.multiasset
            .as_ref()
            .map(|ma| ma.reduce_empty_to_none())
            .flatten();

        if let Some(multiasset) = multiasset {
            serializer.write_array(cbor_event::Len::Len(2))?;
            self.coin.serialize(serializer)?;
            multiasset.serialize(serializer)?;
        } else {
            self.coin.serialize(serializer)?;
        }

        Ok(serializer)
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

pub(crate) const BOUNDED_BYTES_CHUNK_SIZE: usize = 64;

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
                        if len as usize > BOUNDED_BYTES_CHUNK_SIZE {
                            return Err(DeserializeFailure::OutOfRange {
                                min: 0,
                                max: BOUNDED_BYTES_CHUNK_SIZE,
                                found: len as usize,
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
        ; [ A0 | datums | A0 ]
        ; corresponding to a CBOR empty map and an empty map (our apologies).
        ; Before Conway first structure was an empty list, but it was changed to empty map since Conway.
        */
        buf.push(0xA0);
        if let Some(d) = &datums {
            buf.extend(d.to_set_bytes());
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
            buf.extend(d.to_set_bytes());
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
        None => BigNum::zero(),
        Some(x) => {
            x.0.values()
                .try_fold(BigNum::zero(), |acc, ref withdrawal_amt| {
                    acc.checked_add(&withdrawal_amt)
                })?
        }
    };
    let certificate_refund = match &certs {
        None => BigNum::zero(),
        Some(certs) => certs
            .certs
            .iter()
            .try_fold(BigNum::zero(), |acc, ref cert| match &cert.0 {
                CertificateEnum::StakeDeregistration(cert) => {
                    if let Some(coin) = cert.coin {
                        acc.checked_add(&coin)
                    } else {
                        acc.checked_add(&key_deposit)
                    }
                }
                CertificateEnum::PoolRetirement(_) => acc.checked_add(&pool_deposit),
                CertificateEnum::DRepDeregistration(cert) => acc.checked_add(&cert.coin),
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
    let certificate_deposit = match &certs {
        None => BigNum::zero(),
        Some(certs) => certs
            .certs
            .iter()
            .try_fold(BigNum::zero(), |acc, ref cert| match &cert.0 {
                CertificateEnum::PoolRegistration(_) => acc.checked_add(&pool_deposit),
                CertificateEnum::StakeRegistration(cert) => {
                    if let Some(coin) = cert.coin {
                        acc.checked_add(&coin)
                    } else {
                        acc.checked_add(&key_deposit)
                    }
                }
                CertificateEnum::DRepRegistration(cert) => acc.checked_add(&cert.coin),
                CertificateEnum::StakeRegistrationAndDelegation(cert) => {
                    acc.checked_add(&cert.coin)
                }
                CertificateEnum::VoteRegistrationAndDelegation(cert) => acc.checked_add(&cert.coin),
                CertificateEnum::StakeVoteRegistrationAndDelegation(cert) => {
                    acc.checked_add(&cert.coin)
                }
                _ => Ok(acc),
            })?,
    };
    Ok(certificate_deposit)
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
        let mut output: TransactionOutput = self.output.clone();
        for _ in 0..3 {
            let required_coin = Self::calc_required_coin(&output, &self.data_cost)?;
            if output.amount.coin.less_than(&required_coin) {
                output.amount.coin = required_coin.clone();
            } else {
                return Ok(required_coin);
            }
        }
        output.amount.coin = BigNum(u64::MAX);
        Ok(Self::calc_required_coin(&output, &self.data_cost)?)
    }

    fn create_fake_output() -> Result<TransactionOutput, JsError> {
        let fake_base_address: Address = Address::from_bech32("addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w")?;
        let fake_value: Value = Value::new(&BigNum(1000000));
        Ok(TransactionOutput::new(&fake_base_address, &fake_value))
    }

    pub fn calc_size_cost(data_cost: &DataCost, size: usize) -> Result<Coin, JsError> {
        //according to https://hydra.iohk.io/build/15339994/download/1/babbage-changes.pdf
        //See on the page 9 getValue txout
        BigNum(size as u64)
            .checked_add(&BigNum(160))?
            .checked_mul(&data_cost.coins_per_byte())
    }

    pub fn calc_required_coin(
        output: &TransactionOutput,
        data_cost: &DataCost,
    ) -> Result<Coin, JsError> {
        //according to https://hydra.iohk.io/build/15339994/download/1/babbage-changes.pdf
        //See on the page 9 getValue txout
        Self::calc_size_cost(data_cost, output.to_bytes().len())
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

pub(crate) fn opt64_non_empty<T: NoneOrEmpty>(o: &Option<T>) -> u64 {
    (!o.is_none_or_empty()) as u64
}

pub struct ValueShortage {
    pub(crate) ada_shortage: Option<(Coin, Coin, Coin)>,
    pub(crate) asset_shortage: Vec<(PolicyID, AssetName, Coin, Coin)>,
}

impl Display for ValueShortage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shortage: {{")?;
        if let Some((input_data, out_data, fee)) = self.ada_shortage {
            writeln!(
                f,
                "ada in inputs: {}, ada in outputs: {}, fee {}",
                input_data, out_data, fee
            )?;
            writeln!(f, "NOTE! \"ada in inputs\" must be >= (\"ada in outputs\" + fee) before adding change")?;
            writeln!(
                f,
                "and  \"ada in inputs\" must be == (\"ada in outputs\" + fee) after adding change"
            )?;
        }
        for (policy_id, asset_name, asset_shortage, asset_available) in &self.asset_shortage {
            write!(
                f,
                "policy id: \"{}\", asset name: \"{}\" ",
                policy_id, asset_name
            )?;
            writeln!(
                f,
                "coins in inputs: {}, coins in outputs: {}",
                asset_shortage, asset_available
            )?;
        }
        write!(f, " }}")
    }
}

pub(crate) fn get_input_shortage(
    all_inputs_value: &Value,
    all_outputs_value: &Value,
    fee: &Coin,
) -> Result<Option<ValueShortage>, JsError> {
    let mut shortage = ValueShortage {
        ada_shortage: None,
        asset_shortage: Vec::new(),
    };
    if all_inputs_value.coin < all_outputs_value.coin.checked_add(fee)? {
        shortage.ada_shortage = Some((
            all_inputs_value.coin.clone(),
            all_outputs_value.coin.clone(),
            fee.clone(),
        ));
    }

    if let Some(policies) = &all_outputs_value.multiasset {
        for (policy_id, assets) in &policies.0 {
            for (asset_name, coins) in &assets.0 {
                let inputs_coins = match &all_inputs_value.multiasset {
                    Some(multiasset) => multiasset.get_asset(policy_id, asset_name),
                    None => Coin::zero(),
                };

                if inputs_coins < *coins {
                    shortage.asset_shortage.push((
                        policy_id.clone(),
                        asset_name.clone(),
                        inputs_coins,
                        coins.clone(),
                    ));
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

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionSetsState {
    AllSetsHaveTag = 0,
    AllSetsHaveNoTag = 1,
    MixedSets = 2,
}

/// Returns the state of the transaction sets.
/// If all sets have a tag, it returns AllSetsHaveTag.
/// If all sets have no tag, it returns AllSetsHaveNoTag.
/// If there is a mix of tagged and untagged sets, it returns MixedSets.
/// This function is useful for checking if a transaction might be signed by a hardware wallet.
/// And for checking which parameter should be used in a hardware wallet api.
/// WARNING this function will be deleted after all tags for set types will be mandatory. Approx after next hf
#[wasm_bindgen]
pub fn has_transaction_set_tag(tx_bytes: Vec<u8>) -> Result<TransactionSetsState, JsError> {
    let mut has_tag = false;
    let mut has_no_tag  = false;

    let tx = Transaction::from_bytes(tx_bytes)?;
    tx.witness_set.bootstraps.as_ref().map(|bs| {
        match bs.get_set_type() {
            CborSetType::Tagged => has_tag = true,
            CborSetType::Untagged => has_no_tag = true,
        }
    });
    tx.witness_set.vkeys.as_ref().map(|vkeys| {
        match vkeys.get_set_type() {
            CborSetType::Tagged => has_tag = true,
            CborSetType::Untagged => has_no_tag = true,
        }
    });
    tx.witness_set.plutus_data.as_ref().map(|plutus_data| {
        match plutus_data.get_set_type() {
            Some(CborSetType::Tagged) => has_tag = true,
            Some(CborSetType::Untagged) => has_no_tag = true,
            None => has_tag = true,
        }
    });
    tx.witness_set.native_scripts.as_ref().map(|native_scripts| {
        match native_scripts.get_set_type() {
            Some(CborSetType::Tagged) => has_tag = true,
            Some(CborSetType::Untagged) => has_no_tag = true,
            None => has_tag = true,
        }
    });
    tx.witness_set.plutus_scripts.as_ref().map(|plutus_scripts| {
        match plutus_scripts.get_set_type(&Language::new_plutus_v1()) {
            Some(CborSetType::Tagged) => has_tag = true,
            Some(CborSetType::Untagged) => has_no_tag = true,
            None => has_tag = true,
        }
        match plutus_scripts.get_set_type(&Language::new_plutus_v2()) {
            Some(CborSetType::Tagged) => has_tag = true,
            Some(CborSetType::Untagged) => has_no_tag = true,
            None => has_tag = true,
        }
        match plutus_scripts.get_set_type(&Language::new_plutus_v3()) {
            Some(CborSetType::Tagged) => has_tag = true,
            Some(CborSetType::Untagged) => has_no_tag = true,
            None => has_tag = true,
        }
    });

    match tx.body.inputs.get_set_type() {
        CborSetType::Tagged => has_tag = true,
        CborSetType::Untagged => has_no_tag = true,
    }
    tx.body.reference_inputs.as_ref().map(|ref_inputs| {
        match ref_inputs.get_set_type() {
            CborSetType::Tagged => has_tag = true,
            CborSetType::Untagged => has_no_tag = true,
        }
    });
    tx.body.required_signers.as_ref().map(|required_signers| {
        match required_signers.get_set_type() {
            CborSetType::Tagged => has_tag = true,
            CborSetType::Untagged => has_no_tag = true,
        }
    });
    tx.body.voting_proposals.as_ref().map(|voting_proposals| {
        match voting_proposals.get_set_type() {
            CborSetType::Tagged => has_tag = true,
            CborSetType::Untagged => has_no_tag = true,
        }
    });
    tx.body.collateral.as_ref().map(|collateral_inputs| {
        match collateral_inputs.get_set_type() {
            CborSetType::Tagged => has_tag = true,
            CborSetType::Untagged => has_no_tag = true,
        }
    });
    tx.body.certs.as_ref().map(|certs| {
        match certs.get_set_type() {
            CborSetType::Tagged => has_tag = true,
            CborSetType::Untagged => has_no_tag = true,
        }
    });

    tx.body.certs.as_ref().map(|certs| {
        for cert in certs {
            match &cert.0 {
                CertificateEnum::PoolRegistration(pool_reg) => {
                    match pool_reg.pool_params.pool_owners.get_set_type() {
                        CborSetType::Tagged => has_tag = true,
                        CborSetType::Untagged => has_no_tag = true,
                    }
                }
                _ => {}
            }
        }
    });

    tx.body.voting_proposals.as_ref().map(|voting_proposals| {
        for proposal in voting_proposals {
            match &proposal.governance_action.0 {
                GovernanceActionEnum::UpdateCommitteeAction(upd_action) => {
                    match upd_action.members_to_remove.get_set_type() {
                        CborSetType::Tagged => has_tag = true,
                        CborSetType::Untagged => has_no_tag = true,
                    }
                }
                _ => {}
            }
        }
    });

    match (has_tag, has_no_tag) {
        (true, true) => Ok(TransactionSetsState::MixedSets),
        (true, false) => Ok(TransactionSetsState::AllSetsHaveTag),
        (false, true) => Ok(TransactionSetsState::AllSetsHaveNoTag),
        (false, false) => Err(JsError::from_str("Transaction has invalid state")),
    }
}