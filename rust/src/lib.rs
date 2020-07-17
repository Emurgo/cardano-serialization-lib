use std::io::{BufRead, Seek, Write};
use wasm_bindgen::prelude::*;

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}};
use cbor_event::Type as CBORType;
use cbor_event::Special as CBORSpecial;

pub mod address;
pub mod crypto;
pub mod error;
pub mod fees;
pub mod serialization;
pub mod tx_builder;
#[macro_use]
pub mod utils;

use address::*;
use crypto::*;
use error::*;
use utils::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnitInterval {
    index_0: BigNum,
    index_1: BigNum,
}

to_from_bytes!(UnitInterval);

#[wasm_bindgen]
impl UnitInterval {
    pub fn index_0(&self) -> BigNum {
        self.index_0.clone()
    }

    pub fn index_1(&self) -> BigNum {
        self.index_1.clone()
    }

    pub fn new(index_0: BigNum, index_1: BigNum) -> Self {
        Self {
            index_0: index_0,
            index_1: index_1,
        }
    }
}

type Epoch = u32;
type Slot = u32;

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
    pub fn body(&self) -> TransactionBody {
        self.body.clone()
    }

    pub fn witness_set(&self) -> TransactionWitnessSet {
        self.witness_set.clone()
    }

    pub fn metadata(&self) -> Option<TransactionMetadata> {
        self.metadata.clone()
    }

    pub fn new(body: &TransactionBody, witness_set: &TransactionWitnessSet, metadata: Option<TransactionMetadata>) -> Self {
        Self {
            body: body.clone(),
            witness_set: witness_set.clone(),
            metadata: metadata.clone(),
        }
    }
}

// index of a tx within a block
type TransactionIndex = u32;
// index of a cert within a tx
type CertificateIndex = u32;

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
#[derive(Clone, Debug)]
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
    pub fn inputs(&self) -> TransactionInputs {
        self.inputs.clone()
    }

    pub fn outputs(&self) -> TransactionOutputs {
        self.outputs.clone()
    }

    pub fn fee(&self) -> Coin {
        self.fee.clone()
    }

    pub fn ttl(&self) -> u32 {
        self.ttl.clone()
    }

    pub fn set_certs(&mut self, certs: &Certificates) {
        self.certs = Some(certs.clone())
    }

    pub fn certs(&self) -> Option<Certificates> {
        self.certs.clone()
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.withdrawals = Some(withdrawals.clone())
    }

    pub fn withdrawals(&self) -> Option<Withdrawals> {
        self.withdrawals.clone()
    }

    pub fn set_metadata_hash(&mut self, metadata_hash: &MetadataHash) {
        self.metadata_hash = Some(metadata_hash.clone())
    }

    pub fn metadata_hash(&self) -> Option<MetadataHash> {
        self.metadata_hash.clone()
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
    index: TransactionIndex,
}

to_from_bytes!(TransactionInput);

#[wasm_bindgen]
impl TransactionInput {
    pub fn transaction_id(&self) -> TransactionHash {
        self.transaction_id.clone()
    }

    pub fn index(&self) -> TransactionIndex {
        self.index.clone()
    }

