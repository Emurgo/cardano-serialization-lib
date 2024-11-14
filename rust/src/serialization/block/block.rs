use crate::*;
use crate::serialization::utils::is_break_tag;

impl cbor_event::se::Serialize for Block {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(5))?;
        self.header.serialize(serializer)?;
        self.transaction_bodies.serialize(serializer)?;
        self.transaction_witness_sets.serialize(serializer)?;
        self.auxiliary_data_set.serialize(serializer)?;
        serializer.write_array(cbor_event::Len::Len(self.invalid_transactions.len() as u64))?;
        for element in self.invalid_transactions.iter() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Block {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let header = (|| -> Result<_, DeserializeError> { Ok(Header::deserialize(raw)?) })()
                .map_err(|e| e.annotate("header"))?;
            let transaction_bodies =
                (|| -> Result<_, DeserializeError> { Ok(TransactionBodies::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("transaction_bodies"))?;
            let transaction_witness_sets = (|| -> Result<_, DeserializeError> {
                Ok(TransactionWitnessSets::deserialize(raw)?)
            })()
                .map_err(|e| e.annotate("transaction_witness_sets"))?;
            let auxiliary_data_set =
                (|| -> Result<_, DeserializeError> { Ok(AuxiliaryDataSet::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("auxiliary_data_set"))?;
            let invalid_present = match len {
                cbor_event::Len::Indefinite => raw.cbor_type()? == CBORType::Array,
                cbor_event::Len::Len(4) => false,
                _ => true,
            };
            let invalid_transactions = (|| -> Result<_, DeserializeError> {
                let mut arr = Vec::new();
                if invalid_present {
                    read_len.read_elems(1)?;
                    let len = raw.array()?;
                    while match len {
                        cbor_event::Len::Len(n) => arr.len() < n as usize,
                        cbor_event::Len::Indefinite => true,
                    } {
                        if is_break_tag(raw, "Block.invalid_transactions")? {
                            break;
                        }
                        arr.push(TransactionIndex::deserialize(raw)?);
                    }
                }
                Ok(arr)
            })()
                .map_err(|e| e.annotate("invalid_transactions"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Block {
                header,
                transaction_bodies,
                transaction_witness_sets,
                auxiliary_data_set,
                invalid_transactions,
            })
        })()
            .map_err(|e| e.annotate("Block"))
    }
}
