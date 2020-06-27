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
    index_0: u32,
    index_1: u32,
}

#[wasm_bindgen]
impl UnitInterval {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<UnitInterval, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(index_0: u32, index_1: u32) -> Self {
        Self {
            index_0: index_0,
            index_1: index_1,
        }
    }
}

type Coin = u64;

type Epoch = u32;

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

    pub fn new(addr_keyhash: AddrKeyHash) -> Self {
        Self {
            addr_keyhash: addr_keyhash,
        }
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

    pub fn add(&mut self, elem: MultisigScript) {
        self.0.push(elem);
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

    pub fn new(multisig_scripts: MultisigScripts) -> Self {
        Self {
            multisig_scripts: multisig_scripts,
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

    pub fn new(multisig_scripts: MultisigScripts) -> Self {
        Self {
            multisig_scripts: multisig_scripts,
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

    pub fn new(n: u32, multisig_scripts: MultisigScripts) -> Self {
        Self {
            n: n,
            multisig_scripts: multisig_scripts,
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

    pub fn new_msig_pubkey(msig_pubkey: MsigPubkey) -> Self {
        Self(MultisigScriptEnum::MsigPubkey(msig_pubkey))
    }

    pub fn new_msig_all(msig_all: MsigAll) -> Self {
        Self(MultisigScriptEnum::MsigAll(msig_all))
    }

    pub fn new_msig_any(msig_any: MsigAny) -> Self {
        Self(MultisigScriptEnum::MsigAny(msig_any))
    }

    pub fn new_msig_n_of_k(msig_n_of_k: MsigNOfK) -> Self {
        Self(MultisigScriptEnum::MsigNOfK(msig_n_of_k))
    }
}

type TransactionIndex = u32;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionInput {
    transaction_id: TransactionHash,
    index: TransactionIndex,
}

#[wasm_bindgen]
impl TransactionInput {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionInput, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(transaction_id: TransactionHash, index: TransactionIndex) -> Self {
        Self {
            transaction_id: transaction_id,
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

    pub fn new(address: Address, amount: Coin) -> Self {
        Self {
            address: address,
            amount: amount,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Withdrawals(std::collections::BTreeMap<StakeCredential, u32>);

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

    pub fn insert(&mut self, key: StakeCredential, value: u32) -> Option<u32> {
        self.0.insert(key, value)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoveInstantaneousReward(std::collections::BTreeMap<StakeCredential, u32>);

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MoveInstantaneousReward, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: StakeCredential, value: u32) -> Option<u32> {
        self.0.insert(key, value)
    }
}

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
            ipv4: ipv4,
            ipv6: ipv6,
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
    port: Option<Port>,
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

    pub fn new(port: Option<Port>, dns_name: DnsName) -> Self {
        Self {
            port: port,
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

    pub fn new_single_host_addr(single_host_addr: SingleHostAddr) -> Self {
        Self(RelayEnum::SingleHostAddr(single_host_addr))
    }

    pub fn new_single_host_name(single_host_name: SingleHostName) -> Self {
        Self(RelayEnum::SingleHostName(single_host_name))
    }

    pub fn new_multi_host_name(multi_host_name: MultiHostName) -> Self {
        Self(RelayEnum::MultiHostName(multi_host_name))
    }
}

type Port = u32;

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

    pub fn new(url: Url, metadata_hash: MetadataHash) -> Self {
        Self {
            url: url,
            metadata_hash: metadata_hash,
        }
    }
}

type Url = String;

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

    pub fn new(stake_credential: StakeCredential) -> Self {
        Self {
            stake_credential: stake_credential,
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

    pub fn new(stake_credential: StakeCredential) -> Self {
        Self {
            stake_credential: stake_credential,
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

    pub fn new(stake_credential: StakeCredential, pool_keyhash: PoolKeyHash) -> Self {
        Self {
            stake_credential: stake_credential,
            pool_keyhash: pool_keyhash,
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

    pub fn add(&mut self, elem: AddrKeyHash) {
        self.0.push(elem);
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

    pub fn add(&mut self, elem: Relay) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolParams {
    operator: PoolKeyHash,
    vrf_keyhash: VrfKeyHash,
    pledge: Coin,
    cost: Coin,
    margin: UnitInterval,
    reward_account: StakeCredential,
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

    pub fn new(operator: PoolKeyHash, vrf_keyhash: VrfKeyHash, pledge: Coin, cost: Coin, margin: UnitInterval, reward_account: StakeCredential, pool_owners: AddrKeyHashes, relays: Relays, pool_metadata: Option<PoolMetadata>) -> Self {
        Self {
            operator: operator,
            vrf_keyhash: vrf_keyhash,
            pledge: pledge,
            cost: cost,
            margin: margin,
            reward_account: reward_account,
            pool_owners: pool_owners,
            relays: relays,
            pool_metadata: pool_metadata,
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

    pub fn new(pool_params: PoolParams) -> Self {
        Self {
            pool_params: pool_params,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolRetirement {
    pool_keyhash: PoolKeyHash,
    epoch: u32,
}

#[wasm_bindgen]
impl PoolRetirement {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolRetirement, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn new(pool_keyhash: PoolKeyHash, epoch: u32) -> Self {
        Self {
            pool_keyhash: pool_keyhash,
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

    pub fn new(genesishash: GenesisHash, genesis_delegate_hash: GenesisDelegateHash) -> Self {
        Self {
            genesishash: genesishash,
            genesis_delegate_hash: genesis_delegate_hash,
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

    pub fn new(move_instantaneous_reward: MoveInstantaneousReward) -> Self {
        Self {
            move_instantaneous_reward: move_instantaneous_reward,
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

    pub fn new_stake_registration(stake_registration: StakeRegistration) -> Self {
        Self(CertificateEnum::StakeRegistration(stake_registration))
    }

    pub fn new_stake_deregistration(stake_deregistration: StakeDeregistration) -> Self {
        Self(CertificateEnum::StakeDeregistration(stake_deregistration))
    }

    pub fn new_stake_delegation(stake_delegation: StakeDelegation) -> Self {
        Self(CertificateEnum::StakeDelegation(stake_delegation))
    }

    pub fn new_pool_registration(pool_registration: PoolRegistration) -> Self {
        Self(CertificateEnum::PoolRegistration(pool_registration))
    }

    pub fn new_pool_retirement(pool_retirement: PoolRetirement) -> Self {
        Self(CertificateEnum::PoolRetirement(pool_retirement))
    }

    pub fn new_genesis_key_delegation(genesis_key_delegation: GenesisKeyDelegation) -> Self {
        Self(CertificateEnum::GenesisKeyDelegation(genesis_key_delegation))
    }

    pub fn new_move_instantaneous_rewards_cert(move_instantaneous_rewards_cert: MoveInstantaneousRewardsCert) -> Self {
        Self(CertificateEnum::MoveInstantaneousRewardsCert(move_instantaneous_rewards_cert))
    }
}

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

    pub fn add(&mut self, elem: TransactionInput) {
        self.0.push(elem);
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

    pub fn add(&mut self, elem: TransactionOutput) {
        self.0.push(elem);
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

    pub fn add(&mut self, elem: Certificate) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

    pub fn set_certs(&mut self, certs: Certificates) {
        self.certs = Some(certs)
    }

    pub fn set_withdrawals(&mut self, withdrawals: Withdrawals) {
        self.withdrawals = Some(withdrawals)
    }
    
    pub fn set_metadata_hash(&mut self, metadata_hash: MetadataHash) {
        self.metadata_hash = Some(metadata_hash)
    }

    pub fn new(inputs: TransactionInputs, outputs: TransactionOutputs, fee: Coin, ttl: u32) -> Self {
        Self {
            inputs,
            outputs,
            fee,
            ttl,
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
        Vkeywitness::new(Vkey::new(sk.to_public()), sig)
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TransactionWitnessSet {
    vkeys: Option<Vkeywitnesses>,
    scripts: Option<MultisigScripts>,
}

#[wasm_bindgen]
impl TransactionWitnessSet {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(self)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionWitnessSet, JsValue> {
        FromBytes::from_bytes(data)
    }

    pub fn set_vkeys(&mut self, vkeys: Vkeywitnesses) {
        self.vkeys = Some(vkeys)
    }

    pub fn set_scripts(&mut self, scripts: MultisigScripts) {
        self.scripts = Some(scripts)
    }

    pub fn new() -> Self {
        Self {
            vkeys: None,
            scripts: None,
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

    pub fn insert(&mut self, key: TransactionMetadatum, value: TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.insert(key, value)
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

    pub fn add(&mut self, elem: TransactionMetadatum) {
        self.0.push(elem);
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

    pub fn new_map_transaction_metadatum_to_transaction_metadatum(map_transaction_metadatum_to_transaction_metadatum: MapTransactionMetadatumToTransactionMetadatum) -> Self {
        Self(TransactionMetadatumEnum::MapTransactionMetadatumToTransactionMetadatum(map_transaction_metadatum_to_transaction_metadatum))
    }

    pub fn new_arr_transaction_metadatum(arr_transaction_metadatum: TransactionMetadatums) -> Self {
        Self(TransactionMetadatumEnum::ArrTransactionMetadatum(arr_transaction_metadatum))
    }

    pub fn new_int(int: Int) -> Self {
        Self(TransactionMetadatumEnum::Int(int))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self(TransactionMetadatumEnum::Bytes(bytes))
    }

    pub fn new_text(text: String) -> Self {
        Self(TransactionMetadatumEnum::Text(text))
    }
}

type TransactionMetadadumLabel = u32;

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

    pub fn insert(&mut self, key: TransactionMetadadumLabel, value: TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.insert(key, value)
    }
}

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

    pub fn new(body: TransactionBody, witness_set: TransactionWitnessSet, metadata: Option<TransactionMetadata>) -> Self {
        Self {
            body,
            witness_set,
            metadata,
        }
    }
}