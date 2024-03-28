use crate::*;

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
#[wasm_bindgen]
pub struct GovernanceActionId {
    pub(crate) transaction_id: TransactionHash,
    pub(crate) index: GovernanceActionIndex,
}

impl_to_from!(GovernanceActionId);

#[wasm_bindgen]
impl GovernanceActionId {
    pub fn transaction_id(&self) -> TransactionHash {
        self.transaction_id.clone()
    }

    pub fn index(&self) -> GovernanceActionIndex {
        self.index.clone()
    }

    pub fn new(transaction_id: &TransactionHash, index: GovernanceActionIndex) -> Self {
        Self {
            transaction_id: transaction_id.clone(),
            index: index,
        }
    }
}
