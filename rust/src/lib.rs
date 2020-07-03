use std::io::{BufRead, Seek, Write};
use wasm_bindgen::prelude::*;

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}};
use cbor_event::Type as CBORType;
use cbor_event::Special as CBORSpecial;

pub mod address;
pub mod crypto;
pub mod fees;
pub mod prelude;
pub mod serialization;

use address::*;
use crypto::*;
use prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnitInterval {
    index_0: u64,
    index_1: u64,
}

#[wasm_bindgen]
impl UnitInterval {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<UnitInterval, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(index_0: u64, index_1: u64) -> Self {
        Self {
            index_0: index_0,
            index_1: index_1,
        }
    }
}

type Coin = u64;

type Epoch = u32;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Transaction {
    body: TransactionBody,
    witness_set: TransactionWitnessSet,
    metadata: Option<TransactionMetadata>,
}

#[wasm_bindgen]
impl Transaction {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Transaction, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(body: &TransactionBody, witness_set: &TransactionWitnessSet, metadata: Option<TransactionMetadata>) -> Self {
        Self {
            body: body.clone(),
            witness_set: witness_set.clone(),
            metadata: metadata.clone(),
        }
    }
}

type TransactionIndex = u32;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionInputs(Vec<TransactionInput>);

#[wasm_bindgen]
impl TransactionInputs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionInput) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionOutputs(Vec<TransactionOutput>);

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

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Certificates(Vec<Certificate>);

#[wasm_bindgen]
impl Certificates {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Certificate {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Certificate) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TransactionBody {
    inputs: TransactionInputs,
    outputs: TransactionOutputs,
    fee: Coin,
    ttl: u32,
    certs: Option<Certificates>,
    withdrawals: Option<Withdrawals>,
    metadata_hash: Option<MetadataHash>,
}

#[wasm_bindgen]
impl TransactionBody {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionBody, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn set_certs(&mut self, certs: &Certificates) {
        self.certs = Some(certs.clone())
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.withdrawals = Some(withdrawals.clone())
    }

    pub fn set_metadata_hash(&mut self, metadata_hash: &MetadataHash) {
        self.metadata_hash = Some(metadata_hash.clone())
    }

