use crate::*;
#[allow(dead_code)]


/// A set of witnesses for a transaction.
/// Keeps original bytes to allow for safe roundtrip serialization.
/// That helps to avoid incorrect script data hash after adding a  vkey or bootstrap witness.
/// You can add a vkey witness or a bootstrap witness to the set.
/// Or get TransactionWitnessSet to read fields.
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedTxWitnessesSet {
    raw_parts: TransactionWitnessSetRaw,
    tx_witnesses_set: TransactionWitnessSet,
}

#[wasm_bindgen]
impl FixedTxWitnessesSet {

    pub(crate) fn new(mut witnesses_set: TransactionWitnessSet, raw_parts: TransactionWitnessSetRaw) -> Self {
        if let Some(bootstraps) = &mut witnesses_set.bootstraps {
            bootstraps.set_force_original_cbor_set_type(true);
        }
        if let Some(vkeys) = &mut witnesses_set.vkeys {
            vkeys.set_force_original_cbor_set_type(true);
        }
        Self {
            tx_witnesses_set: witnesses_set,
            raw_parts,
        }
    }

    pub(crate) fn new_empty() -> Self {
        Self {
            tx_witnesses_set: TransactionWitnessSet::new(),
            raw_parts: TransactionWitnessSetRaw::new(),
        }
    }

    pub fn tx_witnesses_set(&self) -> TransactionWitnessSet {
        self.tx_witnesses_set.clone()
    }

    pub fn add_vkey_witness(&mut self, vkey_witness: &Vkeywitness) {
        if self.tx_witnesses_set.vkeys.is_none() {
            self.tx_witnesses_set.vkeys = Some(Vkeywitnesses::new());
        }
        if let Some(vkeys) = &mut self.tx_witnesses_set.vkeys {
            vkeys.add(vkey_witness);
        }
        self.raw_parts.vkeys = None;
    }

    pub fn add_bootstrap_witness(&mut self, bootstrap_witness: &BootstrapWitness) {
        if self.tx_witnesses_set.bootstraps.is_none() {
            self.tx_witnesses_set.bootstraps = Some(BootstrapWitnesses::new());
        }
        if let Some(bootstraps) = &mut self.tx_witnesses_set.bootstraps {
            bootstraps.add(bootstrap_witness);
        }
        self.raw_parts.bootstraps = None;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub fn from_bytes(data: Vec<u8>) -> Result<FixedTxWitnessesSet, DeserializeError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(data));
        Self::deserialize(&mut raw)
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn from_bytes(data: Vec<u8>) -> Result<FixedTxWitnessesSet, JsError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(data));
        Ok(Self::deserialize(&mut raw)?)
    }

    pub(crate) fn tx_witnesses_set_ref(&self) -> &TransactionWitnessSet {
        &self.tx_witnesses_set
    }

    pub(crate) fn raw_parts_ref(&self) -> &TransactionWitnessSetRaw {
        &self.raw_parts
    }
}