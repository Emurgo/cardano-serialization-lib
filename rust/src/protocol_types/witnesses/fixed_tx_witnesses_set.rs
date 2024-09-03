use crate::*;
#[allow(dead_code)]

pub(crate) struct FixedTxWitnessesSet {
    pub(crate) raw_parts: TransactionWitnessSetRaw,
    pub(crate) tx_witnesses_set: TransactionWitnessSet,
}

to_from_bytes!(FixedTxWitnessesSet);

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
}