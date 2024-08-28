#![cfg_attr(feature = "with-bench", feature(test))]
#![allow(deprecated)]

#[macro_use]
extern crate cfg_if;

#[cfg(test)]
#[cfg(feature = "with-bench")]
extern crate test;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
extern crate hex;

#[cfg(test)]
mod tests;

#[macro_use]
extern crate num_derive;

use std::convert::TryInto;
use std::io::{BufRead, Seek, Write};

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{Len, Special as CBORSpecial};
use cbor_event::Type as CBORType;
use cbor_event::{
    self,
    de::Deserializer,
    se::{Serialize, Serializer},
};

mod builders;
pub use builders::*;
pub mod chain_core;
pub mod chain_crypto;
mod crypto;
pub(crate) use crypto::*;
mod emip3;
pub use emip3::*;
mod error;
pub use error::*;
mod fees;
pub use fees::*;
pub mod impl_mockchain;
pub mod legacy_address;
pub mod traits;
mod protocol_types;
pub use protocol_types::*;
pub mod typed_bytes;
#[macro_use]
mod utils;
pub use utils::*;
mod serialization;
mod rational;

pub use serialization::*;

use crate::traits::NoneOrEmpty;
use schemars::JsonSchema;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use hashlink::LinkedHashMap;

type DeltaCoin = Int;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Default,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct UnitInterval {
    numerator: BigNum,
    denominator: BigNum,
}

impl_to_from!(UnitInterval);

#[wasm_bindgen]
impl UnitInterval {
    pub fn numerator(&self) -> BigNum {
        self.numerator.clone()
    }

    pub fn denominator(&self) -> BigNum {
        self.denominator.clone()
    }

    pub fn new(numerator: &BigNum, denominator: &BigNum) -> Self {
        Self {
            numerator: numerator.clone(),
            denominator: denominator.clone(),
        }
    }
}

type SubCoin = UnitInterval;
type Epoch = u32;
type Slot32 = u32;
type SlotBigNum = BigNum;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Transaction {
    body: TransactionBody,
    witness_set: TransactionWitnessSet,
    is_valid: bool,
    auxiliary_data: Option<AuxiliaryData>,
}

impl_to_from!(Transaction);

#[wasm_bindgen]
impl Transaction {
    pub fn body(&self) -> TransactionBody {
        self.body.clone()
    }

    pub fn witness_set(&self) -> TransactionWitnessSet {
        self.witness_set.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid.clone()
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.auxiliary_data.clone()
    }

    pub fn set_is_valid(&mut self, valid: bool) {
        self.is_valid = valid
    }

    pub fn new(
        body: &TransactionBody,
        witness_set: &TransactionWitnessSet,
        auxiliary_data: Option<AuxiliaryData>,
    ) -> Self {
        Self {
            body: body.clone(),
            witness_set: witness_set.clone(),
            is_valid: true,
            auxiliary_data: auxiliary_data.clone(),
        }
    }
}

// index of a tx within a block
type TransactionIndex = u32;
// index of a cert within a tx
type CertificateIndex = u32;
type GovernanceActionIndex = u32;

#[wasm_bindgen]
#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct TransactionOutputs(Vec<TransactionOutput>);

impl_to_from!(TransactionOutputs);

#[wasm_bindgen]
impl TransactionOutputs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionOutput {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionOutput) {
        self.0.push(elem.clone());
    }
}

impl<'a> IntoIterator for &'a TransactionOutputs {
    type Item = &'a TransactionOutput;
    type IntoIter = std::slice::Iter<'a, TransactionOutput>;

    fn into_iter(self) -> std::slice::Iter<'a, TransactionOutput> {
        self.0.iter()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DataCost{
    coins_per_byte: Coin
}

#[wasm_bindgen]
impl DataCost {
    pub fn new_coins_per_byte(coins_per_byte: &Coin) -> DataCost {
        DataCost {
            coins_per_byte: coins_per_byte.clone()
        }
    }