    pub fn new(inputs: &TransactionInputs, outputs: &TransactionOutputs, fee: Coin, ttl: u32) -> Self {
        Self {
            inputs: inputs.clone(),
            outputs: outputs.clone(),
            fee: fee,
            ttl: ttl,
            certs: None,
            withdrawals: None,
            metadata_hash: None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionInput {
    transaction_id: TransactionHash,
    index: u32,
}

#[wasm_bindgen]
impl TransactionInput {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionInput, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(transaction_id: &TransactionHash, index: u32) -> Self {
        Self {
            transaction_id: transaction_id.clone(),
            index: index,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionOutput {
    address: Address,
    amount: Coin,
}

#[wasm_bindgen]
impl TransactionOutput {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionOutput, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(address: &Address, amount: Coin) -> Self {
        Self {
            address: address.clone(),
            amount: amount,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeRegistration {
    stake_credential: StakeCredential,
}

#[wasm_bindgen]
impl StakeRegistration {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeRegistration, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeDeregistration {
    stake_credential: StakeCredential,
}

#[wasm_bindgen]
impl StakeDeregistration {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeDeregistration, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeDelegation {
    stake_credential: StakeCredential,
    pool_keyhash: PoolKeyHash,
}

#[wasm_bindgen]
impl StakeDelegation {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeDelegation, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(stake_credential: &StakeCredential, pool_keyhash: &PoolKeyHash) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            pool_keyhash: pool_keyhash.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AddrKeyHashes(Vec<AddrKeyHash>);

#[wasm_bindgen]
impl AddrKeyHashes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AddrKeyHash {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &AddrKeyHash) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Relays(Vec<Relay>);

#[wasm_bindgen]
impl Relays {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Relay {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Relay) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolParams {
    operator: PoolKeyHash,
    vrf_keyhash: VRFKeyHash,
    pledge: Coin,
    cost: Coin,
    margin: UnitInterval,
    reward_account: RewardAccount,
    pool_owners: AddrKeyHashes,
    relays: Relays,
    pool_metadata: Option<PoolMetadata>,
}

#[wasm_bindgen]
impl PoolParams {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolParams, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(operator: &PoolKeyHash, vrf_keyhash: &VRFKeyHash, pledge: Coin, cost: Coin, margin: &UnitInterval, reward_account: &RewardAccount, pool_owners: &AddrKeyHashes, relays: &Relays, pool_metadata: Option<PoolMetadata>) -> Self {
        Self {
            operator: operator.clone(),
            vrf_keyhash: vrf_keyhash.clone(),
            pledge: pledge,
            cost: cost,
            margin: margin.clone(),
            reward_account: reward_account.clone(),
            pool_owners: pool_owners.clone(),
            relays: relays.clone(),
            pool_metadata: pool_metadata.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolRegistration {
    pool_params: PoolParams,
}

#[wasm_bindgen]
impl PoolRegistration {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolRegistration, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(pool_params: &PoolParams) -> Self {
        Self {
            pool_params: pool_params.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolRetirement {
    pool_keyhash: PoolKeyHash,
    epoch: Epoch,
}

#[wasm_bindgen]
impl PoolRetirement {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolRetirement, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(pool_keyhash: &PoolKeyHash, epoch: Epoch) -> Self {
        Self {
            pool_keyhash: pool_keyhash.clone(),
            epoch: epoch,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenesisKeyDelegation {
    genesishash: GenesisHash,
    genesis_delegate_hash: GenesisDelegateHash,
}

#[wasm_bindgen]
impl GenesisKeyDelegation {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<GenesisKeyDelegation, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(genesishash: &GenesisHash, genesis_delegate_hash: &GenesisDelegateHash) -> Self {
        Self {
            genesishash: genesishash.clone(),
            genesis_delegate_hash: genesis_delegate_hash.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoveInstantaneousRewardsCert {
    move_instantaneous_reward: MoveInstantaneousReward,
}

#[wasm_bindgen]
impl MoveInstantaneousRewardsCert {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MoveInstantaneousRewardsCert, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(move_instantaneous_reward: &MoveInstantaneousReward) -> Self {
        Self {
            move_instantaneous_reward: move_instantaneous_reward.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CertificateEnum {
    StakeRegistration(StakeRegistration),
    StakeDeregistration(StakeDeregistration),
    StakeDelegation(StakeDelegation),
    PoolRegistration(PoolRegistration),
    PoolRetirement(PoolRetirement),
    GenesisKeyDelegation(GenesisKeyDelegation),
    MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Certificate(CertificateEnum);

#[wasm_bindgen]
impl Certificate {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Certificate, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new_stake_registration(stake_registration: &StakeRegistration) -> Self {
        Self(CertificateEnum::StakeRegistration(stake_registration.clone()))
    }

    pub fn new_stake_deregistration(stake_deregistration: &StakeDeregistration) -> Self {
        Self(CertificateEnum::StakeDeregistration(stake_deregistration.clone()))
    }

    pub fn new_stake_delegation(stake_delegation: &StakeDelegation) -> Self {
        Self(CertificateEnum::StakeDelegation(stake_delegation.clone()))
    }

    pub fn new_pool_registration(pool_registration: &PoolRegistration) -> Self {
        Self(CertificateEnum::PoolRegistration(pool_registration.clone()))
    }

    pub fn new_pool_retirement(pool_retirement: &PoolRetirement) -> Self {
        Self(CertificateEnum::PoolRetirement(pool_retirement.clone()))
    }

    pub fn new_genesis_key_delegation(genesis_key_delegation: &GenesisKeyDelegation) -> Self {
        Self(CertificateEnum::GenesisKeyDelegation(genesis_key_delegation.clone()))
    }

    pub fn new_move_instantaneous_rewards_cert(move_instantaneous_rewards_cert: &MoveInstantaneousRewardsCert) -> Self {
        Self(CertificateEnum::MoveInstantaneousRewardsCert(move_instantaneous_rewards_cert.clone()))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum I0OrI1Enum {
    I0,
    I1,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct I0OrI1(I0OrI1Enum);

#[wasm_bindgen]
impl I0OrI1 {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<I0OrI1, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new_i0() -> Self {
        Self(I0OrI1Enum::I0)
    }

    pub fn new_i1() -> Self {
        Self(I0OrI1Enum::I1)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MapStakeCredentialToCoin(std::collections::BTreeMap<StakeCredential, Coin>);

#[wasm_bindgen]
impl MapStakeCredentialToCoin {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MapStakeCredentialToCoin, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &StakeCredential, value: Coin) -> Option<Coin> {
        self.0.insert(key.clone(), value)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoveInstantaneousReward {
    index_0: I0OrI1,
    index_1: MapStakeCredentialToCoin,
}

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MoveInstantaneousReward, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(index_0: &I0OrI1, index_1: &MapStakeCredentialToCoin) -> Self {
        Self {
            index_0: index_0.clone(),
            index_1: index_1.clone(),
        }
    }
}

type Port = u16;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ipv4(Vec<u8>);

#[wasm_bindgen]
impl Ipv4 {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Ipv4, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ipv6(Vec<u8>);

#[wasm_bindgen]
impl Ipv6 {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Ipv6, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

type DnsName = String;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SingleHostAddr {
    port: Option<Port>,
    ipv4: Option<Ipv4>,
    ipv6: Option<Ipv6>,
}

#[wasm_bindgen]
impl SingleHostAddr {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<SingleHostAddr, JsValue> {
        FromBytes::from_bytes(data)
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SingleHostName {
    port: Option<Port>,
    dns_name: DnsName,
}

#[wasm_bindgen]
impl SingleHostName {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<SingleHostName, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(port: Option<Port>, dns_name: DnsName) -> Self {
        Self {
            port: port,
            dns_name: dns_name,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MultiHostName {
    dns_name: DnsName,
}

#[wasm_bindgen]
impl MultiHostName {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MultiHostName, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(dns_name: DnsName) -> Self {
        Self {
            dns_name: dns_name,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RelayEnum {
    SingleHostAddr(SingleHostAddr),
    SingleHostName(SingleHostName),
    MultiHostName(MultiHostName),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Relay(RelayEnum);

#[wasm_bindgen]
impl Relay {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Relay, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new_single_host_addr(single_host_addr: &SingleHostAddr) -> Self {
        Self(RelayEnum::SingleHostAddr(single_host_addr.clone()))
    }

    pub fn new_single_host_name(single_host_name: &SingleHostName) -> Self {
        Self(RelayEnum::SingleHostName(single_host_name.clone()))
    }

    pub fn new_multi_host_name(multi_host_name: &MultiHostName) -> Self {
        Self(RelayEnum::MultiHostName(multi_host_name.clone()))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolMetadata {
    url: Url,
    metadata_hash: MetadataHash,
}

#[wasm_bindgen]
impl PoolMetadata {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolMetadata, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(url: Url, metadata_hash: &MetadataHash) -> Self {
        Self {
            url: url,
            metadata_hash: metadata_hash.clone(),
        }
    }
}

type Url = String;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Withdrawals(std::collections::BTreeMap<RewardAccount, Coin>);

#[wasm_bindgen]
impl Withdrawals {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Withdrawals, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &RewardAccount, value: Coin) -> Option<Coin> {
        self.0.insert(key.clone(), value)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MultisigScripts(Vec<MultisigScript>);

#[wasm_bindgen]
impl MultisigScripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> MultisigScript {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &MultisigScript) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TransactionWitnessSet {
    vkeys: Option<Vkeywitnesses>,
    scripts: Option<MultisigScripts>,
    bootstraps: Option<BootstrapWitnesses>,
}

#[wasm_bindgen]
impl TransactionWitnessSet {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionWitnessSet, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn set_vkeys(&mut self, vkeys: &Vkeywitnesses) {
        self.vkeys = Some(vkeys.clone())
    }

    pub fn set_scripts(&mut self, scripts: &MultisigScripts) {
        self.scripts = Some(scripts.clone())
    }

    pub fn set_bootstraps(&mut self, bootstraps: &BootstrapWitnesses) {
        self.bootstraps = Some(bootstraps.clone())
    }

    pub fn new() -> Self {
        Self {
            vkeys: None,
            scripts: None,
            bootstraps: None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MapTransactionMetadatumToTransactionMetadatum(std::collections::BTreeMap<TransactionMetadatum, TransactionMetadatum>);

#[wasm_bindgen]
impl MapTransactionMetadatumToTransactionMetadatum {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MapTransactionMetadatumToTransactionMetadatum, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &TransactionMetadatum, value: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.insert(key.clone(), value.clone())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionMetadatums(Vec<TransactionMetadatum>);

#[wasm_bindgen]
impl TransactionMetadatums {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionMetadatum {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionMetadatum) {
        self.0.push(elem.clone());
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TransactionMetadatumEnum {
    MapTransactionMetadatumToTransactionMetadatum(MapTransactionMetadatumToTransactionMetadatum),
    ArrTransactionMetadatum(TransactionMetadatums),
    Int(Int),
    Bytes(Vec<u8>),
    Text(String),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionMetadatum(TransactionMetadatumEnum);

#[wasm_bindgen]
impl TransactionMetadatum {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionMetadatum, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new_map_transaction_metadatum_to_transaction_metadatum(map_transaction_metadatum_to_transaction_metadatum: &MapTransactionMetadatumToTransactionMetadatum) -> Self {
        Self(TransactionMetadatumEnum::MapTransactionMetadatumToTransactionMetadatum(map_transaction_metadatum_to_transaction_metadatum.clone()))
    }

    pub fn new_arr_transaction_metadatum(arr_transaction_metadatum: &TransactionMetadatums) -> Self {
        Self(TransactionMetadatumEnum::ArrTransactionMetadatum(arr_transaction_metadatum.clone()))
    }

    pub fn new_int(int: &Int) -> Self {
        Self(TransactionMetadatumEnum::Int(int.clone()))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self(TransactionMetadatumEnum::Bytes(bytes))
    }

    pub fn new_text(text: String) -> Self {
        Self(TransactionMetadatumEnum::Text(text))
    }
}

type TransactionMetadadumLabel = u64;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionMetadata(std::collections::BTreeMap<TransactionMetadadumLabel, TransactionMetadatum>);

#[wasm_bindgen]
impl TransactionMetadata {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionMetadata, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: TransactionMetadadumLabel, value: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.insert(key, value.clone())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MsigPubkey {
    addr_keyhash: AddrKeyHash,
}

#[wasm_bindgen]
impl MsigPubkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MsigPubkey, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(addr_keyhash: &AddrKeyHash) -> Self {
        Self {
            addr_keyhash: addr_keyhash.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MsigAll {
    multisig_scripts: MultisigScripts,
}

#[wasm_bindgen]
impl MsigAll {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MsigAll, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(multisig_scripts: &MultisigScripts) -> Self {
        Self {
            multisig_scripts: multisig_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MsigAny {
    multisig_scripts: MultisigScripts,
}

#[wasm_bindgen]
impl MsigAny {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MsigAny, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(multisig_scripts: &MultisigScripts) -> Self {
        Self {
            multisig_scripts: multisig_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MsigNOfK {
    n: u32,
    multisig_scripts: MultisigScripts,
}

#[wasm_bindgen]
impl MsigNOfK {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MsigNOfK, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(n: u32, multisig_scripts: &MultisigScripts) -> Self {
        Self {
            n: n,
            multisig_scripts: multisig_scripts.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum MultisigScriptEnum {
    MsigPubkey(MsigPubkey),
    MsigAll(MsigAll),
    MsigAny(MsigAny),
    MsigNOfK(MsigNOfK),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MultisigScript(MultisigScriptEnum);

#[wasm_bindgen]
impl MultisigScript {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MultisigScript, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new_msig_pubkey(addr_keyhash: &AddrKeyHash) -> Self {
        Self(MultisigScriptEnum::MsigPubkey(MsigPubkey::new(addr_keyhash)))
    }

    pub fn new_msig_all(multisig_scripts: &MultisigScripts) -> Self {
        Self(MultisigScriptEnum::MsigAll(MsigAll::new(multisig_scripts)))
    }

    pub fn new_msig_any(multisig_scripts: &MultisigScripts) -> Self {
        Self(MultisigScriptEnum::MsigAny(MsigAny::new(multisig_scripts)))
    }

    pub fn new_msig_n_of_k(n: u32, multisig_scripts: &MultisigScripts) -> Self {
        Self(MultisigScriptEnum::MsigNOfK(MsigNOfK::new(n, multisig_scripts)))
    }
}