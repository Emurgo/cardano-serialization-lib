use crate::*;

impl cbor_event::se::Serialize for GovernanceActionId {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.transaction_id.serialize(serializer)?;
        self.index.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for GovernanceActionId {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "[transaction_id, gov_action_index]",
                    ))
                        .into());
                }
            }

            let transaction_id = TransactionHash::deserialize(raw).map_err(
                |e| e.annotate("transaction_id")
            )?;

            let index = GovernanceActionIndex::deserialize(raw).map_err(
                |e| e.annotate("index")
            )?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            Ok(Self {
                transaction_id,
                index,
            })

        })()
            .map_err(|e| e.annotate("GovernanceActionId"))
    }
}