use std::io::Write;
use wasm_bindgen::prelude::*;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::{Deserialize, Deserializer}, se::{Serialize, Serializer}};

mod serialization;

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BaseAddress {
    spending: Keyhash,
    deleg: Keyhash,
}

#[wasm_bindgen]
impl BaseAddress {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Keyhash, deleg: Keyhash) -> Self {
        Self {
            spending: spending,
            deleg: deleg,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BaseAddressScriptDelegation {
    spending: Keyhash,
    deleg: Scripthash,
}

#[wasm_bindgen]
impl BaseAddressScriptDelegation {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Keyhash, deleg: Scripthash) -> Self {
        Self {
            spending: spending,
            deleg: deleg,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BaseScriptAddress {
    spending: Scripthash,
    deleg: Keyhash,
}

#[wasm_bindgen]
impl BaseScriptAddress {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Scripthash, deleg: Keyhash) -> Self {
        Self {
            spending: spending,
            deleg: deleg,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BaseScriptAddressScriptDeleg {
    spending: Scripthash,
    deleg: Scripthash,
}

#[wasm_bindgen]
impl BaseScriptAddressScriptDeleg {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Scripthash, deleg: Scripthash) -> Self {
        Self {
            spending: spending,
            deleg: deleg,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pointer {
    slot: u32,
    tx_index: u32,
    cert_index: u32,
}

#[wasm_bindgen]
impl Pointer {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(slot: u32, tx_index: u32, cert_index: u32) -> Self {
        Self {
            slot: slot,
            tx_index: tx_index,
            cert_index: cert_index,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PointerAddress {
    spending: Keyhash,
    deleg: Pointer,
}

#[wasm_bindgen]
impl PointerAddress {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Keyhash, deleg: Pointer) -> Self {
        Self {
            spending: spending,
            deleg: deleg,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PointerMultisigAddress {
    spending: Scripthash,
    deleg: Pointer,
}

#[wasm_bindgen]
impl PointerMultisigAddress {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Scripthash, deleg: Pointer) -> Self {
        Self {
            spending: spending,
            deleg: deleg,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnterpriseAddress {
    spending: Keyhash,
}

#[wasm_bindgen]
impl EnterpriseAddress {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Keyhash) -> Self {
        Self {
            spending: spending,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnterpriseMultisigAddress {
    spending: Scripthash,
}

#[wasm_bindgen]
impl EnterpriseMultisigAddress {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Scripthash) -> Self {
        Self {
            spending: spending,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BootstrapAddress {
    spending: Keyhash,
}

#[wasm_bindgen]
impl BootstrapAddress {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(spending: Keyhash) -> Self {
        Self {
            spending: spending,
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum AddressEnum {
    BaseAddress(BaseAddress),
    BaseAddressScriptDelegation(BaseAddressScriptDelegation),
    BaseScriptAddress(BaseScriptAddress),
    BaseScriptAddressScriptDeleg(BaseScriptAddressScriptDeleg),
    PointerAddress(PointerAddress),
    PointerMultisigAddress(PointerMultisigAddress),
    EnterpriseAddress(EnterpriseAddress),
    EnterpriseMultisigAddress(EnterpriseMultisigAddress),
    BootstrapAddress(BootstrapAddress),
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Address(AddressEnum);

#[wasm_bindgen]
impl Address {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_base_(spending: Keyhash, deleg: Keyhash) -> Self {
        Self(AddressEnum::BaseAddress(BaseAddress::new(spending, deleg)))
    }

    pub fn new_base_with_script_delegation(spending: Keyhash, deleg: Scripthash) -> Self {
        Self(AddressEnum::BaseAddressScriptDelegation(BaseAddressScriptDelegation::new(spending, deleg)))
    }

    pub fn new_base_script(spending: Scripthash, deleg: Keyhash) -> Self {
        Self(AddressEnum::BaseScriptAddress(BaseScriptAddress::new(spending, deleg)))
    }

    pub fn new_base_script_with_script_deleg(spending: Scripthash, deleg: Scripthash) -> Self {
        Self(AddressEnum::BaseScriptAddressScriptDeleg(BaseScriptAddressScriptDeleg::new(spending, deleg)))
    }

    pub fn new_pointer(spending: Keyhash, deleg: Pointer) -> Self {
        Self(AddressEnum::PointerAddress(PointerAddress::new(spending, deleg)))
    }

    pub fn new_pointer_multisig(spending: Scripthash, deleg: Pointer) -> Self {
        Self(AddressEnum::PointerMultisigAddress(PointerMultisigAddress::new(spending, deleg)))
    }

    pub fn new_enterprise(spending: Keyhash) -> Self {
        Self(AddressEnum::EnterpriseAddress(EnterpriseAddress::new(spending)))
    }

    pub fn new_enterprise_multisig(spending: Scripthash) -> Self {
        Self(AddressEnum::EnterpriseMultisigAddress(EnterpriseMultisigAddress::new(spending)))
    }

    pub fn new_bootstrap(spending: Keyhash) -> Self {
        Self(AddressEnum::BootstrapAddress(BootstrapAddress::new(spending)))
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionInputs(Vec<TransactionInput>);

#[wasm_bindgen]
impl TransactionInputs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn size(&self) -> usize {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionOutputs(Vec<TransactionOutput>);

#[wasm_bindgen]
impl TransactionOutputs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn size(&self) -> usize {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificates(Vec<DelegationCertificate>);

#[wasm_bindgen]
impl DelegationCertificates {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn size(&self) -> usize {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vkeywitnesss(Vec<Vkeywitness>);

#[wasm_bindgen]
impl Vkeywitnesss {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn size(&self) -> usize {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Scripts(Vec<Script>);

#[wasm_bindgen]
impl Scripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn size(&self) -> usize {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Script0 {
    keyhash: Keyhash,
}

#[wasm_bindgen]
impl Script0 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Script1 {
    scripts: Scripts,
}

#[wasm_bindgen]
impl Script1 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Script2 {
    scripts: Scripts,
}

#[wasm_bindgen]
impl Script2 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Script3 {
    m: u32,
    scripts: Scripts,
}

#[wasm_bindgen]
impl Script3 {
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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum ScriptEnum {
    Script0(Script0),
    Script1(Script1),
    Script2(Script2),
    Script3(Script3),
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Script(ScriptEnum);

#[wasm_bindgen]
impl Script {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_script0(keyhash: Keyhash) -> Self {
        Self(ScriptEnum::Script0(Script0::new(keyhash)))
    }

    pub fn new_script1(scripts: Scripts) -> Self {
        Self(ScriptEnum::Script1(Script1::new(scripts)))
    }

    pub fn new_script2(scripts: Scripts) -> Self {
        Self(ScriptEnum::Script2(Script2::new(scripts)))
    }

    pub fn new_script3(m: u32, scripts: Scripts) -> Self {
        Self(ScriptEnum::Script3(Script3::new(m, scripts)))
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum CredentialEnum {
    Credential0(Credential0),
    Credential1(Credential1),
    Credential2(Credential2),
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Credentials(Vec<Credential>);

#[wasm_bindgen]
impl Credentials {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn size(&self) -> usize {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
}

type UnitInterval = Rational;

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate0 {
    keyhash: Keyhash,
}

#[wasm_bindgen]
impl DelegationCertificate0 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate1 {
    scripthash: Scripthash,
}

#[wasm_bindgen]
impl DelegationCertificate1 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate2 {
    keyhash: Keyhash,
}

#[wasm_bindgen]
impl DelegationCertificate2 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate3 {
    scripthash: Scripthash,
}

#[wasm_bindgen]
impl DelegationCertificate3 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate4 {
    deleg_from: Keyhash,
    deleg_to: Keyhash,
}

#[wasm_bindgen]
impl DelegationCertificate4 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate5 {
    deleg_from: Scripthash,
    deleg_to: Keyhash,
}

#[wasm_bindgen]
impl DelegationCertificate5 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Keyhashs(Vec<Keyhash>);

#[wasm_bindgen]
impl Keyhashs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn size(&self) -> usize {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolParams {
    owners: Keyhashs,
    cost: Coin,
    margin: UnitInterval,
    pledge: Coin,
    operator: Keyhash,
    vrf_keyhash: VrfKeyhash,
    reward_account: Credentials,
}

#[wasm_bindgen]
impl PoolParams {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(owners: Keyhashs, cost: Coin, margin: UnitInterval, pledge: Coin, operator: Keyhash, vrf_keyhash: VrfKeyhash, reward_account: Credentials) -> Self {
        Self {
            owners: owners,
            cost: cost,
            margin: margin,
            pledge: pledge,
            operator: operator,
            vrf_keyhash: vrf_keyhash,
            reward_account: reward_account,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate6 {
    keyhash: Keyhash,
    pool_params: PoolParams,
}

#[wasm_bindgen]
impl DelegationCertificate6 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate7 {
    keyhash: Keyhash,
    epoch: Epoch,
}

#[wasm_bindgen]
impl DelegationCertificate7 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate8 {
    deleg_from: Genesishash,
    deleg_to: Keyhash,
}

#[wasm_bindgen]
impl DelegationCertificate8 {
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
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate9 {
    move_instantaneous_reward: MoveInstantaneousReward,
}

#[wasm_bindgen]
impl DelegationCertificate9 {
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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum DelegationCertificateEnum {
    DelegationCertificate0(DelegationCertificate0),
    DelegationCertificate1(DelegationCertificate1),
    DelegationCertificate2(DelegationCertificate2),
    DelegationCertificate3(DelegationCertificate3),
    DelegationCertificate4(DelegationCertificate4),
    DelegationCertificate5(DelegationCertificate5),
    DelegationCertificate6(DelegationCertificate6),
    DelegationCertificate7(DelegationCertificate7),
    DelegationCertificate8(DelegationCertificate8),
    DelegationCertificate9(DelegationCertificate9),
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate(DelegationCertificateEnum);

#[wasm_bindgen]
impl DelegationCertificate {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_delegation_certificate0(keyhash: Keyhash) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate0(DelegationCertificate0::new(keyhash)))
    }

    pub fn new_delegation_certificate1(scripthash: Scripthash) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate1(DelegationCertificate1::new(scripthash)))
    }

    pub fn new_delegation_certificate2(keyhash: Keyhash) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate2(DelegationCertificate2::new(keyhash)))
    }

    pub fn new_delegation_certificate3(scripthash: Scripthash) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate3(DelegationCertificate3::new(scripthash)))
    }

    pub fn new_delegation_certificate4(deleg_from: Keyhash, deleg_to: Keyhash) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate4(DelegationCertificate4::new(deleg_from, deleg_to)))
    }

    pub fn new_delegation_certificate5(deleg_from: Scripthash, deleg_to: Keyhash) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate5(DelegationCertificate5::new(deleg_from, deleg_to)))
    }

    pub fn new_delegation_certificate6(keyhash: Keyhash, pool_params: PoolParams) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate6(DelegationCertificate6::new(keyhash, pool_params)))
    }

    pub fn new_delegation_certificate7(keyhash: Keyhash, epoch: Epoch) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate7(DelegationCertificate7::new(keyhash, epoch)))
    }

    pub fn new_delegation_certificate8(deleg_from: Genesishash, deleg_to: Keyhash) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate8(DelegationCertificate8::new(deleg_from, deleg_to)))
    }

    pub fn new_delegation_certificate9(move_instantaneous_reward: MoveInstantaneousReward) -> Self {
        Self(DelegationCertificateEnum::DelegationCertificate9(DelegationCertificate9::new(move_instantaneous_reward)))
    }
}

#[wasm_bindgen]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
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
}