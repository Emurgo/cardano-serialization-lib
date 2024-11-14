use crate::*;

pub type TransactionIndexes = Vec<TransactionIndex>;

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Block {
    pub(crate) header: Header,
    pub(crate) transaction_bodies: TransactionBodies,
    pub(crate) transaction_witness_sets: TransactionWitnessSets,
    pub(crate) auxiliary_data_set: AuxiliaryDataSet,
    pub(crate) invalid_transactions: TransactionIndexes,
}

impl_to_from!(Block);

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

    pub fn new(
        header: &Header,
        transaction_bodies: &TransactionBodies,
        transaction_witness_sets: &TransactionWitnessSets,
        auxiliary_data_set: &AuxiliaryDataSet,
        invalid_transactions: TransactionIndexes,
    ) -> Self {
        Self {
            header: header.clone(),
            transaction_bodies: transaction_bodies.clone(),
            transaction_witness_sets: transaction_witness_sets.clone(),
            auxiliary_data_set: auxiliary_data_set.clone(),
            invalid_transactions: invalid_transactions,
        }
    }
}