    pub fn new(transaction_id: &TransactionHash, index: TransactionIndex) -> Self {
        Self {
            transaction_id: transaction_id.clone(),
            index: index,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionOutput {
    address: Address,
    amount: Coin,
}

to_from_bytes!(TransactionOutput);

#[wasm_bindgen]
impl TransactionOutput {
    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn amount(&self) -> Coin {
        self.amount.clone()
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

to_from_bytes!(StakeRegistration);

#[wasm_bindgen]
impl StakeRegistration {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
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

to_from_bytes!(StakeDeregistration);

#[wasm_bindgen]
impl StakeDeregistration {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
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

to_from_bytes!(StakeDelegation);

#[wasm_bindgen]
impl StakeDelegation {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
    }

    pub fn pool_keyhash(&self) -> PoolKeyHash {
        self.pool_keyhash.clone()
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
    pub fn operator(&self) -> PoolKeyHash {
        self.operator.clone()
    }

    pub fn vrf_keyhash(&self) -> VRFKeyHash {
        self.vrf_keyhash.clone()
    }

    pub fn pledge(&self) -> Coin {
        self.pledge.clone()
    }

    pub fn cost(&self) -> Coin {
        self.cost.clone()
    }

    pub fn margin(&self) -> UnitInterval {
        self.margin.clone()
    }

    pub fn reward_account(&self) -> RewardAddress {
        self.reward_account.clone()
    }

    pub fn pool_owners(&self) -> AddrKeyHashes {
        self.pool_owners.clone()
    }

    pub fn relays(&self) -> Relays {
        self.relays.clone()
    }

    pub fn pool_metadata(&self) -> Option<PoolMetadata> {
        self.pool_metadata.clone()
    }

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
    pub fn pool_params(&self) -> PoolParams {
        self.pool_params.clone()
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

to_from_bytes!(PoolRetirement);

#[wasm_bindgen]
impl PoolRetirement {
    pub fn pool_keyhash(&self) -> PoolKeyHash {
        self.pool_keyhash.clone()
    }

    pub fn epoch(&self) -> Epoch {
        self.epoch.clone()
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
    vrf_keyhash: VRFKeyHash,
}

to_from_bytes!(GenesisKeyDelegation);

#[wasm_bindgen]
impl GenesisKeyDelegation {
    pub fn genesishash(&self) -> GenesisHash {
        self.genesishash.clone()
    }

    pub fn genesis_delegate_hash(&self) -> GenesisDelegateHash {
        self.genesis_delegate_hash.clone()
    }

    pub fn vrf_keyhash(&self) -> VRFKeyHash {
        self.vrf_keyhash.clone()
    }

    pub fn new(genesishash: &GenesisHash, genesis_delegate_hash: &GenesisDelegateHash, vrf_keyhash: &VRFKeyHash) -> Self {
        Self {
            genesishash: genesishash.clone(),
            genesis_delegate_hash: genesis_delegate_hash.clone(),
            vrf_keyhash: vrf_keyhash.clone(),
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
    pub fn move_instantaneous_reward(&self) -> MoveInstantaneousReward {
        self.move_instantaneous_reward.clone()
    }

    pub fn new(move_instantaneous_reward: &MoveInstantaneousReward) -> Self {
        Self {
            move_instantaneous_reward: move_instantaneous_reward.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CertificateKind {
    StakeRegistration,
    StakeDeregistration,
    StakeDelegation,
    PoolRegistration,
    PoolRetirement,
    GenesisKeyDelegation,
    MoveInstantaneousRewardsCert,
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

    pub fn kind(&self) -> CertificateKind {
        match &self.0 {
            CertificateEnum::StakeRegistration(_) => CertificateKind::StakeRegistration,
            CertificateEnum::StakeDeregistration(_) => CertificateKind::StakeDeregistration,
            CertificateEnum::StakeDelegation(_) => CertificateKind::StakeDelegation,
            CertificateEnum::PoolRegistration(_) => CertificateKind::PoolRegistration,
            CertificateEnum::PoolRetirement(_) => CertificateKind::PoolRetirement,
            CertificateEnum::GenesisKeyDelegation(_) => CertificateKind::GenesisKeyDelegation,
            CertificateEnum::MoveInstantaneousRewardsCert(_) => CertificateKind::MoveInstantaneousRewardsCert,
        }
    }

    pub fn as_stake_registration(&self) -> Option<StakeRegistration> {
        match &self.0 {
            CertificateEnum::StakeRegistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            CertificateEnum::StakeDeregistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_stake_delegation(&self) -> Option<StakeDelegation> {
        match &self.0 {
            CertificateEnum::StakeDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_pool_registration(&self) -> Option<PoolRegistration> {
        match &self.0 {
            CertificateEnum::PoolRegistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_pool_retirement(&self) -> Option<PoolRetirement> {
        match &self.0 {
            CertificateEnum::PoolRetirement(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_genesis_key_delegation(&self) -> Option<GenesisKeyDelegation> {
        match &self.0 {
            CertificateEnum::GenesisKeyDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_move_instantaneous_rewards_cert(&self) -> Option<MoveInstantaneousRewardsCert> {
        match &self.0 {
            CertificateEnum::MoveInstantaneousRewardsCert(x) => Some(x.clone()),
            _ => None,
        }
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SingleHostName {
    port: Option<Port>,
    dns_name: DnsName,
}

to_from_bytes!(SingleHostName);

#[wasm_bindgen]
impl SingleHostName {
    pub fn port(&self) -> Option<Port> {
        self.port.clone()
    }

    pub fn dns_name(&self) -> DnsName {
        self.dns_name.clone()
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

to_from_bytes!(MultiHostName);

#[wasm_bindgen]
impl MultiHostName {
    pub fn dns_name(&self) -> DnsName {
        self.dns_name.clone()
    }

    pub fn new(dns_name: DnsName) -> Self {
        Self {
            dns_name: dns_name,
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolMetadata {
    url: Url,
    metadata_hash: MetadataHash,
}

to_from_bytes!(PoolMetadata);

#[wasm_bindgen]
impl PoolMetadata {
    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn metadata_hash(&self) -> MetadataHash {
        self.metadata_hash.clone()
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

    pub fn get(&self, key: &RewardAddress) -> Option<Coin> {
        self.0.get(key).map(|v| v.clone())
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

    pub fn vkeys(&self) -> Option<Vkeywitnesses> {
        self.vkeys.clone()
    }

    pub fn set_scripts(&mut self, scripts: &MultisigScripts) {
        self.scripts = Some(scripts.clone())
    }

    pub fn scripts(&self) -> Option<MultisigScripts> {
        self.scripts.clone()
    }

    pub fn set_bootstraps(&mut self, bootstraps: &BootstrapWitnesses) {
        self.bootstraps = Some(bootstraps.clone())
    }

    pub fn bootstraps(&self) -> Option<BootstrapWitnesses> {
        self.bootstraps.clone()
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

    pub fn get(&self, key: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.get(key).map(|v| v.clone())
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

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TransactionMetadatumKind {
    MapTransactionMetadatumToTransactionMetadatum,
    ArrTransactionMetadatum,
    Int,
    Bytes,
    Text,
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

    pub fn kind(&self) -> TransactionMetadatumKind {
        match &self.0 {
            TransactionMetadatumEnum::MapTransactionMetadatumToTransactionMetadatum(_) => TransactionMetadatumKind::MapTransactionMetadatumToTransactionMetadatum,
            TransactionMetadatumEnum::ArrTransactionMetadatum(_) => TransactionMetadatumKind::ArrTransactionMetadatum,
            TransactionMetadatumEnum::Int(_) => TransactionMetadatumKind::Int,
            TransactionMetadatumEnum::Bytes(_) => TransactionMetadatumKind::Bytes,
            TransactionMetadatumEnum::Text(_) => TransactionMetadatumKind::Text,
        }
    }

    pub fn as_map_transaction_metadatum_to_transaction_metadatum(&self) -> Option<MapTransactionMetadatumToTransactionMetadatum> {
        match &self.0 {
            TransactionMetadatumEnum::MapTransactionMetadatumToTransactionMetadatum(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_arr_transaction_metadatum(&self) -> Option<TransactionMetadatums> {
        match &self.0 {
            TransactionMetadatumEnum::ArrTransactionMetadatum(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        match &self.0 {
            TransactionMetadatumEnum::Int(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.0 {
            TransactionMetadatumEnum::Bytes(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<String> {
        match &self.0 {
            TransactionMetadatumEnum::Text(x) => Some(x.clone()),
            _ => None,
        }
    }
}

type TransactionMetadadumLabel = BigNum;

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

    pub fn get(&self, key: TransactionMetadadumLabel) -> Option<TransactionMetadatum> {
        self.0.get(&key).map(|v| v.clone())
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
    pub fn addr_keyhash(&self) -> AddrKeyHash {
        self.addr_keyhash.clone()
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

to_from_bytes!(MsigAll);

#[wasm_bindgen]
impl MsigAll {
    pub fn multisig_scripts(&self) -> MultisigScripts {
        self.multisig_scripts.clone()
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

to_from_bytes!(MsigAny);

#[wasm_bindgen]
impl MsigAny {
    pub fn multisig_scripts(&self) -> MultisigScripts {
        self.multisig_scripts.clone()
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

to_from_bytes!(MsigNOfK);

#[wasm_bindgen]
impl MsigNOfK {
    pub fn n(&self) -> u32 {
        self.n
    }
    pub fn multisig_scripts(&self) -> MultisigScripts {
        self.multisig_scripts.clone()
    }
    pub fn new(n: u32, multisig_scripts: &MultisigScripts) -> Self {
        Self {
            n: n,
            multisig_scripts: multisig_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MultisigScriptKind {
    MsigPubkey,
    MsigAll,
    MsigAny,
    MsigNOfK,
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

    pub fn kind(&self) -> MultisigScriptKind {
        match &self.0 {
            MultisigScriptEnum::MsigPubkey(_) => MultisigScriptKind::MsigPubkey,
            MultisigScriptEnum::MsigAll(_) => MultisigScriptKind::MsigAll,
            MultisigScriptEnum::MsigAny(_) => MultisigScriptKind::MsigAny,
            MultisigScriptEnum::MsigNOfK(_) => MultisigScriptKind::MsigNOfK,
        }
    }

    pub fn as_multisig_script0(&self) -> Option<MsigPubkey> {
        match &self.0 {
            MultisigScriptEnum::MsigPubkey(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_multisig_script1(&self) -> Option<MsigAll> {
        match &self.0 {
            MultisigScriptEnum::MsigAll(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_multisig_script2(&self) -> Option<MsigAny> {
        match &self.0 {
            MultisigScriptEnum::MsigAny(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_multisig_script3(&self) -> Option<MsigNOfK> {
        match &self.0 {
            MultisigScriptEnum::MsigNOfK(x) => Some(x.clone()),
            _ => None,
        }
    }
}