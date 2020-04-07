use std::io::Write;
use wasm_bindgen::prelude::*;
use prelude::*;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cardano-serialization-lib/tree/master/cddl_test

use cbor_event::{self, de::{Deserialize, Deserializer}, se::{Serialize, Serializer}};

mod prelude;

mod groups;

mod js_chain_libs;

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hash {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for Hash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Hash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Keyhash {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for Keyhash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Keyhash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Scripthash {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for Scripthash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Scripthash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Genesishash {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for Genesishash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Genesishash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vkey {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for Vkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Vkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Signature {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for Signature {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    // we probably don't want to create one directly from bytes, do we?
    // pub fn new(data: Vec<u8>) -> Self {
    //     Self {
    //         data: data,
    //     }
    // }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct VrfKeyhash {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for VrfKeyhash {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl VrfKeyhash {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct VrfVkey {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for VrfVkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl VrfVkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct VrfProof {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for VrfProof {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl VrfProof {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct KesVkey {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for KesVkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl KesVkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct KesSignature {
    data: Vec<u8>,
}

impl cbor_event::se::Serialize for KesSignature {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Array(Primitive("u8"))
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl KesSignature {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionInput {
    group: groups::TransactionInput,
}

impl cbor_event::se::Serialize for TransactionInput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::TransactionInput> for TransactionInput {
    fn from(group: groups::TransactionInput) -> Self {
        TransactionInput { group: group }
    }
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
            group: groups::TransactionInput::new(transaction_id, index)
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pointer {
    group: groups::Pointer,
}

impl cbor_event::se::Serialize for Pointer {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::Pointer> for Pointer {
    fn from(group: groups::Pointer) -> Self {
        Pointer { group: group }
    }
}

#[wasm_bindgen]

impl Pointer {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(index_0: u32, index_1: u32, index_2: u32) -> Self {
        Self {
            group: groups::Pointer::new(index_0, index_1, index_2)
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Address {
    group: groups::Address,
}

impl cbor_event::se::Serialize for Address {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::Address> for Address {
    fn from(group: groups::Address) -> Self {
        Address { group: group }
    }
}

#[wasm_bindgen]

impl Address {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_base(spending: Keyhash, deleg: Keyhash) -> Self {
        Self {
            group: groups::Address::Address0(groups::Address0::new(spending, deleg))
        }
    }

    pub fn new_base_with_multisig_delegation(spending: Keyhash, deleg: Scripthash) -> Self {
        Self {
            group: groups::Address::Address1(groups::Address1::new(spending, deleg))
        }
    }

    pub fn new_multisig_base_delegation(spending: Scripthash, deleg: Keyhash) -> Self {
        Self {
            group: groups::Address::Address2(groups::Address2::new(spending, deleg))
        }
    }

    pub fn new_multisig(spending: Scripthash, deleg: Scripthash) -> Self {
        Self {
            group: groups::Address::Address3(groups::Address3::new(spending, deleg))
        }
    }

    pub fn new_base_pointer(spending: Keyhash, deleg: Pointer) -> Self {
        Self {
            group: groups::Address::Address4(groups::Address4::new(spending, deleg))
        }
    }

    pub fn new_multisig_pointer(spending: Scripthash, deleg: Pointer) -> Self {
        Self {
            group: groups::Address::Address5(groups::Address5::new(spending, deleg))
        }
    }

    pub fn new_enterprise(spending: Keyhash) -> Self {
        Self {
            group: groups::Address::Address6(groups::Address6::new(spending))
        }
    }

    pub fn new_multisig_enterprise(spending: Scripthash) -> Self {
        Self {
            group: groups::Address::Address7(groups::Address7::new(spending))
        }
    }

    pub fn new_bootstrap(spending: Keyhash) -> Self {
        Self {
            group: groups::Address::Address8(groups::Address8::new(spending))
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionOutput {
    group: groups::TransactionOutput,
}

impl cbor_event::se::Serialize for TransactionOutput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::TransactionOutput> for TransactionOutput {
    fn from(group: groups::TransactionOutput) -> Self {
        TransactionOutput { group: group }
    }
}

#[wasm_bindgen]

impl TransactionOutput {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(address: Address, amount: Coin) -> Self {
        Self {
            group: groups::TransactionOutput::new(address, amount)
        }
    }
}

type Coin = u32;

type Epoch = u32;

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionInputs {
    data: Vec<TransactionInput>,
}

impl cbor_event::se::Serialize for TransactionInputs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl TransactionInputs {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.data[index].clone()
    }

    pub fn add(&mut self, elem: TransactionInput) {
        self.data.push(elem);
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionOutputs {
    data: Vec<TransactionOutput>,
}

impl cbor_event::se::Serialize for TransactionOutputs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl TransactionOutputs {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> TransactionOutput {
        self.data[index].clone()
    }

    pub fn add(&mut self, elem: TransactionOutput) {
        self.data.push(elem);
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificates {
    data: Vec<DelegationCertificate>,
}

impl cbor_event::se::Serialize for DelegationCertificates {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl DelegationCertificates {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> DelegationCertificate {
        self.data[index].clone()
    }

    pub fn add(&mut self, elem: DelegationCertificate) {
        self.data.push(elem);
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionBody {
    group: groups::TransactionBody,
}

impl cbor_event::se::Serialize for TransactionBody {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

impl From<groups::TransactionBody> for TransactionBody {
    fn from(group: groups::TransactionBody) -> Self {
        TransactionBody { group: group }
    }
}

#[wasm_bindgen]

impl TransactionBody {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(inputs: TransactionInputs, outputs: TransactionOutputs, fee: Coin, ttl: u32) -> Self {
        Self {
            group: groups::TransactionBody::new(TaggedData::<TransactionInputs>::new(inputs, 258), outputs, fee, ttl)
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vkeywitness {
    group: groups::Vkeywitness,
}

impl cbor_event::se::Serialize for Vkeywitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::Vkeywitness> for Vkeywitness {
    fn from(group: groups::Vkeywitness) -> Self {
        Vkeywitness { group: group }
    }
}

#[wasm_bindgen]

impl Vkeywitness {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(index_0: Vkey, index_1: Signature) -> Self {
        Self {
            group: groups::Vkeywitness::new(index_0, index_1)
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vkeywitnesss {
    data: Vec<Vkeywitness>,
}

impl cbor_event::se::Serialize for Vkeywitnesss {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Vkeywitnesss {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.data[index].clone()
    }

    pub fn add(&mut self, elem: Vkeywitness) {
        self.data.push(elem);
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Scripts {
    data: Vec<Script>,
}

impl cbor_event::se::Serialize for Scripts {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Scripts {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Script {
        self.data[index].clone()
    }

    pub fn add(&mut self, elem: Script) {
        self.data.push(elem);
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionWitnessSet {
    group: groups::TransactionWitnessSet,
}

impl cbor_event::se::Serialize for TransactionWitnessSet {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

impl From<groups::TransactionWitnessSet> for TransactionWitnessSet {
    fn from(group: groups::TransactionWitnessSet) -> Self {
        TransactionWitnessSet { group: group }
    }
}

#[wasm_bindgen]

impl TransactionWitnessSet {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new() -> Self {
        Self {
            group: groups::TransactionWitnessSet::new()
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Script {
    group: groups::Script,
}

impl cbor_event::se::Serialize for Script {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::Script> for Script {
    fn from(group: groups::Script) -> Self {
        Script { group: group }
    }
}

#[wasm_bindgen]

impl Script {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_script0(index_1: Keyhash) -> Self {
        Self {
            group: groups::Script::Script0(groups::Script0::new(index_1))
        }
    }

    pub fn new_script1(index_1: Scripts) -> Self {
        Self {
            group: groups::Script::Script1(groups::Script1::new(index_1))
        }
    }

    pub fn new_script2(index_1: Scripts) -> Self {
        Self {
            group: groups::Script::Script2(groups::Script2::new(index_1))
        }
    }

    pub fn new_script3(index_1: u32, index_2: Scripts) -> Self {
        Self {
            group: groups::Script::Script3(groups::Script3::new(index_1, index_2))
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Credential {
    group: groups::Credential,
}

impl cbor_event::se::Serialize for Credential {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::Credential> for Credential {
    fn from(group: groups::Credential) -> Self {
        Credential { group: group }
    }
}

#[wasm_bindgen]

impl Credential {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_credential0(index_1: Keyhash) -> Self {
        Self {
            group: groups::Credential::Credential0(groups::Credential0::new(index_1))
        }
    }

    pub fn new_credential1(index_1: Scripthash) -> Self {
        Self {
            group: groups::Credential::Credential1(groups::Credential1::new(index_1))
        }
    }

    pub fn new_credential2(index_1: Genesishash) -> Self {
        Self {
            group: groups::Credential::Credential2(groups::Credential2::new(index_1))
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Credentials {
    data: Vec<Credential>,
}

impl cbor_event::se::Serialize for Credentials {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Credentials {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Credential {
        self.data[index].clone()
    }

    pub fn add(&mut self, elem: Credential) {
        self.data.push(elem);
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Withdrawals {
    group: groups::Withdrawals,
}

impl cbor_event::se::Serialize for Withdrawals {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

impl From<groups::Withdrawals> for Withdrawals {
    fn from(group: groups::Withdrawals) -> Self {
        Withdrawals { group: group }
    }
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
            group: groups::Withdrawals::new(),
        }
    }

    pub fn insert(&mut self, key: Credentials, value: Coin) {
        self.group.table.insert(key, value);
    }
}

type UnitInterval = Rational;

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct UntaggedRational {
    group: groups::UntaggedRational,
}

impl cbor_event::se::Serialize for UntaggedRational {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::UntaggedRational> for UntaggedRational {
    fn from(group: groups::UntaggedRational) -> Self {
        UntaggedRational { group: group }
    }
}

#[wasm_bindgen]

impl UntaggedRational {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            group: groups::UntaggedRational::new(numerator, denominator)
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rational {
    data: TaggedData<UntaggedRational>,
}

impl cbor_event::se::Serialize for Rational {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Tagged(30, Rust("UntaggedRational"))
        serializer.write_tag(30u64)?;
        // DEBUG - generated from: Rust("UntaggedRational")
        self.data.data.serialize(serializer)?;
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Rational {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(data: UntaggedRational) -> Self {
        Self {
            data: TaggedData::<UntaggedRational>::new(data, 30),
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Keyhashs {
    data: Vec<Keyhash>,
}

impl cbor_event::se::Serialize for Keyhashs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.data.len() as u64))?;
        for element in &self.data {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

#[wasm_bindgen]

impl Keyhashs {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Keyhash {
        self.data[index].clone()
    }

    pub fn add(&mut self, elem: Keyhash) {
        self.data.push(elem);
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PoolParams {
    group: groups::PoolParams,
}

impl cbor_event::se::Serialize for PoolParams {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::PoolParams> for PoolParams {
    fn from(group: groups::PoolParams) -> Self {
        PoolParams { group: group }
    }
}

#[wasm_bindgen]

impl PoolParams {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new(index_0: Keyhashs, index_1: Coin, index_2: UnitInterval, index_3: Coin, index_4: Keyhash, index_5: VrfKeyhash, index_6: Credentials) -> Self {
        Self {
            group: groups::PoolParams::new(TaggedData::<Keyhashs>::new(index_0, 258), index_1, index_2, index_3, index_4, index_5, index_6)
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DelegationCertificate {
    group: groups::DelegationCertificate,
}

impl cbor_event::se::Serialize for DelegationCertificate {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_array(serializer)
    }
}

impl From<groups::DelegationCertificate> for DelegationCertificate {
    fn from(group: groups::DelegationCertificate) -> Self {
        DelegationCertificate { group: group }
    }
}

#[wasm_bindgen]

impl DelegationCertificate {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    pub fn new_delegation_certificate0(index_1: Keyhash) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate0(groups::DelegationCertificate0::new(index_1))
        }
    }

    pub fn new_delegation_certificate1(index_1: Scripthash) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate1(groups::DelegationCertificate1::new(index_1))
        }
    }

    pub fn new_delegation_certificate2(index_1: Keyhash) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate2(groups::DelegationCertificate2::new(index_1))
        }
    }

    pub fn new_delegation_certificate3(index_1: Scripthash) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate3(groups::DelegationCertificate3::new(index_1))
        }
    }

    pub fn new_delegation_certificate4(index_1: Keyhash, index_2: Keyhash) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate4(groups::DelegationCertificate4::new(index_1, index_2))
        }
    }

    pub fn new_delegation_certificate5(index_1: Scripthash, index_2: Keyhash) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate5(groups::DelegationCertificate5::new(index_1, index_2))
        }
    }

    pub fn new_delegation_certificate6(index_1: Keyhash, index_2: PoolParams) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate6(groups::DelegationCertificate6::new(index_1, index_2))
        }
    }

    pub fn new_delegation_certificate7(index_1: Keyhash, index_2: Epoch) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate7(groups::DelegationCertificate7::new(index_1, index_2))
        }
    }

    pub fn new_delegation_certificate8(index_1: Genesishash, index_2: Keyhash) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate8(groups::DelegationCertificate8::new(index_1, index_2))
        }
    }

    pub fn new_delegation_certificate9(index_1: MoveInstantaneousReward) -> Self {
        Self {
            group: groups::DelegationCertificate::DelegationCertificate9(groups::DelegationCertificate9::new(index_1))
        }
    }
}

#[wasm_bindgen]

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MoveInstantaneousReward {
    group: groups::MoveInstantaneousReward,
}

impl cbor_event::se::Serialize for MoveInstantaneousReward {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.group.serialize_as_map(serializer)
    }
}

impl From<groups::MoveInstantaneousReward> for MoveInstantaneousReward {
    fn from(group: groups::MoveInstantaneousReward) -> Self {
        MoveInstantaneousReward { group: group }
    }
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
            group: groups::MoveInstantaneousReward::new(),
        }
    }

    pub fn insert(&mut self, key: Keyhash, value: Coin) {
        self.group.table.insert(key, value);
    }
}