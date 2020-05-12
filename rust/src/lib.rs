use std::io::{BufRead, Write};
use wasm_bindgen::prelude::*;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::{Deserializer}, se::{Serialize, Serializer}};

mod address;
mod serialization;
mod js_chain_libs;
mod fees;
mod prelude;

use address::Address;

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hash(Vec<u8>);

#[wasm_bindgen]
impl Hash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Keyhash(Vec<u8>);

#[wasm_bindgen]
impl Keyhash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Scripthash(Vec<u8>);

#[wasm_bindgen]
impl Scripthash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Genesishash(Vec<u8>);

#[wasm_bindgen]
impl Genesishash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MetadataHash(Vec<u8>);

#[wasm_bindgen]
impl MetadataHash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vkey(Vec<u8>);

#[wasm_bindgen]
impl Vkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Signature(Vec<u8>);

#[wasm_bindgen]
impl Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct VrfKeyhash(Vec<u8>);

#[wasm_bindgen]
impl VrfKeyhash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct VrfVkey(Vec<u8>);

#[wasm_bindgen]
impl VrfVkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct VrfProof(Vec<u8>);

#[wasm_bindgen]
impl VrfProof {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct KesVkey(Vec<u8>);

#[wasm_bindgen]
impl KesVkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct KesSignature(Vec<u8>);

#[wasm_bindgen]
impl KesSignature {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionInput {
    transaction_id: Hash,
    index: u32,
}

#[wasm_bindgen]
impl TransactionInput {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(transaction_id: Hash, index: u32) -> Self {
        Self {
            transaction_id: transaction_id,
            index: index,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionOutput {
    address: Address,
    amount: u32,
}

#[wasm_bindgen]
impl TransactionOutput {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(address: Address, amount: u32) -> Self {
        Self {
            address: address,
            amount: amount,
        }
    }
}

type Coin = u32;

type Epoch = u32;

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificates(Vec<DelegationCertificate>);

#[wasm_bindgen]
impl DelegationCertificates {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> DelegationCertificate {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: DelegationCertificate) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionBody {
    inputs: TransactionInputs,
    outputs: TransactionOutputs,
    certs: Option<DelegationCertificates>,
    withdrawals: Option<Withdrawals>,
    fee: Coin,
    ttl: u32,
}

#[wasm_bindgen]
impl TransactionBody {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn set_certs(&mut self, certs: DelegationCertificates) {
        self.certs = Some(certs)
    }

    pub fn set_withdrawals(&mut self, withdrawals: Withdrawals) {
        self.withdrawals = Some(withdrawals)
    }

    pub fn new(inputs: TransactionInputs, outputs: TransactionOutputs, fee: Coin, ttl: u32) -> Self {
        Self {
            inputs: inputs,
            outputs: outputs,
            certs: None,
            withdrawals: None,
            fee: fee,
            ttl: ttl,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vkeywitness {
    vkey: Vkey,
    signature: Signature,
}

#[wasm_bindgen]
impl Vkeywitness {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(vkey: Vkey, signature: Signature) -> Self {
        Self {
            vkey: vkey,
            signature: signature,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vkeywitnesss(Vec<Vkeywitness>);

#[wasm_bindgen]
impl Vkeywitnesss {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: Vkeywitness) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Scripts(Vec<Script>);

#[wasm_bindgen]
impl Scripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Script {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: Script) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionWitnessSet {
    key_witnesses: Option<Vkeywitnesss>,
    script_witnesses: Option<Scripts>,
}

#[wasm_bindgen]
impl TransactionWitnessSet {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn set_key_witnesses(&mut self, key_witnesses: Vkeywitnesss) {
        self.key_witnesses = Some(key_witnesses)
    }

    pub fn set_script_witnesses(&mut self, script_witnesses: Scripts) {
        self.script_witnesses = Some(script_witnesses)
    }

    pub fn new() -> Self {
        Self {
            key_witnesses: None,
            script_witnesses: None,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScriptKeyNode {
    keyhash: Keyhash,
}

#[wasm_bindgen]
impl ScriptKeyNode {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(keyhash: Keyhash) -> Self {
        Self {
            keyhash: keyhash,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScriptAllOfNode {
    scripts: Scripts,
}

#[wasm_bindgen]
impl ScriptAllOfNode {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(scripts: Scripts) -> Self {
        Self {
            scripts: scripts,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScriptAnyOfNode {
    scripts: Scripts,
}

#[wasm_bindgen]
impl ScriptAnyOfNode {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(scripts: Scripts) -> Self {
        Self {
            scripts: scripts,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScriptMOfNode {
    m: u32,
    scripts: Scripts,
}

#[wasm_bindgen]
impl ScriptMOfNode {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(m: u32, scripts: Scripts) -> Self {
        Self {
            m: m,
            scripts: scripts,
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum ScriptEnum {
    ScriptKeyNode(ScriptKeyNode),
    ScriptAllOfNode(ScriptAllOfNode),
    ScriptAnyOfNode(ScriptAnyOfNode),
    ScriptMOfNode(ScriptMOfNode),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Script(ScriptEnum);

#[wasm_bindgen]
impl Script {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_script0(keyhash: Keyhash) -> Self {
        Self(ScriptEnum::ScriptKeyNode(ScriptKeyNode::new(keyhash)))
    }

    pub fn new_script1(scripts: Scripts) -> Self {
        Self(ScriptEnum::ScriptAllOfNode(ScriptAllOfNode::new(scripts)))
    }

    pub fn new_script2(scripts: Scripts) -> Self {
        Self(ScriptEnum::ScriptAnyOfNode(ScriptAnyOfNode::new(scripts)))
    }

    pub fn new_script3(m: u32, scripts: Scripts) -> Self {
        Self(ScriptEnum::ScriptMOfNode(ScriptMOfNode::new(m, scripts)))
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Credential0 {
    keyhash: Keyhash,
}

#[wasm_bindgen]
impl Credential0 {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(keyhash: Keyhash) -> Self {
        Self {
            keyhash: keyhash,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Credential1 {
    scripthash: Scripthash,
}

#[wasm_bindgen]
impl Credential1 {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(scripthash: Scripthash) -> Self {
        Self {
            scripthash: scripthash,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Credential2 {
    genesishash: Genesishash,
}

#[wasm_bindgen]
impl Credential2 {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(genesishash: Genesishash) -> Self {
        Self {
            genesishash: genesishash,
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum CredentialEnum {
    Credential0(Credential0),
    Credential1(Credential1),
    Credential2(Credential2),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Credential(CredentialEnum);

#[wasm_bindgen]
impl Credential {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_credential0(keyhash: Keyhash) -> Self {
        Self(CredentialEnum::Credential0(Credential0::new(keyhash)))
    }

    pub fn new_credential1(scripthash: Scripthash) -> Self {
        Self(CredentialEnum::Credential1(Credential1::new(scripthash)))
    }

    pub fn new_credential2(genesishash: Genesishash) -> Self {
        Self(CredentialEnum::Credential2(Credential2::new(genesishash)))
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Credentials(Vec<Credential>);

#[wasm_bindgen]
impl Credentials {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Credential {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: Credential) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Withdrawals {
    table: std::collections::BTreeMap<Credentials, Coin>,
}

#[wasm_bindgen]
impl Withdrawals {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new() -> Self {
        Self {
            table: std::collections::BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: Credentials, value: Coin) {
        self.table.insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}

type UnitInterval = Rational;

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rational {
    numerator: u32,
    denominator: u32,
}

#[wasm_bindgen]
impl Rational {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator: numerator,
            denominator: denominator,
        }
    }
}

type Url = String;

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolMetadata {
    url: Url,
    metadata_hash: MetadataHash,
}

#[wasm_bindgen]
impl PoolMetadata {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(url: Url, metadata_hash: MetadataHash) -> Self {
        Self {
            url: url,
            metadata_hash: metadata_hash,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeKeyReg {
    keyhash: Keyhash,
}

#[wasm_bindgen]
impl StakeKeyReg {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(keyhash: Keyhash) -> Self {
        Self {
            keyhash: keyhash,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeScriptKeyReg {
    scripthash: Scripthash,
}

#[wasm_bindgen]
impl StakeScriptKeyReg {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(scripthash: Scripthash) -> Self {
        Self {
            scripthash: scripthash,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeKeyDereg {
    keyhash: Keyhash,
}

#[wasm_bindgen]
impl StakeKeyDereg {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(keyhash: Keyhash) -> Self {
        Self {
            keyhash: keyhash,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeScriptKeyDereg {
    scripthash: Scripthash,
}

#[wasm_bindgen]
impl StakeScriptKeyDereg {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(scripthash: Scripthash) -> Self {
        Self {
            scripthash: scripthash,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeDeleg {
    deleg_from: Keyhash,
    deleg_to: Keyhash,
}

#[wasm_bindgen]
impl StakeDeleg {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(deleg_from: Keyhash, deleg_to: Keyhash) -> Self {
        Self {
            deleg_from: deleg_from,
            deleg_to: deleg_to,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct StakeScriptDeleg {
    deleg_from: Scripthash,
    deleg_to: Keyhash,
}

#[wasm_bindgen]
impl StakeScriptDeleg {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(deleg_from: Scripthash, deleg_to: Keyhash) -> Self {
        Self {
            deleg_from: deleg_from,
            deleg_to: deleg_to,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Keyhashs(Vec<Keyhash>);

#[wasm_bindgen]
impl Keyhashs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Keyhash {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: Keyhash) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Urls(Vec<Url>);

#[wasm_bindgen]
impl Urls {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Url {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: Url) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolMetadatas(Vec<PoolMetadata>);

#[wasm_bindgen]
impl PoolMetadatas {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PoolMetadata {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: PoolMetadata) {
        self.0.push(elem);
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolParams {
    operator: Keyhash,
    vrf_keyhash: VrfKeyhash,
    cost: Coin,
    pledge: Coin,
    margin: UnitInterval,
    reward_account: Credential,
    owners: Keyhashs,
    relays: Urls,
    metadata: PoolMetadatas,
}

#[wasm_bindgen]
impl PoolParams {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(operator: Keyhash, vrf_keyhash: VrfKeyhash, cost: Coin, pledge: Coin, margin: UnitInterval, reward_account: Credential, owners: Keyhashs, relays: Urls, metadata: PoolMetadatas) -> Self {
        Self {
            operator: operator,
            vrf_keyhash: vrf_keyhash,
            cost: cost,
            pledge: pledge,
            margin: margin,
            reward_account: reward_account,
            owners: owners,
            relays: relays,
            metadata: metadata,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolRegistration {
    keyhash: Keyhash,
    pool_params: PoolParams,
}

#[wasm_bindgen]
impl PoolRegistration {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(keyhash: Keyhash, pool_params: PoolParams) -> Self {
        Self {
            keyhash: keyhash,
            pool_params: pool_params,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolRetirement {
    keyhash: Keyhash,
    epoch: Epoch,
}

#[wasm_bindgen]
impl PoolRetirement {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(keyhash: Keyhash, epoch: Epoch) -> Self {
        Self {
            keyhash: keyhash,
            epoch: epoch,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenesisKeyDeleg {
    deleg_from: Genesishash,
    deleg_to: Keyhash,
}

#[wasm_bindgen]
impl GenesisKeyDeleg {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(deleg_from: Genesishash, deleg_to: Keyhash) -> Self {
        Self {
            deleg_from: deleg_from,
            deleg_to: deleg_to,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoveRewardsCert {
    move_instantaneous_reward: MoveInstantaneousReward,
}

#[wasm_bindgen]
impl MoveRewardsCert {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(move_instantaneous_reward: MoveInstantaneousReward) -> Self {
        Self {
            move_instantaneous_reward: move_instantaneous_reward,
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum DelegationCertificateEnum {
    StakeKeyReg(StakeKeyReg),
    StakeScriptKeyReg(StakeScriptKeyReg),
    StakeKeyDereg(StakeKeyDereg),
    StakeScriptKeyDereg(StakeScriptKeyDereg),
    StakeDeleg(StakeDeleg),
    StakeScriptDeleg(StakeScriptDeleg),
    PoolRegistration(PoolRegistration),
    PoolRetirement(PoolRetirement),
    GenesisKeyDeleg(GenesisKeyDeleg),
    MoveRewardsCert(MoveRewardsCert),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate(DelegationCertificateEnum);

#[wasm_bindgen]
impl DelegationCertificate {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_key_reg(keyhash: Keyhash) -> Self {
        Self(DelegationCertificateEnum::StakeKeyReg(StakeKeyReg::new(keyhash)))
    }

    pub fn new_script_key_reg(scripthash: Scripthash) -> Self {
        Self(DelegationCertificateEnum::StakeScriptKeyReg(StakeScriptKeyReg::new(scripthash)))
    }

    pub fn new_key_dereg(keyhash: Keyhash) -> Self {
        Self(DelegationCertificateEnum::StakeKeyDereg(StakeKeyDereg::new(keyhash)))
    }

    pub fn new_script_key_dereg(scripthash: Scripthash) -> Self {
        Self(DelegationCertificateEnum::StakeScriptKeyDereg(StakeScriptKeyDereg::new(scripthash)))
    }

    pub fn new_delegation(deleg_from: Keyhash, deleg_to: Keyhash) -> Self {
        Self(DelegationCertificateEnum::StakeDeleg(StakeDeleg::new(deleg_from, deleg_to)))
    }

    pub fn new_script_delegation(deleg_from: Scripthash, deleg_to: Keyhash) -> Self {
        Self(DelegationCertificateEnum::StakeScriptDeleg(StakeScriptDeleg::new(deleg_from, deleg_to)))
    }

    pub fn new_pool_reg(keyhash: Keyhash, pool_params: PoolParams) -> Self {
        Self(DelegationCertificateEnum::PoolRegistration(PoolRegistration::new(keyhash, pool_params)))
    }

    pub fn new_pool_retire(keyhash: Keyhash, epoch: Epoch) -> Self {
        Self(DelegationCertificateEnum::PoolRetirement(PoolRetirement::new(keyhash, epoch)))
    }

    pub fn new_genesis_delegate(deleg_from: Genesishash, deleg_to: Keyhash) -> Self {
        Self(DelegationCertificateEnum::GenesisKeyDeleg(GenesisKeyDeleg::new(deleg_from, deleg_to)))
    }

    pub fn new_move_rewards(move_instantaneous_reward: MoveInstantaneousReward) -> Self {
        Self(DelegationCertificateEnum::MoveRewardsCert(MoveRewardsCert::new(move_instantaneous_reward)))
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoveInstantaneousReward {
    table: std::collections::BTreeMap<Keyhash, Coin>,
}

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new() -> Self {
        Self {
            table: std::collections::BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: Keyhash, value: Coin) {
        self.table.insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}
