#![cfg_attr(feature = "with-bench", feature(test))]

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


use std::io::{BufRead, Seek, Write};
use wasm_bindgen::prelude::*;

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::Special as CBORSpecial;
use cbor_event::Type as CBORType;
use cbor_event::{
    self,
    de::Deserializer,
    se::{Serialize, Serializer},
};

pub mod address;
pub mod chain_core;
pub mod chain_crypto;
pub mod crypto;
pub mod error;
pub mod fees;
pub mod impl_mockchain;
pub mod legacy_address;
pub mod metadata;
pub mod serialization;
pub mod tx_builder;
pub mod typed_bytes;
#[macro_use]
pub mod utils;

use address::*;
use crypto::*;
use error::*;
use metadata::*;
use utils::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnitInterval {
    numerator: BigNum,
    denominator: BigNum,
}

to_from_bytes!(UnitInterval);

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

type Rational = UnitInterval;
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

    pub fn new(
        body: &TransactionBody,
        witness_set: &TransactionWitnessSet,
        metadata: Option<TransactionMetadata>,
    ) -> Self {
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
    update: Option<Update>,
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

    pub fn set_update(&mut self, update: &Update) {
        self.update = Some(update.clone())
    }

    pub fn update(&self) -> Option<Update> {
        self.update.clone()
    }
    pub fn set_metadata_hash(&mut self, metadata_hash: &MetadataHash) {
        self.metadata_hash = Some(metadata_hash.clone())
    }

    pub fn metadata_hash(&self) -> Option<MetadataHash> {
        self.metadata_hash.clone()
    }

    pub fn new(
        inputs: &TransactionInputs,
        outputs: &TransactionOutputs,
        fee: &Coin,
        ttl: u32,
    ) -> Self {
        Self {
            inputs: inputs.clone(),
            outputs: outputs.clone(),
            fee: fee.clone(),
            ttl: ttl,
            certs: None,
            withdrawals: None,
            update: None,
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

    pub fn new(address: &Address, amount: &Coin) -> Self {
        Self {
            address: address.clone(),
            amount: amount.clone(),
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
    pool_keyhash: Ed25519KeyHash,
}

to_from_bytes!(StakeDelegation);

#[wasm_bindgen]
impl StakeDelegation {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
    }

    pub fn pool_keyhash(&self) -> Ed25519KeyHash {
        self.pool_keyhash.clone()
    }

    pub fn new(stake_credential: &StakeCredential, pool_keyhash: &Ed25519KeyHash) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            pool_keyhash: pool_keyhash.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ed25519KeyHashes(Vec<Ed25519KeyHash>);

to_from_bytes!(Ed25519KeyHashes);

#[wasm_bindgen]
impl Ed25519KeyHashes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) {
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
    operator: Ed25519KeyHash,
    vrf_keyhash: VRFKeyHash,
    pledge: Coin,
    cost: Coin,
    margin: UnitInterval,
    reward_account: RewardAddress,
    pool_owners: Ed25519KeyHashes,
    relays: Relays,
    pool_metadata: Option<PoolMetadata>,
}

to_from_bytes!(PoolParams);

#[wasm_bindgen]
impl PoolParams {
    pub fn operator(&self) -> Ed25519KeyHash {
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

    pub fn pool_owners(&self) -> Ed25519KeyHashes {
        self.pool_owners.clone()
    }

    pub fn relays(&self) -> Relays {
        self.relays.clone()
    }

    pub fn pool_metadata(&self) -> Option<PoolMetadata> {
        self.pool_metadata.clone()
    }

    pub fn new(
        operator: &Ed25519KeyHash,
        vrf_keyhash: &VRFKeyHash,
        pledge: &Coin,
        cost: &Coin,
        margin: &UnitInterval,
        reward_account: &RewardAddress,
        pool_owners: &Ed25519KeyHashes,
        relays: &Relays,
        pool_metadata: Option<PoolMetadata>,
    ) -> Self {
        Self {
            operator: operator.clone(),
            vrf_keyhash: vrf_keyhash.clone(),
            pledge: pledge.clone(),
            cost: cost.clone(),
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
    pool_keyhash: Ed25519KeyHash,
    epoch: Epoch,
}

to_from_bytes!(PoolRetirement);

#[wasm_bindgen]
impl PoolRetirement {
    pub fn pool_keyhash(&self) -> Ed25519KeyHash {
        self.pool_keyhash.clone()
    }

    pub fn epoch(&self) -> Epoch {
        self.epoch.clone()
    }

    pub fn new(pool_keyhash: &Ed25519KeyHash, epoch: Epoch) -> Self {
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

    pub fn new(
        genesishash: &GenesisHash,
        genesis_delegate_hash: &GenesisDelegateHash,
        vrf_keyhash: &VRFKeyHash,
    ) -> Self {
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
        Self(CertificateEnum::StakeRegistration(
            stake_registration.clone(),
        ))
    }

    pub fn new_stake_deregistration(stake_deregistration: &StakeDeregistration) -> Self {
        Self(CertificateEnum::StakeDeregistration(
            stake_deregistration.clone(),
        ))
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
        Self(CertificateEnum::GenesisKeyDelegation(
            genesis_key_delegation.clone(),
        ))
    }

    pub fn new_move_instantaneous_rewards_cert(
        move_instantaneous_rewards_cert: &MoveInstantaneousRewardsCert,
    ) -> Self {
        Self(CertificateEnum::MoveInstantaneousRewardsCert(
            move_instantaneous_rewards_cert.clone(),
        ))
    }

    pub fn kind(&self) -> CertificateKind {
        match &self.0 {
            CertificateEnum::StakeRegistration(_) => CertificateKind::StakeRegistration,
            CertificateEnum::StakeDeregistration(_) => CertificateKind::StakeDeregistration,
            CertificateEnum::StakeDelegation(_) => CertificateKind::StakeDelegation,
            CertificateEnum::PoolRegistration(_) => CertificateKind::PoolRegistration,
            CertificateEnum::PoolRetirement(_) => CertificateKind::PoolRetirement,
            CertificateEnum::GenesisKeyDelegation(_) => CertificateKind::GenesisKeyDelegation,
            CertificateEnum::MoveInstantaneousRewardsCert(_) => {
                CertificateKind::MoveInstantaneousRewardsCert
            }
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
    rewards: linked_hash_map::LinkedHashMap<StakeCredential, Coin>,
}

to_from_bytes!(MoveInstantaneousReward);

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn new(pot: MIRPot) -> Self {
        Self {
            pot,
            rewards: linked_hash_map::LinkedHashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.rewards.len()
    }

    pub fn insert(&mut self, key: &StakeCredential, value: &Coin) -> Option<Coin> {
        self.rewards.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &StakeCredential) -> Option<Coin> {
        self.rewards.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> StakeCredentials {
        StakeCredentials(
            self.rewards
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<StakeCredential>>(),
        )
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
        Self { dns_name: dns_name }
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
pub struct StakeCredentials(Vec<StakeCredential>);

to_from_bytes!(StakeCredentials);

#[wasm_bindgen]
impl StakeCredentials {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> StakeCredential {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &StakeCredential) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RewardAddresses(Vec<RewardAddress>);

to_from_bytes!(RewardAddresses);

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
pub struct Withdrawals(linked_hash_map::LinkedHashMap<RewardAddress, Coin>);

to_from_bytes!(Withdrawals);

#[wasm_bindgen]
impl Withdrawals {
    pub fn new() -> Self {
        Self(linked_hash_map::LinkedHashMap::new())
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
pub struct MsigPubkey {
    addr_keyhash: Ed25519KeyHash,
}

to_from_bytes!(MsigPubkey);

#[wasm_bindgen]
impl MsigPubkey {
    pub fn addr_keyhash(&self) -> Ed25519KeyHash {
        self.addr_keyhash.clone()
    }
    pub fn new(addr_keyhash: &Ed25519KeyHash) -> Self {
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
    pub fn new_msig_pubkey(addr_keyhash: &Ed25519KeyHash) -> Self {
        Self(MultisigScriptEnum::MsigPubkey(MsigPubkey::new(
            addr_keyhash,
        )))
    }

    pub fn new_msig_all(multisig_scripts: &MultisigScripts) -> Self {
        Self(MultisigScriptEnum::MsigAll(MsigAll::new(multisig_scripts)))
    }

    pub fn new_msig_any(multisig_scripts: &MultisigScripts) -> Self {
        Self(MultisigScriptEnum::MsigAny(MsigAny::new(multisig_scripts)))
    }

    pub fn new_msig_n_of_k(n: u32, multisig_scripts: &MultisigScripts) -> Self {
        Self(MultisigScriptEnum::MsigNOfK(MsigNOfK::new(
            n,
            multisig_scripts,
        )))
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

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Update {
    proposed_protocol_parameter_updates: ProposedProtocolParameterUpdates,
    epoch: Epoch,
}

to_from_bytes!(Update);

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenesisHashes(Vec<GenesisHash>);

to_from_bytes!(GenesisHashes);

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProposedProtocolParameterUpdates(
    linked_hash_map::LinkedHashMap<GenesisHash, ProtocolParamUpdate>,
);

to_from_bytes!(ProposedProtocolParameterUpdates);

#[wasm_bindgen]
impl ProposedProtocolParameterUpdates {
    pub fn new() -> Self {
        Self(linked_hash_map::LinkedHashMap::new())
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProtocolVersion {
    major: u32,
    minor: u32,
}

to_from_bytes!(ProtocolVersion);

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
pub struct ProtocolVersions(Vec<ProtocolVersion>);

to_from_bytes!(ProtocolVersions);

#[wasm_bindgen]
impl ProtocolVersions {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ProtocolVersion {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &ProtocolVersion) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProtocolParamUpdate {
    minfee_a: Option<Coin>,
    minfee_b: Option<Coin>,
    max_block_body_size: Option<u32>,
    max_tx_size: Option<u32>,
    max_block_header_size: Option<u32>,
    key_deposit: Option<Coin>,
    pool_deposit: Option<Coin>,
    max_epoch: Option<Epoch>,
    // desired number of stake pools
    n_opt: Option<u32>,
    pool_pledge_influence: Option<Rational>,
    expansion_rate: Option<UnitInterval>,
    treasury_growth_rate: Option<UnitInterval>,
    // decentralization constant
    d: Option<UnitInterval>,
    extra_entropy: Option<Nonce>,
    protocol_version: Option<ProtocolVersions>,
    min_utxo_value: Option<Coin>,
}

to_from_bytes!(ProtocolParamUpdate);

#[wasm_bindgen]
impl ProtocolParamUpdate {
    pub fn set_minfee_a(&mut self, minfee_a: &Coin) {
        self.minfee_a = Some(minfee_a.clone())
    }

    pub fn minfee_a(&self) -> Option<Coin> {
        self.minfee_a.clone()
    }

    pub fn set_minfee_b(&mut self, minfee_b: &Coin) {
        self.minfee_b = Some(minfee_b.clone())
    }

    pub fn minfee_b(&self) -> Option<Coin> {
        self.minfee_b.clone()
    }

    pub fn set_max_block_body_size(&mut self, max_block_body_size: u32) {
        self.max_block_body_size = Some(max_block_body_size)
    }

    pub fn max_block_body_size(&self) -> Option<u32> {
        self.max_block_body_size.clone()
    }

    pub fn set_max_tx_size(&mut self, max_tx_size: u32) {
        self.max_tx_size = Some(max_tx_size)
    }

    pub fn max_tx_size(&self) -> Option<u32> {
        self.max_tx_size.clone()
    }

    pub fn set_max_block_header_size(&mut self, max_block_header_size: u32) {
        self.max_block_header_size = Some(max_block_header_size)
    }

    pub fn max_block_header_size(&self) -> Option<u32> {
        self.max_block_header_size.clone()
    }

    pub fn set_key_deposit(&mut self, key_deposit: &Coin) {
        self.key_deposit = Some(key_deposit.clone())
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.key_deposit.clone()
    }

    pub fn set_pool_deposit(&mut self, pool_deposit: &Coin) {
        self.pool_deposit = Some(pool_deposit.clone())
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.pool_deposit.clone()
    }

    pub fn set_max_epoch(&mut self, max_epoch: Epoch) {
        self.max_epoch = Some(max_epoch.clone())
    }

    pub fn max_epoch(&self) -> Option<Epoch> {
        self.max_epoch.clone()
    }

    pub fn set_n_opt(&mut self, n_opt: u32) {
        self.n_opt = Some(n_opt)
    }

    pub fn n_opt(&self) -> Option<u32> {
        self.n_opt.clone()
    }

    pub fn set_pool_pledge_influence(&mut self, pool_pledge_influence: &Rational) {
        self.pool_pledge_influence = Some(pool_pledge_influence.clone())
    }

    pub fn pool_pledge_influence(&self) -> Option<Rational> {
        self.pool_pledge_influence.clone()
    }

    pub fn set_expansion_rate(&mut self, expansion_rate: &UnitInterval) {
        self.expansion_rate = Some(expansion_rate.clone())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.expansion_rate.clone()
    }

    pub fn set_treasury_growth_rate(&mut self, treasury_growth_rate: &UnitInterval) {
        self.treasury_growth_rate = Some(treasury_growth_rate.clone())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.treasury_growth_rate.clone()
    }

    pub fn set_d(&mut self, d: &UnitInterval) {
        self.d = Some(d.clone())
    }

    pub fn d(&self) -> Option<UnitInterval> {
        self.d.clone()
    }

    pub fn set_extra_entropy(&mut self, extra_entropy: &Nonce) {
        self.extra_entropy = Some(extra_entropy.clone())
    }

    pub fn extra_entropy(&self) -> Option<Nonce> {
        self.extra_entropy.clone()
    }

    pub fn set_protocol_version(&mut self, protocol_version: &ProtocolVersions) {
        self.protocol_version = Some(protocol_version.clone())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersions> {
        self.protocol_version.clone()
    }

    pub fn set_min_utxo_value(&mut self, min_utxo_value: &Coin) {
        self.min_utxo_value = Some(min_utxo_value.clone())
    }

    pub fn min_utxo_value(&self) -> Option<Coin> {
        self.min_utxo_value.clone()
    }

    pub fn new() -> Self {
        Self {
            minfee_a: None,
            minfee_b: None,
            max_block_body_size: None,
            max_tx_size: None,
            max_block_header_size: None,
            key_deposit: None,
            pool_deposit: None,
            max_epoch: None,
            n_opt: None,
            pool_pledge_influence: None,
            expansion_rate: None,
            treasury_growth_rate: None,
            d: None,
            extra_entropy: None,
            protocol_version: None,
            min_utxo_value: None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TransactionBodies(Vec<TransactionBody>);

to_from_bytes!(TransactionBodies);

#[wasm_bindgen]
impl TransactionBodies {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionBody {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionBody) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TransactionWitnessSets(Vec<TransactionWitnessSet>);

to_from_bytes!(TransactionWitnessSets);
#[wasm_bindgen]
impl TransactionWitnessSets {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionWitnessSet {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionWitnessSet) {
        self.0.push(elem.clone());
    }
}

pub type TransactionIndexes = Vec<TransactionIndex>;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MapTransactionIndexToTransactionMetadata(linked_hash_map::LinkedHashMap<TransactionIndex, TransactionMetadata>);

#[wasm_bindgen]
impl MapTransactionIndexToTransactionMetadata {
    pub fn new() -> Self {
        Self(linked_hash_map::LinkedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: TransactionIndex, value: &TransactionMetadata) -> Option<TransactionMetadata> {
        self.0.insert(key, value.clone())
    }

    pub fn get(&self, key: TransactionIndex) -> Option<TransactionMetadata> {
        self.0.get(&key).map(|v| v.clone())
    }

    pub fn keys(&self) -> TransactionIndexes {
        self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<TransactionIndex>>()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Block {
    header: Header,
    transaction_bodies: TransactionBodies,
    transaction_witness_sets: TransactionWitnessSets,
    transaction_metadata_set: MapTransactionIndexToTransactionMetadata,
}

to_from_bytes!(Block);

#[wasm_bindgen]
impl Block {
    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn transaction_bodies(&self) -> TransactionBodies {
        self.transaction_bodies.clone()
    }

    pub fn transaction_witness_sets(&self) -> TransactionWitnessSets {
        self.transaction_witness_sets.clone()
    }

    pub fn transaction_metadata_set(&self) -> MapTransactionIndexToTransactionMetadata {
        self.transaction_metadata_set.clone()
    }

    pub fn new(header: &Header, transaction_bodies: &TransactionBodies, transaction_witness_sets: &TransactionWitnessSets, transaction_metadata_set: &MapTransactionIndexToTransactionMetadata) -> Self {
        Self {
            header: header.clone(),
            transaction_bodies: transaction_bodies.clone(),
            transaction_witness_sets: transaction_witness_sets.clone(),
            transaction_metadata_set: transaction_metadata_set.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Header {
    header_body: HeaderBody,
    body_signature: KESSignature,
}

to_from_bytes!(Header);

#[wasm_bindgen]
impl Header {
    pub fn header_body(&self) -> HeaderBody {
        self.header_body.clone()
    }

    pub fn body_signature(&self) -> KESSignature {
        self.body_signature.clone()
    }

    pub fn new(header_body: &HeaderBody, body_signature: &KESSignature) -> Self {
        Self {
            header_body: header_body.clone(),
            body_signature: body_signature.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct OperationalCert {
    hot_vkey: KESVKey,
    sequence_number: u32,
    kes_period: u32,
    sigma: Ed25519Signature,
}

to_from_bytes!(OperationalCert);

#[wasm_bindgen]
impl OperationalCert {
    pub fn hot_vkey(&self) -> KESVKey {
        self.hot_vkey.clone()
    }

    pub fn sequence_number(&self) -> u32 {
        self.sequence_number.clone()
    }

    pub fn kes_period(&self) -> u32 {
        self.kes_period.clone()
    }

    pub fn sigma(&self) -> Ed25519Signature {
        self.sigma.clone()
    }

    pub fn new(hot_vkey: &KESVKey, sequence_number: u32, kes_period: u32, sigma: &Ed25519Signature) -> Self {
        Self {
            hot_vkey: hot_vkey.clone(),
            sequence_number: sequence_number,
            kes_period: kes_period,
            sigma: sigma.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct HeaderBody {
    block_number: u32,
    slot: Slot,
    prev_hash: Option<BlockHash>,
    issuer_vkey: Vkey,
    vrf_vkey: VRFVKey,
    nonce_vrf: VRFCert,
    leader_vrf: VRFCert,
    block_body_size: u32,
    block_body_hash: BlockHash,
    operational_cert: OperationalCert,
    protocol_version: ProtocolVersion,
}

to_from_bytes!(HeaderBody);

#[wasm_bindgen]
impl HeaderBody {
    pub fn block_number(&self) -> u32 {
        self.block_number.clone()
    }

    pub fn slot(&self) -> Slot {
        self.slot.clone()
    }

    pub fn prev_hash(&self) -> Option<BlockHash> {
        self.prev_hash.clone()
    }

    pub fn issuer_vkey(&self) -> Vkey {
        self.issuer_vkey.clone()
    }

    pub fn vrf_vkey(&self) -> VRFVKey {
        self.vrf_vkey.clone()
    }

    pub fn nonce_vrf(&self) -> VRFCert {
        self.nonce_vrf.clone()
    }

    pub fn leader_vrf(&self) -> VRFCert {
        self.leader_vrf.clone()
    }

    pub fn block_body_size(&self) -> u32 {
        self.block_body_size.clone()
    }

    pub fn block_body_hash(&self) -> BlockHash {
        self.block_body_hash.clone()
    }

    pub fn operational_cert(&self) -> OperationalCert {
        self.operational_cert.clone()
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version.clone()
    }

    pub fn new(block_number: u32, slot: Slot, prev_hash: Option<BlockHash>, issuer_vkey: &Vkey, vrf_vkey: &VRFVKey, nonce_vrf: &VRFCert, leader_vrf: &VRFCert, block_body_size: u32, block_body_hash: &BlockHash, operational_cert: &OperationalCert, protocol_version: &ProtocolVersion) -> Self {
        Self {
            block_number: block_number,
            slot: slot,
            prev_hash: prev_hash.clone(),
            issuer_vkey: issuer_vkey.clone(),
            vrf_vkey: vrf_vkey.clone(),
            nonce_vrf: nonce_vrf.clone(),
            leader_vrf: leader_vrf.clone(),
            block_body_size: block_body_size,
            block_body_hash: block_body_hash.clone(),
            operational_cert: operational_cert.clone(),
            protocol_version: protocol_version.clone(),
        }
    }
}
