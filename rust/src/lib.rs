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
#[macro_use]
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

to_from_bytes!(UnitInterval);

#[wasm_bindgen]
impl UnitInterval {
    pub fn new(index_0: u64, index_1: u64) -> Self {
        Self {
            index_0: index_0,
            index_1: index_1,
        }
    }
}

// Specifies an amount of ADA in terms of lovelace
// String functions are for environemnts that don't support u64 or BigInt/etc
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coin(u64);

to_from_bytes!(Coin);

#[wasm_bindgen]
impl Coin {
    // May not be supported in all environments as it maps to BigInt with wasm_bindgen
    pub fn new(value: u64) -> Coin {
        Self(value)
    }

    // Create a Coin from a standard rust string representation
    pub fn from_str(string: &str) -> Result<Coin, JsValue> {
        string.parse::<u64>()
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
            .map(Coin)
    }

    // String representation of the Coin value for use from environemtnst hat don't support BigInt
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    pub fn checked_add(&self, other: &Coin) -> Result<Coin, JsValue> {
        match self.0.checked_add(other.0) {
            Some(value) => Ok(Coin(value)),
            None => Err(JsValue::from_str("overflow")),
        }
    }

    pub fn checked_sub(&self, other: &Coin) -> Result<Coin, JsValue> {
        match self.0.checked_sub(other.0) {
            Some(value) => Ok(Coin(value)),
            None => Err(JsValue::from_str("underflow")),
        }
    }
}

type Epoch = u32;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Transaction {
    body: TransactionBody,
    witness_set: TransactionWitnessSet,
    metadata: Option<TransactionMetadata>,
}

to_from_bytes!(Transaction);

#[wasm_bindgen]
impl Transaction {
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

to_from_bytes!(TransactionInputs);

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

to_from_bytes!(TransactionOutputs);

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

to_from_bytes!(Certificates);

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

to_from_bytes!(TransactionBody);

#[wasm_bindgen]
impl TransactionBody {
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

    pub fn hash(&self) -> TransactionHash {
        TransactionHash::from(crypto::blake2b256(self.to_bytes().as_ref()))
    }

    pub fn sign(&self, sk: &PrivateKey) -> Vkeywitness {
        let tx_sign_data = self.hash();
        let sig = sk.sign(tx_sign_data.0.as_ref());
        Vkeywitness::new(&Vkey::new(&sk.to_public()), &sig)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionInput {
    transaction_id: TransactionHash,
    index: u32,
}

to_from_bytes!(TransactionInput);

#[wasm_bindgen]
impl TransactionInput {
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

to_from_bytes!(TransactionOutput);

#[wasm_bindgen]
impl TransactionOutput {
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

to_from_bytes!(StakeRegistration);

#[wasm_bindgen]
impl StakeRegistration {
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

to_from_bytes!(StakeDeregistration);

#[wasm_bindgen]
impl StakeDeregistration {
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

to_from_bytes!(StakeDelegation);

#[wasm_bindgen]
impl StakeDelegation {
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

to_from_bytes!(AddrKeyHashes);

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

to_from_bytes!(Relays);

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
    reward_account: RewardAddress,
    pool_owners: AddrKeyHashes,
    relays: Relays,
    pool_metadata: Option<PoolMetadata>,
}

to_from_bytes!(PoolParams);

#[wasm_bindgen]
impl PoolParams {
    pub fn new(operator: &PoolKeyHash, vrf_keyhash: &VRFKeyHash, pledge: Coin, cost: Coin, margin: &UnitInterval, reward_account: &RewardAddress, pool_owners: &AddrKeyHashes, relays: &Relays, pool_metadata: Option<PoolMetadata>) -> Self {
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

to_from_bytes!(PoolRegistration);

#[wasm_bindgen]
impl PoolRegistration {
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

to_from_bytes!(PoolRetirement);

#[wasm_bindgen]
impl PoolRetirement {
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

to_from_bytes!(GenesisKeyDelegation);

#[wasm_bindgen]
impl GenesisKeyDelegation {
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

to_from_bytes!(MoveInstantaneousRewardsCert);

#[wasm_bindgen]
impl MoveInstantaneousRewardsCert {
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

to_from_bytes!(Certificate);

#[wasm_bindgen]
impl Certificate {
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

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MIRPot {
    Reserves,
    Treasury,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoveInstantaneousReward {
    pot: MIRPot,
    rewards: std::collections::BTreeMap<StakeCredential, Coin>,
}

to_from_bytes!(MoveInstantaneousReward);

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn new(pot: MIRPot) -> Self {
        Self {
            pot,
            rewards: std::collections::BTreeMap::new()
        }
    }

    pub fn len(&self) -> usize {
        self.rewards.len()
    }

    pub fn insert(&mut self, key: &StakeCredential, value: Coin) -> Option<Coin> {
        self.rewards.insert(key.clone(), value)
    }
}

type Port = u16;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ipv4(Vec<u8>);

to_from_bytes!(Ipv4);

#[wasm_bindgen]
impl Ipv4 {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ipv6(Vec<u8>);

to_from_bytes!(Ipv6);

#[wasm_bindgen]
impl Ipv6 {
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

to_from_bytes!(SingleHostAddr);

#[wasm_bindgen]
impl SingleHostAddr {
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

to_from_bytes!(SingleHostName);

#[wasm_bindgen]
impl SingleHostName {
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

to_from_bytes!(MultiHostName);

#[wasm_bindgen]
impl MultiHostName {
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

to_from_bytes!(Relay);

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
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolMetadata {
    url: Url,
    metadata_hash: MetadataHash,
}

to_from_bytes!(PoolMetadata);

#[wasm_bindgen]
impl PoolMetadata {
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
pub struct Withdrawals(std::collections::BTreeMap<RewardAddress, Coin>);

to_from_bytes!(Withdrawals);

#[wasm_bindgen]
impl Withdrawals {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &RewardAddress, value: Coin) -> Option<Coin> {
        self.0.insert(key.clone(), value)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MultisigScripts(Vec<MultisigScript>);

to_from_bytes!(MultisigScripts);

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

to_from_bytes!(TransactionWitnessSet);

#[wasm_bindgen]
impl TransactionWitnessSet {
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

to_from_bytes!(MapTransactionMetadatumToTransactionMetadatum);

#[wasm_bindgen]
impl MapTransactionMetadatumToTransactionMetadatum {
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

to_from_bytes!(TransactionMetadatums);

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

to_from_bytes!(TransactionMetadatum);

#[wasm_bindgen]
impl TransactionMetadatum {
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

to_from_bytes!(TransactionMetadata);

#[wasm_bindgen]
impl TransactionMetadata {
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

to_from_bytes!(MsigPubkey);

#[wasm_bindgen]
impl MsigPubkey {
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

to_from_bytes!(MsigAll);

#[wasm_bindgen]
impl MsigAll {
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

to_from_bytes!(MsigAny);

#[wasm_bindgen]
impl MsigAny {
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

to_from_bytes!(MsigNOfK);

#[wasm_bindgen]
impl MsigNOfK {
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

to_from_bytes!(MultisigScript);

#[wasm_bindgen]
impl MultisigScript {
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