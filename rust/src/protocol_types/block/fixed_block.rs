use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq)]
/// Read only view of a block with more strict structs for hash sensitive structs.
/// Warning: This is experimental and may be removed or changed in the future.
pub struct FixedBlock {
    pub(crate) header: Header,
    pub(crate) transaction_bodies: FixedTransactionBodies,
    pub(crate) transaction_witness_sets: TransactionWitnessSets,
    pub(crate) auxiliary_data_set: AuxiliaryDataSet,
    pub(crate) invalid_transactions: TransactionIndexes,
    pub(crate) block_hash: BlockHash,
}

from_bytes!(FixedBlock);

#[wasm_bindgen]
impl FixedBlock {
    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn transaction_bodies(&self) -> FixedTransactionBodies {
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

    pub fn block_hash(&self) -> BlockHash {
        self.block_hash.clone()
    }
}