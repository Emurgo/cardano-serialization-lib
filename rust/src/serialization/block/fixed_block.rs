use crate::serialization::utils::{deserilized_with_orig_bytes, is_break_tag};
use crate::*;

impl Deserialize for FixedBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        let (header, header_bytes) =
            deserilized_with_orig_bytes(raw, |raw| -> Result<_, DeserializeError> {
                Ok(Header::deserialize(raw)?)
            })
            .map_err(|e| e.annotate("header"))?;
        let transaction_bodies =
            (|| -> Result<_, DeserializeError> { Ok(FixedTransactionBodies::deserialize(raw)?) })()
                .map_err(|e| e.annotate("fixed_transaction_bodies"))?;
        let transaction_witness_sets =
            (|| -> Result<_, DeserializeError> { Ok(TransactionWitnessSets::deserialize(raw)?) })()
                .map_err(|e| e.annotate("transaction_witness_sets"))?;
        let auxiliary_data_set =
            (|| -> Result<_, DeserializeError> { Ok(AuxiliaryDataSet::deserialize(raw)?) })()
                .map_err(|e| e.annotate("auxiliary_data_set"))?;
        let invalid_present = match len {
            Len::Indefinite => raw.cbor_type()? == CBORType::Array,
            Len::Len(4) => false,
            _ => true,
        };
        let invalid_transactions = (|| -> Result<_, DeserializeError> {
            let mut arr = Vec::new();
            if invalid_present {
                read_len.read_elems(1)?;
                let len = raw.array()?;
                while match len {
                    Len::Len(n) => arr.len() < n as usize,
                    Len::Indefinite => true,
                } {
                    if is_break_tag(raw, "invalid_transactions")? {
                        break;
                    }
                    arr.push(TransactionIndex::deserialize(raw)?);
                }
            }
            Ok(arr)
        })()
        .map_err(|e| e.annotate("invalid_transactions"))?;
        match len {
            Len::Len(_) => (),
            Len::Indefinite => match raw.special()? {
                CBORSpecial::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        let block_hash = BlockHash(blake2b256(header_bytes.as_ref()));
        Ok(FixedBlock {
            header,
            transaction_bodies,
            transaction_witness_sets,
            auxiliary_data_set,
            invalid_transactions,
            block_hash,
        })
    }
}
