use std::fmt::Formatter;
use crate::*;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct TransactionInput {
    pub(crate) transaction_id: TransactionHash,
    pub(crate) index: TransactionIndex,
}

impl_to_from!(TransactionInput);

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

impl Display for TransactionInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.transaction_id, self.index)
    }
}