    pub fn coins_per_byte(&self) -> Coin {
        self.coins_per_byte.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct TransactionOutput {
    address: Address,
    amount: Value,
    plutus_data: Option<DataOption>,
    script_ref: Option<ScriptRef>,

    #[serde(skip)]
    serialization_format: Option<CborContainerType>,
}

impl_to_from!(TransactionOutput);

#[wasm_bindgen]
impl TransactionOutput {
    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn amount(&self) -> Value {
        self.amount.clone()
    }

    pub fn data_hash(&self) -> Option<DataHash> {
        match &self.plutus_data {
            Some(DataOption::DataHash(data_hash)) => Some(data_hash.clone()),
            _ => None,
        }
    }

    pub fn plutus_data(&self) -> Option<PlutusData> {
        match &self.plutus_data {
            Some(DataOption::Data(plutus_data)) => Some(plutus_data.clone()),
            _ => None,
        }
    }

    pub fn script_ref(&self) -> Option<ScriptRef> {
        self.script_ref.clone()
    }

    pub fn set_script_ref(&mut self, script_ref: &ScriptRef) {
        self.script_ref = Some(script_ref.clone());
    }

    pub fn set_plutus_data(&mut self, data: &PlutusData) {
        self.plutus_data = Some(DataOption::Data(data.clone()));
    }

    pub fn set_data_hash(&mut self, data_hash: &DataHash) {
        self.plutus_data = Some(DataOption::DataHash(data_hash.clone()));
    }

    pub fn has_plutus_data(&self) -> bool {
        match &self.plutus_data {
            Some(DataOption::Data(_)) => true,
            _ => false,
        }
    }

    pub fn has_data_hash(&self) -> bool {
        match &self.plutus_data {
            Some(DataOption::DataHash(_)) => true,
            _ => false,
        }
    }

    pub fn has_script_ref(&self) -> bool {
        self.script_ref.is_some()
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self {
            address: address.clone(),
            amount: amount.clone(),
            plutus_data: None,
            script_ref: None,
            serialization_format: None,
        }
    }

    pub fn serialization_format(&self) -> Option<CborContainerType> {
        self.serialization_format.clone()
    }
}

impl PartialEq for TransactionOutput {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
            && self.amount == other.amount
            && self.plutus_data == other.plutus_data
            && self.script_ref == other.script_ref
    }
}

type Port = u16;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Ipv4([u8; 4]);

impl_to_from!(Ipv4);

#[wasm_bindgen]
impl Ipv4 {
    pub fn new(data: Vec<u8>) -> Result<Ipv4, JsError> {
        Self::new_impl(data).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub(crate) fn new_impl(data: Vec<u8>) -> Result<Ipv4, DeserializeError> {
        data.as_slice().try_into().map(Self).map_err(|_e| {
            let cbor_error = cbor_event::Error::WrongLen(
                4,
                cbor_event::Len::Len(data.len() as u64),
                "Ipv4 address length",
            );
            DeserializeError::new("Ipv4", DeserializeFailure::CBOR(cbor_error))
        })
    }

    pub fn ip(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Ipv6([u8; 16]);

impl_to_from!(Ipv6);

#[wasm_bindgen]
impl Ipv6 {
    pub fn new(data: Vec<u8>) -> Result<Ipv6, JsError> {
        Self::new_impl(data).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub(crate) fn new_impl(data: Vec<u8>) -> Result<Ipv6, DeserializeError> {
        data.as_slice().try_into().map(Self).map_err(|_e| {
            let cbor_error = cbor_event::Error::WrongLen(
                16,
                cbor_event::Len::Len(data.len() as u64),
                "Ipv6 address length",
            );
            DeserializeError::new("Ipv6", DeserializeFailure::CBOR(cbor_error))
        })
    }

    pub fn ip(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

static URL_MAX_LEN: usize = 128;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct URL(String);

impl_to_from!(URL);

#[wasm_bindgen]
impl URL {
    pub fn new(url: String) -> Result<URL, JsError> {
        Self::new_impl(url).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub(crate) fn new_impl(url: String) -> Result<URL, DeserializeError> {
        if url.len() <= URL_MAX_LEN {
            Ok(Self(url))
        } else {
            Err(DeserializeError::new(
                "URL",
                DeserializeFailure::OutOfRange {
                    min: 0,
                    max: URL_MAX_LEN,
                    found: url.len(),
                },
            ))
        }
    }

    pub fn url(&self) -> String {
        self.0.clone()
    }
}

static DNS_NAME_MAX_LEN: usize = 128;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct DNSRecordAorAAAA(String);

impl_to_from!(DNSRecordAorAAAA);

#[wasm_bindgen]
impl DNSRecordAorAAAA {
    pub fn new(dns_name: String) -> Result<DNSRecordAorAAAA, JsError> {
        Self::new_impl(dns_name).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub(crate) fn new_impl(dns_name: String) -> Result<DNSRecordAorAAAA, DeserializeError> {
        if dns_name.len() <= DNS_NAME_MAX_LEN {
            Ok(Self(dns_name))
        } else {
            Err(DeserializeError::new(
                "DNSRecordAorAAAA",
                DeserializeFailure::OutOfRange {
                    min: 0,
                    max: DNS_NAME_MAX_LEN,
                    found: dns_name.len(),
                },
            ))
        }
    }

    pub fn record(&self) -> String {
        self.0.clone()
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct DNSRecordSRV(String);

impl_to_from!(DNSRecordSRV);

#[wasm_bindgen]
impl DNSRecordSRV {
    pub fn new(dns_name: String) -> Result<DNSRecordSRV, JsError> {
        Self::new_impl(dns_name).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub(crate) fn new_impl(dns_name: String) -> Result<DNSRecordSRV, DeserializeError> {
        if dns_name.len() <= DNS_NAME_MAX_LEN {
            Ok(Self(dns_name))
        } else {
            Err(DeserializeError::new(
                "DNSRecordSRV",
                DeserializeFailure::OutOfRange {
                    min: 0,
                    max: DNS_NAME_MAX_LEN,
                    found: dns_name.len(),
                },
            ))
        }
    }

    pub fn record(&self) -> String {
        self.0.clone()
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct SingleHostAddr {
    port: Option<Port>,
    ipv4: Option<Ipv4>,
    ipv6: Option<Ipv6>,
}

impl_to_from!(SingleHostAddr);

#[wasm_bindgen]
impl SingleHostAddr {
    pub fn port(&self) -> Option<Port> {
        self.port.clone()
    }

    pub fn ipv4(&self) -> Option<Ipv4> {
        self.ipv4.clone()
    }

    pub fn ipv6(&self) -> Option<Ipv6> {
        self.ipv6.clone()
    }

    pub fn new(port: Option<Port>, ipv4: Option<Ipv4>, ipv6: Option<Ipv6>) -> Self {
        Self {
            port: port,
            ipv4: ipv4.clone(),
            ipv6: ipv6.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct SingleHostName {
    port: Option<Port>,
    dns_name: DNSRecordAorAAAA,
}

impl_to_from!(SingleHostName);

#[wasm_bindgen]
impl SingleHostName {
    pub fn port(&self) -> Option<Port> {
        self.port.clone()
    }

    pub fn dns_name(&self) -> DNSRecordAorAAAA {
        self.dns_name.clone()
    }

    pub fn new(port: Option<Port>, dns_name: &DNSRecordAorAAAA) -> Self {
        Self {
            port: port,
            dns_name: dns_name.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct MultiHostName {
    dns_name: DNSRecordSRV,
}

impl_to_from!(MultiHostName);

#[wasm_bindgen]
impl MultiHostName {
    pub fn dns_name(&self) -> DNSRecordSRV {
        self.dns_name.clone()
    }

    pub fn new(dns_name: &DNSRecordSRV) -> Self {
        Self {
            dns_name: dns_name.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RelayKind {
    SingleHostAddr,
    SingleHostName,
    MultiHostName,
}

#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub enum RelayEnum {
    SingleHostAddr(SingleHostAddr),
    SingleHostName(SingleHostName),
    MultiHostName(MultiHostName),
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Relay(RelayEnum);

impl_to_from!(Relay);

#[wasm_bindgen]
impl Relay {
    pub fn new_single_host_addr(single_host_addr: &SingleHostAddr) -> Self {
        Self(RelayEnum::SingleHostAddr(single_host_addr.clone()))
    }

    pub fn new_single_host_name(single_host_name: &SingleHostName) -> Self {
        Self(RelayEnum::SingleHostName(single_host_name.clone()))
    }

    pub fn new_multi_host_name(multi_host_name: &MultiHostName) -> Self {
        Self(RelayEnum::MultiHostName(multi_host_name.clone()))
    }

    pub fn kind(&self) -> RelayKind {
        match &self.0 {
            RelayEnum::SingleHostAddr(_) => RelayKind::SingleHostAddr,
            RelayEnum::SingleHostName(_) => RelayKind::SingleHostName,
            RelayEnum::MultiHostName(_) => RelayKind::MultiHostName,
        }
    }

    pub fn as_single_host_addr(&self) -> Option<SingleHostAddr> {
        match &self.0 {
            RelayEnum::SingleHostAddr(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_single_host_name(&self) -> Option<SingleHostName> {
        match &self.0 {
            RelayEnum::SingleHostName(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_multi_host_name(&self) -> Option<MultiHostName> {
        match &self.0 {
            RelayEnum::MultiHostName(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct PoolMetadata {
    url: URL,
    pool_metadata_hash: PoolMetadataHash,
}

impl_to_from!(PoolMetadata);

#[wasm_bindgen]
impl PoolMetadata {
    pub fn url(&self) -> URL {
        self.url.clone()
    }

    pub fn pool_metadata_hash(&self) -> PoolMetadataHash {
        self.pool_metadata_hash.clone()
    }

    pub fn new(url: &URL, pool_metadata_hash: &PoolMetadataHash) -> Self {
        Self {
            url: url.clone(),
            pool_metadata_hash: pool_metadata_hash.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct RewardAddresses(pub(crate) Vec<RewardAddress>);

impl_to_from!(RewardAddresses);

#[wasm_bindgen]
impl RewardAddresses {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> RewardAddress {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &RewardAddress) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Withdrawals(LinkedHashMap<RewardAddress, Coin>);

impl_to_from!(Withdrawals);

impl NoneOrEmpty for Withdrawals {
    fn is_none_or_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[wasm_bindgen]
impl Withdrawals {
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &RewardAddress, value: &Coin) -> Option<Coin> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &RewardAddress) -> Option<Coin> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> RewardAddresses {
        RewardAddresses(
            self.0
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<RewardAddress>>(),
        )
    }
}

impl serde::Serialize for Withdrawals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = self.0.iter().collect::<std::collections::BTreeMap<_, _>>();
        map.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Withdrawals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let map = <std::collections::BTreeMap<_, _> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self(map.into_iter().collect()))
    }
}

impl JsonSchema for Withdrawals {
    fn schema_name() -> String {
        String::from("Withdrawals")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        std::collections::BTreeMap::<RewardAddress, Coin>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        std::collections::BTreeMap::<RewardAddress, Coin>::is_referenceable()
    }
}

#[derive(
    Debug, Clone, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub enum DataOption {
    DataHash(DataHash),
    Data(PlutusData),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct OutputDatum(pub(crate) DataOption);

#[wasm_bindgen]
impl OutputDatum {
    pub fn new_data_hash(data_hash: &DataHash) -> Self {
        Self(DataOption::DataHash(data_hash.clone()))
    }

    pub fn new_data(data: &PlutusData) -> Self {
        Self(DataOption::Data(data.clone()))
    }

    pub fn data_hash(&self) -> Option<DataHash> {
        match &self.0 {
            DataOption::DataHash(data_hash) => Some(data_hash.clone()),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<PlutusData> {
        match &self.0 {
            DataOption::Data(data) => Some(data.clone()),
            _ => None,
        }
    }
}

/// Each new language uses a different namespace for hashing its script
/// This is because you could have a language where the same bytes have different semantics
/// So this avoids scripts in different languages mapping to the same hash
/// Note that the enum value here is different than the enum value for deciding the cost model of a script
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ScriptHashNamespace {
    NativeScript = 0,
    PlutusScript = 1,
    PlutusScriptV2 = 2,
    PlutusScriptV3 = 3,
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Update {
    proposed_protocol_parameter_updates: ProposedProtocolParameterUpdates,
    epoch: Epoch,
}

impl_to_from!(Update);

#[wasm_bindgen]
impl Update {
    pub fn proposed_protocol_parameter_updates(&self) -> ProposedProtocolParameterUpdates {
        self.proposed_protocol_parameter_updates.clone()
    }

    pub fn epoch(&self) -> Epoch {
        self.epoch.clone()
    }

    pub fn new(
        proposed_protocol_parameter_updates: &ProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self {
            proposed_protocol_parameter_updates: proposed_protocol_parameter_updates.clone(),
            epoch: epoch.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct GenesisHashes(Vec<GenesisHash>);

impl_to_from!(GenesisHashes);

#[wasm_bindgen]
impl GenesisHashes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> GenesisHash {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &GenesisHash) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ScriptHashes(pub(crate) Vec<ScriptHash>);

impl_to_from!(ScriptHashes);

#[wasm_bindgen]
impl ScriptHashes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ScriptHash {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &ScriptHash) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProposedProtocolParameterUpdates(
    LinkedHashMap<GenesisHash, ProtocolParamUpdate>,
);

impl serde::Serialize for ProposedProtocolParameterUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = self.0.iter().collect::<std::collections::BTreeMap<_, _>>();
        map.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for ProposedProtocolParameterUpdates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let map = <std::collections::BTreeMap<_, _> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self(map.into_iter().collect()))
    }
}

impl JsonSchema for ProposedProtocolParameterUpdates {
    fn schema_name() -> String {
        String::from("ProposedProtocolParameterUpdates")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        std::collections::BTreeMap::<GenesisHash, ProtocolParamUpdate>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        std::collections::BTreeMap::<GenesisHash, ProtocolParamUpdate>::is_referenceable()
    }
}

impl_to_from!(ProposedProtocolParameterUpdates);

#[wasm_bindgen]
impl ProposedProtocolParameterUpdates {
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &GenesisHash,
        value: &ProtocolParamUpdate,
    ) -> Option<ProtocolParamUpdate> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &GenesisHash) -> Option<ProtocolParamUpdate> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> GenesisHashes {
        GenesisHashes(
            self.0
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<GenesisHash>>(),
        )
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct ProtocolVersion {
    major: u32,
    minor: u32,
}

impl_to_from!(ProtocolVersion);

#[wasm_bindgen]
impl ProtocolVersion {
    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AuxiliaryDataSet(LinkedHashMap<TransactionIndex, AuxiliaryData>);

#[wasm_bindgen]
impl AuxiliaryDataSet {
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        tx_index: TransactionIndex,
        data: &AuxiliaryData,
    ) -> Option<AuxiliaryData> {
        self.0.insert(tx_index, data.clone())
    }

    pub fn get(&self, tx_index: TransactionIndex) -> Option<AuxiliaryData> {
        self.0.get(&tx_index).map(|v| v.clone())
    }

    pub fn indices(&self) -> TransactionIndexes {
        self.0
            .iter()
            .map(|(k, _v)| k.clone())
            .collect::<Vec<TransactionIndex>>()
    }
}

impl serde::Serialize for AuxiliaryDataSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = self.0.iter().collect::<std::collections::BTreeMap<_, _>>();
        map.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for AuxiliaryDataSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let map = <std::collections::BTreeMap<_, _> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self(map.into_iter().collect()))
    }
}

impl JsonSchema for AuxiliaryDataSet {
    fn schema_name() -> String {
        String::from("AuxiliaryDataSet")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        std::collections::BTreeMap::<TransactionIndex, AuxiliaryData>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        std::collections::BTreeMap::<TransactionIndex, AuxiliaryData>::is_referenceable()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AssetName(Vec<u8>);

impl Display for AssetName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

impl Ord for AssetName {
    fn cmp(&self, other: &Self) -> Ordering {
        // Implementing canonical CBOR order for asset names,
        // as they might be of different length.
        return match self.0.len().cmp(&other.0.len()) {
            Ordering::Equal => self.0.cmp(&other.0),
            x => x,
        };
    }
}

impl PartialOrd for AssetName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl_to_from!(AssetName);

#[wasm_bindgen]
impl AssetName {
    pub fn new(name: Vec<u8>) -> Result<AssetName, JsError> {
        Self::new_impl(name).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub(crate) fn new_impl(name: Vec<u8>) -> Result<AssetName, DeserializeError> {
        if name.len() <= 32 {
            Ok(Self(name))
        } else {
            Err(DeserializeError::new(
                "AssetName",
                DeserializeFailure::OutOfRange {
                    min: 0,
                    max: 32,
                    found: name.len(),
                },
            ))
        }
    }

    pub fn name(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl serde::Serialize for AssetName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

impl<'de> serde::de::Deserialize<'de> for AssetName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        if let Ok(bytes) = hex::decode(&s) {
            if let Ok(asset_name) = AssetName::new(bytes) {
                return Ok(asset_name);
            }
        }
        Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&s),
            &"AssetName as hex string e.g. F8AB28C2",
        ))
    }
}

impl JsonSchema for AssetName {
    fn schema_name() -> String {
        String::from("AssetName")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct AssetNames(Vec<AssetName>);

impl_to_from!(AssetNames);

#[wasm_bindgen]
impl AssetNames {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AssetName {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &AssetName) {
        self.0.push(elem.clone());
    }
}

pub type PolicyID = ScriptHash;
pub type PolicyIDs = ScriptHashes;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Assets(pub(crate) std::collections::BTreeMap<AssetName, BigNum>);

impl_to_from!(Assets);

#[wasm_bindgen]
impl Assets {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetName, value: &BigNum) -> Option<BigNum> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &AssetName) -> Option<BigNum> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> AssetNames {
        AssetNames(
            self.0
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<AssetName>>(),
        )
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct MultiAsset(pub(crate) std::collections::BTreeMap<PolicyID, Assets>);

impl_to_from!(MultiAsset);

#[wasm_bindgen]
impl MultiAsset {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    /// the number of unique policy IDs in the multiasset
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// set (and replace if it exists) all assets with policy {policy_id} to a copy of {assets}
    pub fn insert(&mut self, policy_id: &PolicyID, assets: &Assets) -> Option<Assets> {
        self.0.insert(policy_id.clone(), assets.clone())
    }

    /// all assets under {policy_id}, if any exist, or else None (undefined in JS)
    pub fn get(&self, policy_id: &PolicyID) -> Option<Assets> {
        self.0.get(policy_id).map(|v| v.clone())
    }

    /// sets the asset {asset_name} to {value} under policy {policy_id}
    /// returns the previous amount if it was set, or else None (undefined in JS)
    pub fn set_asset(
        &mut self,
        policy_id: &PolicyID,
        asset_name: &AssetName,
        value: &BigNum,
    ) -> Option<BigNum> {
        self.0
            .entry(policy_id.clone())
            .or_default()
            .insert(asset_name, value)
    }

    /// returns the amount of asset {asset_name} under policy {policy_id}
    /// If such an asset does not exist, 0 is returned.
    pub fn get_asset(&self, policy_id: &PolicyID, asset_name: &AssetName) -> BigNum {
        (|| self.0.get(policy_id)?.get(asset_name))().unwrap_or(BigNum::zero())
    }

    /// returns all policy IDs used by assets in this multiasset
    pub fn keys(&self) -> PolicyIDs {
        ScriptHashes(
            self.0
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<PolicyID>>(),
        )
    }

    /// removes an asset from the list if the result is 0 or less
    /// does not modify this object, instead the result is returned
    pub fn sub(&self, rhs_ma: &MultiAsset) -> MultiAsset {
        let mut lhs_ma = self.clone();
        for (policy, assets) in &rhs_ma.0 {
            for (asset_name, amount) in &assets.0 {
                match lhs_ma.0.get_mut(policy) {
                    Some(assets) => match assets.0.get_mut(asset_name) {
                        Some(current) => match current.checked_sub(&amount) {
                            Ok(new) => match new.compare(&BigNum(0)) {
                                0 => {
                                    assets.0.remove(asset_name);
                                    match assets.0.len() {
                                        0 => {
                                            lhs_ma.0.remove(policy);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => *current = new,
                            },
                            Err(_) => {
                                assets.0.remove(asset_name);
                                match assets.0.len() {
                                    0 => {
                                        lhs_ma.0.remove(policy);
                                    }
                                    _ => {}
                                }
                            }
                        },
                        None => {
                            // asset name is missing from left hand side
                        }
                    },
                    None => {
                        // policy id missing from left hand side
                    }
                }
            }
        }
        lhs_ma
    }
}

// deriving PartialOrd doesn't work in a way that's useful , as the
// implementation of PartialOrd for BTreeMap compares keys by their order,
// i.e, is equivalent to comparing the iterators of (pid, Assets).
// that would mean that: v1 < v2 if the min_pid(v1) < min_pid(v2)
// this function instead compares amounts, assuming that if a pair (pid, aname)
// is not in the MultiAsset then it has an amount of 0
impl PartialOrd for MultiAsset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn amount_or_zero(ma: &MultiAsset, pid: &PolicyID, aname: &AssetName) -> Coin {
            ma.get(&pid)
                .and_then(|assets| assets.get(aname))
                .unwrap_or(BigNum(0u64)) // assume 0 if asset not present
        }

        // idea: if (a-b) > 0 for some asset, then a > b for at least some asset
        fn is_all_zeros(lhs: &MultiAsset, rhs: &MultiAsset) -> bool {
            for (pid, assets) in lhs.0.iter() {
                for (aname, amount) in assets.0.iter() {
                    match amount
                        .clamped_sub(&amount_or_zero(&rhs, pid, aname))
                        .cmp(&BigNum::zero())
                    {
                        std::cmp::Ordering::Equal => (),
                        _ => return false,
                    }
                }
            }
            true
        }

        match (is_all_zeros(self, other), is_all_zeros(other, self)) {
            (true, true) => Some(std::cmp::Ordering::Equal),
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            (false, false) => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct MintsAssets(Vec<MintAssets>);

to_from_json!(MintsAssets);

#[wasm_bindgen]
impl MintsAssets {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, mint_assets: &MintAssets) {
        self.0.push(mint_assets.clone())
    }

    pub fn get(&self, index: usize) -> Option<MintAssets> {
        self.0.get(index).map(|v| v.clone())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct MintAssets(std::collections::BTreeMap<AssetName, Int>);

#[wasm_bindgen]
impl MintAssets {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn new_from_entry(key: &AssetName, value: &Int) -> Result<MintAssets, JsError> {
        if value.0 == 0 {
            return Err(JsError::from_str("MintAssets cannot be created with 0 value"));
        }
        let mut ma = MintAssets::new();
        ma.insert(key, value.clone())?;
        Ok(ma)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetName, value: Int) -> Result<Option<Int>, JsError> {
        if value.0 == 0 {
            return Err(JsError::from_str("MintAssets cannot be created with 0 value"));
        }
        Ok(self.0.insert(key.clone(), value))
    }

    pub(crate) fn insert_unchecked(&mut self, key: &AssetName, value: Int) -> Option<Int> {
        self.0.insert(key.clone(), value)
    }

    pub fn get(&self, key: &AssetName) -> Option<Int> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> AssetNames {
        AssetNames(
            self.0
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<AssetName>>(),
        )
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Mint(Vec<(PolicyID, MintAssets)>);

impl_to_from!(Mint);

impl NoneOrEmpty for Mint {
    fn is_none_or_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[wasm_bindgen]
impl Mint {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_from_entry(key: &PolicyID, value: &MintAssets) -> Self {
        let mut m = Mint::new();
        m.insert(key, value);
        m
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    //always returns None, because insert doesn't replace an old value
    pub fn insert(&mut self, key: &PolicyID, value: &MintAssets) -> Option<MintAssets> {
        self.0.push((key.clone(), value.clone()));
        None
    }

    pub fn get(&self, key: &PolicyID) -> Option<MintsAssets> {
        let mints: Vec<MintAssets> = self
            .0
            .iter()
            .filter(|(k, _)| k.eq(key))
            .map(|(_k, v)| v.clone())
            .collect();
        if mints.is_empty() {
            None
        } else {
            Some(MintsAssets(mints))
        }
    }

    pub fn keys(&self) -> PolicyIDs {
        ScriptHashes(
            self.0
                .iter()
                .map(|(k, _)| k.clone())
                .collect::<Vec<ScriptHash>>(),
        )
    }

    fn as_multiasset(&self, is_positive: bool) -> MultiAsset {
        self.0
            .iter()
            .fold(MultiAsset::new(), |res, e: &(PolicyID, MintAssets)| {
                let assets: Assets = (e.1).0.iter().fold(Assets::new(), |res, e| {
                    let mut assets = res;
                    if e.1.is_positive() == is_positive {
                        let amount = match is_positive {
                            true => e.1.as_positive(),
                            false => e.1.as_negative(),
                        };
                        assets.insert(&e.0, &amount.unwrap());
                    }
                    assets
                });
                let mut ma = res;
                if !assets.0.is_empty() {
                    ma.insert(&e.0, &assets);
                }
                ma
            })
    }

    /// Returns the multiasset where only positive (minting) entries are present
    pub fn as_positive_multiasset(&self) -> MultiAsset {
        self.as_multiasset(true)
    }

    /// Returns the multiasset where only negative (burning) entries are present
    pub fn as_negative_multiasset(&self) -> MultiAsset {
        self.as_multiasset(false)
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub enum NetworkIdKind {
    Testnet,
    Mainnet,
}

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct NetworkId(NetworkIdKind);

impl_to_from!(NetworkId);

#[wasm_bindgen]
impl NetworkId {
    pub fn testnet() -> Self {
        Self(NetworkIdKind::Testnet)
    }

    pub fn mainnet() -> Self {
        Self(NetworkIdKind::Mainnet)
    }

    pub fn kind(&self) -> NetworkIdKind {
        self.0
    }
}

impl From<&NativeScript> for Ed25519KeyHashes {
    fn from(script: &NativeScript) -> Self {
        match &script.0 {
            NativeScriptEnum::ScriptPubkey(spk) => {
                let mut set = Ed25519KeyHashes::new();
                set.add_move(spk.addr_keyhash());
                set
            }
            NativeScriptEnum::ScriptAll(all) => Ed25519KeyHashes::from(&all.native_scripts),
            NativeScriptEnum::ScriptAny(any) => Ed25519KeyHashes::from(&any.native_scripts),
            NativeScriptEnum::ScriptNOfK(ofk) => Ed25519KeyHashes::from(&ofk.native_scripts),
            _ => Ed25519KeyHashes::new(),
        }
    }
}