use crate::*;
#[allow(dead_code)]

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FixedTxWitnessesSet {
    pub(crate) raw_parts: TransactionWitnessSetRaw,
    pub(crate) tx_witnesses_set: TransactionWitnessSet,
}

impl FixedTxWitnessesSet {
    pub(crate) fn tx_witnesses_set(&self) -> TransactionWitnessSet {
        self.tx_witnesses_set.clone()
    }

    pub(crate) fn add_vkey_witness(&mut self, vkey_witness: Vkeywitness) {
        if self.tx_witnesses_set.vkeys.is_none() {
            self.tx_witnesses_set.vkeys = Some(Vkeywitnesses::new());
        }
        if let Some(vkeys) = &mut self.tx_witnesses_set.vkeys {
            vkeys.add(&vkey_witness);
        }
        self.raw_parts.vkeys = None;
    }

    pub(crate) fn add_bootstrap_witness(&mut self, bootstrap_witness: BootstrapWitness) {
        if self.tx_witnesses_set.bootstraps.is_none() {
            self.tx_witnesses_set.bootstraps = Some(BootstrapWitnesses::new());
        }
        if let Some(bootstraps) = &mut self.tx_witnesses_set.bootstraps {
            bootstraps.add(&bootstrap_witness);
        }
        self.raw_parts.bootstraps = None;
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }

    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub(crate) fn from_bytes(data: Vec<u8>) -> Result<FixedTxWitnessesSet, DeserializeError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(data));
        Self::deserialize(&mut raw)
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub(crate) fn from_bytes(data: Vec<u8>) -> Result<FixedTxWitnessesSet, JsError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(data));
        Ok(Self::deserialize(&mut raw)?)
    }
}