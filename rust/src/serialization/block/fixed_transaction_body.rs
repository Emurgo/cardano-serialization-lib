use crate::*;
use crate::serialization::utils::{deserilized_with_orig_bytes};

impl Deserialize for FixedTransactionBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (body, orig_bytes) = deserilized_with_orig_bytes(raw, |raw| -> Result<_, DeserializeError> {
            let body = TransactionBody::deserialize(raw)?;
            Ok(body)
        }).map_err(|e| e.annotate("TransactionBody"))?;
        let hash = TransactionHash(blake2b256(orig_bytes.as_ref()));
        Ok(FixedTransactionBody {
            body,
            tx_hash: hash,
            original_bytes: orig_bytes,
        })
    }
}
