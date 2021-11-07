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

use std::convert::TryInto;
use std::io::{BufRead, Seek, Write};

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
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
pub mod plutus;
pub mod serialization;
pub mod tx_builder;
pub mod typed_bytes;
pub mod emip3;
#[macro_use]
pub mod utils;

use address::*;
use crypto::*;
use error::*;
use plutus::*;
use metadata::*;
use utils::*;
use std::cmp::Ordering;

type DeltaCoin = Int;

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

type SubCoin = UnitInterval;
type Rational = UnitInterval;
type Epoch = u32;
type Slot = u32;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Transaction {
    body: TransactionBody,
    witness_set: TransactionWitnessSet,
    is_valid: bool,
    auxiliary_data: Option<AuxiliaryData>,
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

pub type RequiredSigners = Ed25519KeyHashes;

#[wasm_bindgen]
#[derive(Clone)]
pub struct TransactionBody {
    inputs: TransactionInputs,
    outputs: TransactionOutputs,
    fee: Coin,
    ttl: Option<Slot>,
    certs: Option<Certificates>,
    withdrawals: Option<Withdrawals>,
    update: Option<Update>,
    auxiliary_data_hash: Option<AuxiliaryDataHash>,
    validity_start_interval: Option<Slot>,
    mint: Option<Mint>,
    script_data_hash: Option<ScriptDataHash>,
    collateral: Option<TransactionInputs>,
    required_signers: Option<RequiredSigners>,
    network_id: Option<NetworkId>,
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

    pub fn ttl(&self) -> Option<Slot> {
        self.ttl
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

    pub fn set_auxiliary_data_hash(&mut self, auxiliary_data_hash: &AuxiliaryDataHash) {
        self.auxiliary_data_hash = Some(auxiliary_data_hash.clone())
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.auxiliary_data_hash.clone()
    }

    pub fn set_validity_start_interval(&mut self, validity_start_interval: Slot) {
        self.validity_start_interval = Some(validity_start_interval)
    }

    pub fn validity_start_interval(&self) -> Option<Slot> {
        self.validity_start_interval.clone()
    }

    pub fn set_mint(&mut self, mint: &Mint) {
        self.mint = Some(mint.clone())
    }

    pub fn multiassets(&self) -> Option<Mint> {
        self.mint.clone()
    }

    pub fn set_script_data_hash(&mut self, script_data_hash: &ScriptDataHash) {
        self.script_data_hash = Some(script_data_hash.clone())
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        self.script_data_hash.clone()
    }

    pub fn set_collateral(&mut self, collateral: &TransactionInputs) {
        self.collateral = Some(collateral.clone())
    }

    pub fn collateral(&self) -> Option<TransactionInputs> {
        self.collateral.clone()
    }

    pub fn set_required_signers(&mut self, required_signers: &RequiredSigners) {
        self.required_signers = Some(required_signers.clone())
    }

    pub fn required_signers(&self) -> Option<RequiredSigners> {
        self.required_signers.clone()
    }

    pub fn set_network_id(&mut self, network_id: &NetworkId) {
        self.network_id = Some(network_id.clone())
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.network_id.clone()
    }

    pub fn new(
        inputs: &TransactionInputs,
        outputs: &TransactionOutputs,
        fee: &Coin,
        ttl: Option<Slot>) -> Self {
        Self {
            inputs: inputs.clone(),
            outputs: outputs.clone(),
            fee: fee.clone(),
            ttl: ttl,
            certs: None,
            withdrawals: None,
            update: None,
            auxiliary_data_hash: None,
            validity_start_interval: None,
            mint: None,
            script_data_hash: None,
            collateral: None,
            required_signers: None,
            network_id: None,
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
    pub (crate) amount: Value,
    data_hash: Option<DataHash>,
}

to_from_bytes!(TransactionOutput);

#[wasm_bindgen]
impl TransactionOutput {
    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn amount(&self) -> Value {
        self.amount.clone()
    }

    pub fn data_hash(&self) -> Option<DataHash> {
        self.data_hash.clone()
    }

    pub fn set_data_hash(&mut self, data_hash: &DataHash) {
        self.data_hash = Some(data_hash.clone());
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self {
            address: address.clone(),
            amount: amount.clone(),
            data_hash: None,
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
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MIRPot {
    Reserves,
    Treasury,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MIREnum {
    ToOtherPot(Coin),
    ToStakeCredentials(MIRToStakeCredentials),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MIRKind {
    ToOtherPot,
    ToStakeCredentials,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MIRToStakeCredentials {
    rewards: linked_hash_map::LinkedHashMap<StakeCredential, DeltaCoin>,
}

to_from_bytes!(MIRToStakeCredentials);

#[wasm_bindgen]
impl MIRToStakeCredentials {
    pub fn new() -> Self {
        Self {
            rewards: linked_hash_map::LinkedHashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.rewards.len()
    }

    pub fn insert(&mut self, cred: &StakeCredential, delta: &DeltaCoin) -> Option<DeltaCoin> {
        self.rewards.insert(cred.clone(), delta.clone())
    }

    pub fn get(&self, cred: &StakeCredential) -> Option<DeltaCoin> {
        self.rewards.get(cred).map(|v| v.clone())
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

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoveInstantaneousReward {
    pot: MIRPot,
    variant: MIREnum, 
}

to_from_bytes!(MoveInstantaneousReward);

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn new_to_other_pot(pot: MIRPot, amount: &Coin) -> Self {
        Self {
            pot,
            variant: MIREnum::ToOtherPot(amount.clone()),
        }
    }

    pub fn new_to_stake_creds(pot: MIRPot, amounts: &MIRToStakeCredentials) -> Self {
        Self {
            pot,
            variant: MIREnum::ToStakeCredentials(amounts.clone()),
        }
    }

    pub fn pot(&self) -> MIRPot {
        self.pot
    }

    pub fn kind(&self) -> MIRKind {
        match &self.variant {
            MIREnum::ToOtherPot(_) => MIRKind::ToOtherPot,
            MIREnum::ToStakeCredentials(_) => MIRKind::ToStakeCredentials,
        }
    }

    pub fn as_to_other_pot(&self) -> Option<Coin> {
        match &self.variant {
            MIREnum::ToOtherPot(amount) => Some(amount.clone()),
            MIREnum::ToStakeCredentials(_) => None,
        }
    }

    pub fn as_to_stake_creds(&self) -> Option<MIRToStakeCredentials> {
        match &self.variant {
            MIREnum::ToOtherPot(_) => None,
            MIREnum::ToStakeCredentials(amounts) => Some(amounts.clone()),
        }
    }
}

type Port = u16;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ipv4([u8; 4]);

to_from_bytes!(Ipv4);

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ipv6([u8; 16]);

to_from_bytes!(Ipv6);

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

static URL_MAX_LEN: usize = 64;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct URL(String);

to_from_bytes!(URL);

#[wasm_bindgen]
impl URL {
    pub fn new(url: String) -> Result<URL, JsError> {
        Self::new_impl(url).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub (crate) fn new_impl(url: String) -> Result<URL, DeserializeError> {
        if url.len() <= URL_MAX_LEN {
            Ok(Self(url))
        } else {
            Err(DeserializeError::new("URL", DeserializeFailure::OutOfRange{
                min: 0,
                max: URL_MAX_LEN,
                found: url.len(),
            }))
        }
    }

    pub fn url(&self) -> String {
        self.0.clone()
    }
}

static DNS_NAME_MAX_LEN: usize = 64;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DNSRecordAorAAAA(String);

to_from_bytes!(DNSRecordAorAAAA);

#[wasm_bindgen]
impl DNSRecordAorAAAA {
    pub fn new(dns_name: String) -> Result<DNSRecordAorAAAA, JsError> {
        Self::new_impl(dns_name).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub (crate) fn new_impl(dns_name: String) -> Result<DNSRecordAorAAAA, DeserializeError> {
        if dns_name.len() <= DNS_NAME_MAX_LEN {
            Ok(Self(dns_name))
        } else {
            Err(DeserializeError::new("DNSRecordAorAAAA", DeserializeFailure::OutOfRange{
                min: 0,
                max: DNS_NAME_MAX_LEN,
                found: dns_name.len(),
            }))
        }
    }

    pub fn record(&self) -> String {
        self.0.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DNSRecordSRV(String);

to_from_bytes!(DNSRecordSRV);

#[wasm_bindgen]
impl DNSRecordSRV {
    pub fn new(dns_name: String) -> Result<DNSRecordSRV, JsError> {
        Self::new_impl(dns_name).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub (crate) fn new_impl(dns_name: String) -> Result<DNSRecordSRV, DeserializeError> {
        if dns_name.len() <= DNS_NAME_MAX_LEN {
            Ok(Self(dns_name))
        } else {
            Err(DeserializeError::new("DNSRecordSRV", DeserializeFailure::OutOfRange{
                min: 0,
                max: DNS_NAME_MAX_LEN,
                found: dns_name.len(),
            }))
        }
    }

    pub fn record(&self) -> String {
        self.0.clone()
    }
}

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
    dns_name: DNSRecordAorAAAA,
}

to_from_bytes!(SingleHostName);

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MultiHostName {
    dns_name: DNSRecordSRV,
}

to_from_bytes!(MultiHostName);

#[wasm_bindgen]
impl MultiHostName {
    pub fn dns_name(&self) -> DNSRecordSRV {
        self.dns_name.clone()
    }

    pub fn new(dns_name: &DNSRecordSRV) -> Self {
        Self { dns_name: dns_name.clone() }
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
    url: URL,
    pool_metadata_hash: PoolMetadataHash,
}

to_from_bytes!(PoolMetadata);

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
#[derive(Clone)]
pub struct TransactionWitnessSet {
    vkeys: Option<Vkeywitnesses>,
    native_scripts: Option<NativeScripts>,
    bootstraps: Option<BootstrapWitnesses>,
    plutus_scripts: Option<PlutusScripts>,
    plutus_data: Option<PlutusList>,
    redeemers: Option<Redeemers>,
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

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScripts) {
        self.native_scripts = Some(native_scripts.clone())
    }

    pub fn native_scripts(&self) -> Option<NativeScripts> {
        self.native_scripts.clone()
    }

    pub fn set_bootstraps(&mut self, bootstraps: &BootstrapWitnesses) {
        self.bootstraps = Some(bootstraps.clone())
    }

    pub fn bootstraps(&self) -> Option<BootstrapWitnesses> {
        self.bootstraps.clone()
    }

    pub fn set_plutus_scripts(&mut self, plutus_scripts: &PlutusScripts) {
        self.plutus_scripts = Some(plutus_scripts.clone())
    }

    pub fn plutus_scripts(&self) -> Option<PlutusScripts> {
        self.plutus_scripts.clone()
    }

    pub fn set_plutus_data(&mut self, plutus_data: &PlutusList) {
        self.plutus_data = Some(plutus_data.clone())
    }

    pub fn plutus_data(&self) -> Option<PlutusList> {
        self.plutus_data.clone()
    }

    pub fn set_redeemers(&mut self, redeemers: &Redeemers) {
        self.redeemers = Some(redeemers.clone())
    }

    pub fn redeemers(&self) -> Option<Redeemers> {
        self.redeemers.clone()
    }

    pub fn new() -> Self {
        Self {
            vkeys: None,
            native_scripts: None,
            bootstraps: None,
            plutus_scripts: None,
            plutus_data: None,
            redeemers: None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScriptPubkey {
    addr_keyhash: Ed25519KeyHash,
}

to_from_bytes!(ScriptPubkey);

#[wasm_bindgen]
impl ScriptPubkey {
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
pub struct ScriptAll {
    native_scripts: NativeScripts,
}

to_from_bytes!(ScriptAll);

#[wasm_bindgen]
impl ScriptAll {
    pub fn native_scripts(&self) -> NativeScripts {
        self.native_scripts.clone()
    }

    pub fn new(native_scripts: &NativeScripts) -> Self {
        Self {
            native_scripts: native_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScriptAny {
    native_scripts: NativeScripts,
}

to_from_bytes!(ScriptAny);

#[wasm_bindgen]
impl ScriptAny {
    pub fn native_scripts(&self) -> NativeScripts {
        self.native_scripts.clone()
    }

    pub fn new(native_scripts: &NativeScripts) -> Self {
        Self {
            native_scripts: native_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScriptNOfK {
    n: u32,
    native_scripts: NativeScripts,
}

to_from_bytes!(ScriptNOfK);

#[wasm_bindgen]
impl ScriptNOfK {
    pub fn n(&self) -> u32 {
        self.n
    }

    pub fn native_scripts(&self) -> NativeScripts {
        self.native_scripts.clone()
    }

    pub fn new(n: u32, native_scripts: &NativeScripts) -> Self {
        Self {
            n: n,
            native_scripts: native_scripts.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TimelockStart {
    slot: Slot,
}

to_from_bytes!(TimelockStart);

#[wasm_bindgen]
impl TimelockStart {
    pub fn slot(&self) -> Slot {
        self.slot
    }

    pub fn new(slot: Slot) -> Self {
        Self {
            slot,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TimelockExpiry {
    slot: Slot,
}

to_from_bytes!(TimelockExpiry);

#[wasm_bindgen]
impl TimelockExpiry {
    pub fn slot(&self) -> Slot {
        self.slot
    }

    pub fn new(slot: Slot) -> Self {
        Self {
            slot,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NativeScriptKind {
    ScriptPubkey,
    ScriptAll,
    ScriptAny,
    ScriptNOfK,
    TimelockStart,
    TimelockExpiry,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum NativeScriptEnum {
    ScriptPubkey(ScriptPubkey),
    ScriptAll(ScriptAll),
    ScriptAny(ScriptAny),
    ScriptNOfK(ScriptNOfK),
    TimelockStart(TimelockStart),
    TimelockExpiry(TimelockExpiry),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NativeScript(NativeScriptEnum);

to_from_bytes!(NativeScript);

/// Each new language uses a different namespace for hashing its script
/// This is because you could have a language where the same bytes have different semantics
/// So this avoids scripts in different languages mapping to the same hash
/// Note that the enum value here is different than the enum value for deciding the cost model of a script
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ScriptHashNamespace {
    NativeScript,
    // TODO: do we need to update this for Plutus?
}

#[wasm_bindgen]
impl NativeScript {
    pub fn hash(&self, namespace: ScriptHashNamespace) -> Ed25519KeyHash {
        let mut bytes = Vec::with_capacity(self.to_bytes().len() + 1);
        bytes.extend_from_slice(&vec![namespace as u8]);
        bytes.extend_from_slice(&self.to_bytes());
        Ed25519KeyHash::from(blake2b224(bytes.as_ref()))
    }

    pub fn new_script_pubkey(script_pubkey: &ScriptPubkey) -> Self {
        Self(NativeScriptEnum::ScriptPubkey(script_pubkey.clone()))
    }

    pub fn new_script_all(script_all: &ScriptAll) -> Self {
        Self(NativeScriptEnum::ScriptAll(script_all.clone()))
    }

    pub fn new_script_any(script_any: &ScriptAny) -> Self {
        Self(NativeScriptEnum::ScriptAny(script_any.clone()))
    }

    pub fn new_script_n_of_k(script_n_of_k: &ScriptNOfK) -> Self {
        Self(NativeScriptEnum::ScriptNOfK(script_n_of_k.clone()))
    }

    pub fn new_timelock_start(timelock_start: &TimelockStart) -> Self {
        Self(NativeScriptEnum::TimelockStart(timelock_start.clone()))
    }

    pub fn new_timelock_expiry(timelock_expiry: &TimelockExpiry) -> Self {
        Self(NativeScriptEnum::TimelockExpiry(timelock_expiry.clone()))
    }

    pub fn kind(&self) -> NativeScriptKind {
        match &self.0 {
            NativeScriptEnum::ScriptPubkey(_) => NativeScriptKind::ScriptPubkey,
            NativeScriptEnum::ScriptAll(_) => NativeScriptKind::ScriptAll,
            NativeScriptEnum::ScriptAny(_) => NativeScriptKind::ScriptAny,
            NativeScriptEnum::ScriptNOfK(_) => NativeScriptKind::ScriptNOfK,
            NativeScriptEnum::TimelockStart(_) => NativeScriptKind::TimelockStart,
            NativeScriptEnum::TimelockExpiry(_) => NativeScriptKind::TimelockExpiry,
        }
    }

    pub fn as_script_pubkey(&self) -> Option<ScriptPubkey> {
        match &self.0 {
            NativeScriptEnum::ScriptPubkey(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_script_all(&self) -> Option<ScriptAll> {
        match &self.0 {
            NativeScriptEnum::ScriptAll(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_script_any(&self) -> Option<ScriptAny> {
        match &self.0 {
            NativeScriptEnum::ScriptAny(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_script_n_of_k(&self) -> Option<ScriptNOfK> {
        match &self.0 {
            NativeScriptEnum::ScriptNOfK(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_timelock_start(&self) -> Option<TimelockStart> {
        match &self.0 {
            NativeScriptEnum::TimelockStart(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_timelock_expiry(&self) -> Option<TimelockExpiry> {
        match &self.0 {
            NativeScriptEnum::TimelockExpiry(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NativeScripts(Vec<NativeScript>);

#[wasm_bindgen]
impl NativeScripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> NativeScript {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &NativeScript) {
        self.0.push(elem.clone());
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
pub struct ScriptHashes(pub (crate) Vec<ScriptHash>);

to_from_bytes!(ScriptHashes);

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
    min_pool_cost: Option<Coin>,
    ada_per_utxo_byte: Option<Coin>,
    cost_models: Option<Costmdls>,
    execution_costs: Option<ExUnitPrices>,
    max_tx_ex_units: Option<ExUnits>,
    max_block_ex_units: Option<ExUnits>,
    max_value_size: Option<u32>,
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

    pub fn set_min_pool_cost(&mut self, min_pool_cost: &Coin) {
        self.min_pool_cost = Some(min_pool_cost.clone())
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        self.min_pool_cost.clone()
    }

    pub fn set_ada_per_utxo_byte(&mut self, ada_per_utxo_byte: &Coin) {
        self.ada_per_utxo_byte = Some(ada_per_utxo_byte.clone())
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        self.ada_per_utxo_byte.clone()
    }

    pub fn set_cost_models(&mut self, cost_models: &Costmdls) {
        self.cost_models = Some(cost_models.clone())
    }

    pub fn cost_models(&self) -> Option<Costmdls> {
        self.cost_models.clone()
    }

    pub fn set_execution_costs(&mut self, execution_costs: &ExUnitPrices) {
        self.execution_costs = Some(execution_costs.clone())
    }

    pub fn execution_costs(&self) -> Option<ExUnitPrices> {
        self.execution_costs.clone()
    }

    pub fn set_max_tx_ex_units(&mut self, max_tx_ex_units: &ExUnits) {
        self.max_tx_ex_units = Some(max_tx_ex_units.clone())
    }

    pub fn max_tx_ex_units(&self) -> Option<ExUnits> {
        self.max_tx_ex_units.clone()
    }

    pub fn set_max_block_ex_units(&mut self, max_block_ex_units: &ExUnits) {
        self.max_block_ex_units = Some(max_block_ex_units.clone())
    }

    pub fn max_block_ex_units(&self) -> Option<ExUnits> {
        self.max_block_ex_units.clone()
    }

    pub fn set_max_value_size(&mut self, max_value_size: u32) {
        self.max_value_size = Some(max_value_size.clone())
    }

    pub fn max_value_size(&self) -> Option<u32> {
        self.max_value_size.clone()
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
            min_pool_cost: None,
            ada_per_utxo_byte: None,
            cost_models: None,
            execution_costs: None,
            max_tx_ex_units: None,
            max_block_ex_units: None,
            max_value_size: None,
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
pub struct AuxiliaryDataSet(linked_hash_map::LinkedHashMap<TransactionIndex, AuxiliaryData>);

#[wasm_bindgen]
impl AuxiliaryDataSet {
    pub fn new() -> Self {
        Self(linked_hash_map::LinkedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, tx_index: TransactionIndex, data: &AuxiliaryData) -> Option<AuxiliaryData> {
        self.0.insert(tx_index, data.clone())
    }

    pub fn get(&self, tx_index: TransactionIndex) -> Option<AuxiliaryData> {
        self.0.get(&tx_index).map(|v| v.clone())
    }

    pub fn indices(&self) -> TransactionIndexes {
        self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<TransactionIndex>>()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Block {
    header: Header,
    transaction_bodies: TransactionBodies,
    transaction_witness_sets: TransactionWitnessSets,
    auxiliary_data_set: AuxiliaryDataSet,
    invalid_transactions: TransactionIndexes,
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

    pub fn auxiliary_data_set(&self) -> AuxiliaryDataSet {
        self.auxiliary_data_set.clone()
    }

    pub fn invalid_transactions(&self) -> TransactionIndexes {
        self.invalid_transactions.clone()
    }

    pub fn new(header: &Header, transaction_bodies: &TransactionBodies, transaction_witness_sets: &TransactionWitnessSets, auxiliary_data_set: &AuxiliaryDataSet, invalid_transactions: TransactionIndexes) -> Self {
        Self {
            header: header.clone(),
            transaction_bodies: transaction_bodies.clone(),
            transaction_witness_sets: transaction_witness_sets.clone(),
            auxiliary_data_set: auxiliary_data_set.clone(),
            invalid_transactions: invalid_transactions,
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



#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetName(Vec<u8>);

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

to_from_bytes!(AssetName);

#[wasm_bindgen]
impl AssetName {
    pub fn new(name: Vec<u8>) -> Result<AssetName, JsError> {
        Self::new_impl(name).map_err(|e| JsError::from_str(&e.to_string()))
    }

    pub (crate) fn new_impl(name: Vec<u8>) -> Result<AssetName, DeserializeError> {
        if name.len() <= 32 {
            Ok(Self(name))
        } else {
            Err(DeserializeError::new("AssetName", DeserializeFailure::OutOfRange{
                min: 0,
                max: 32,
                found: name.len(),
            }))
        }
    }

    pub fn name(&self) -> Vec<u8> {
        self.0.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AssetNames(Vec<AssetName>);

to_from_bytes!(AssetNames);

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Assets(pub (crate) std::collections::BTreeMap<AssetName, BigNum>);

to_from_bytes!(Assets);

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
        AssetNames(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<AssetName>>())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq)]
pub struct MultiAsset(pub (crate) std::collections::BTreeMap<PolicyID, Assets>);

to_from_bytes!(MultiAsset);

#[wasm_bindgen]
impl MultiAsset {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PolicyID, value: &Assets) -> Option<Assets> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &PolicyID) -> Option<Assets> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> PolicyIDs {
        ScriptHashes(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<PolicyID>>())
    }

    /// removes an asset from the list if the result is 0 or less
    pub fn sub(&self, rhs_ma: &MultiAsset) -> MultiAsset {
        let mut lhs_ma = self.clone();
        for (policy, assets) in &rhs_ma.0 {
            for (asset_name, amount) in &assets.0 {
                match lhs_ma.0.get_mut(policy) {
                    Some(assets) => match assets.0.get_mut(asset_name) {
                        Some(current) => match current.checked_sub(&amount) {
                            Ok(new) => {
                                match new.compare(&to_bignum(0)) {
                                    0 => {
                                        assets.0.remove(asset_name);
                                        match assets.0.len() {
                                            0 => { lhs_ma.0.remove(policy); },
                                            _ => {},
                                        }
                                    },
                                    _ => *current = new
                                }
                            },
                            Err(_) => {
                                assets.0.remove(asset_name);
                                match assets.0.len() {
                                    0 => { lhs_ma.0.remove(policy); },
                                    _ => {},
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
            ma.get(&pid).and_then(|assets| assets.get(aname))
                .unwrap_or(to_bignum(0u64)) // assume 0 if asset not present
        }

        // idea: if (a-b) > 0 for some asset, then a > b for at least some asset
        fn is_all_zeros(lhs: &MultiAsset, rhs: &MultiAsset) -> bool {
            for (pid, assets) in lhs.0.iter() {
                for (aname, amount) in assets.0.iter() {
                    match amount
                            .clamped_sub(&amount_or_zero(&rhs, pid, aname))
                            .cmp(&to_bignum(0))
                    {
                        std::cmp::Ordering::Equal => (),
                        _ => return false
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MintAssets(std::collections::BTreeMap<AssetName, Int>);

#[wasm_bindgen]
impl MintAssets {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetName, value: Int) -> Option<Int> {
        self.0.insert(key.clone(), value)
    }

    pub fn get(&self, key: &AssetName) -> Option<Int> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> AssetNames {
        AssetNames(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<AssetName>>())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Mint(std::collections::BTreeMap<PolicyID, MintAssets>);

to_from_bytes!(Mint);

#[wasm_bindgen]
impl Mint {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PolicyID, value: &MintAssets) -> Option<MintAssets> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &PolicyID) -> Option<MintAssets> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> PolicyIDs {
        ScriptHashes(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<ScriptHash>>())
    }
}


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NetworkIdKind {
    Testnet,
    Mainnet,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NetworkId(NetworkIdKind);

to_from_bytes!(NetworkId);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_script_hash() {
        let hash = Ed25519KeyHash::from_bytes(vec![143, 180, 186, 93, 223, 42, 243, 7, 81, 98, 86, 125, 97, 69, 110, 52, 130, 243, 244, 98, 246, 13, 33, 212, 128, 168, 136, 40]).unwrap();
        assert_eq!(hex::encode(&hash.to_bytes()), "8fb4ba5ddf2af3075162567d61456e3482f3f462f60d21d480a88828");

        let script = NativeScript::new_script_pubkey(&ScriptPubkey::new(&hash));

        let script_hash = ScriptHash::from_bytes(
            script.hash(ScriptHashNamespace::NativeScript).to_bytes()
        ).unwrap();

        assert_eq!(hex::encode(&script_hash.to_bytes()), "187b8d3ddcb24013097c003da0b8d8f7ddcf937119d8f59dccd05a0f");
    }

    #[test]
    fn asset_name_ord() {

        let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let name11 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let name2 = AssetName::new(vec![0u8, 4, 5, 6]).unwrap();
        let name22 = AssetName::new(vec![0u8, 4, 5, 6]).unwrap();

        let name3 = AssetName::new(vec![0u8, 7, 8]).unwrap();
        let name33 = AssetName::new(vec![0u8, 7, 8]).unwrap();

        assert_eq!(name1.cmp(&name2), Ordering::Less);
        assert_eq!(name2.cmp(&name1), Ordering::Greater);
        assert_eq!(name1.cmp(&name3), Ordering::Greater);
        assert_eq!(name2.cmp(&name3), Ordering::Greater);
        assert_eq!(name3.cmp(&name1), Ordering::Less);
        assert_eq!(name3.cmp(&name2), Ordering::Less);

        assert_eq!(name1.cmp(&name11), Ordering::Equal);
        assert_eq!(name2.cmp(&name22), Ordering::Equal);
        assert_eq!(name3.cmp(&name33), Ordering::Equal);

        let mut map = Assets::new();
        map.insert(&name2, &to_bignum(1));
        map.insert(&name1, &to_bignum(1));
        map.insert(&name3, &to_bignum(1));

        assert_eq!(map.keys(), AssetNames(vec![name3, name1, name2]));

        let mut map2 = MintAssets::new();
        map2.insert(&name11, Int::new_i32(1));
        map2.insert(&name33, Int::new_i32(1));
        map2.insert(&name22, Int::new_i32(1));

        assert_eq!(map2.keys(), AssetNames(vec![name33, name11, name22]));
    }
}
