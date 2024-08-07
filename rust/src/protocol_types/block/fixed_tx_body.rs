use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq)]
/// Read-only view of a transaction body. With correct hash and original bytes.
/// Warning: This is experimental and may be removed in the future.
pub struct FixedTransactionBody {
    pub(crate) body: TransactionBody,
    pub(crate) tx_hash: TransactionHash,
    pub(crate) original_bytes: Vec<u8>,
}

from_bytes!(FixedTransactionBody);
from_hex!(FixedTransactionBody);

#[wasm_bindgen]
impl FixedTransactionBody {

    pub fn transaction_body(&self) -> TransactionBody {
        self.body.clone()
    }

    pub fn tx_hash(&self) -> TransactionHash {
        self.tx_hash.clone()
    }

    pub fn original_bytes(&self) -> Vec<u8> {
        self.original_bytes.clone()
    }
}