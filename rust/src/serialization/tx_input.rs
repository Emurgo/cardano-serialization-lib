use crate::*;

impl cbor_event::se::Serialize for TransactionInput {
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

impl Deserialize for TransactionInput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) =>
                /* TODO: check finite len somewhere */
                    {
                        ()
                    }
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break =>
                    /* it's ok */
                        {
                            ()
                        }
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("TransactionInput"))
    }
}

impl DeserializeEmbeddedGroup for TransactionInput {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let transaction_id =
            (|| -> Result<_, DeserializeError> { Ok(TransactionHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("transaction_id"))?;
        let index = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("index"))?;
        Ok(TransactionInput {
            transaction_id,
            index,
        })
    }
}