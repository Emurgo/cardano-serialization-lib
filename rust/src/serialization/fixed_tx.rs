use crate::*;
use crate::serialization::utils::deserilized_with_orig_bytes;

impl cbor_event::se::Serialize for FixedTransaction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_raw_bytes(&self.body_bytes)?;
        serializer.write_raw_bytes(&self.witness_bytes)?;
        serializer.write_special(CBORSpecial::Bool(self.is_valid))?;
        match &self.auxiliary_bytes {
            Some(auxiliary_bytes) => serializer.write_raw_bytes(auxiliary_bytes)?,
            None => serializer.write_special(CBORSpecial::Null)?,
        };
        Ok(serializer)
    }
}

impl Deserialize for FixedTransaction {
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
        .map_err(|e| e.annotate("Transaction"))
    }
}

impl DeserializeEmbeddedGroup for FixedTransaction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let (body, body_bytes) =
            deserilized_with_orig_bytes(raw, |raw| TransactionBody::deserialize(raw))
                .map_err(|e| e.annotate("body"))?;
        let (witness_set, witness_bytes) =
            deserilized_with_orig_bytes(raw, |raw| TransactionWitnessSet::deserialize(raw))
                .map_err(|e| e.annotate("witness_set"))?;
        let mut checked_auxiliary_data = false;
        let mut auxiliary_data = None;
        let mut auxiliary_bytes = None;
        let is_valid = (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? == CBORType::Special {
                true => {
                    // if it's special it can be either a bool or null. if it's null, then it's empty auxiliary data, otherwise not a valid encoding
                    let special = raw.special()?;
                    if let CBORSpecial::Bool(b) = special {
                        return Ok(b);
                    } else if special == CBORSpecial::Null {
                        checked_auxiliary_data = true;
                        return Ok(true);
                    } else {
                        return Err(DeserializeFailure::ExpectedBool.into());
                    }
                }
                false => {
                    let (auxiliary_data_deser, auxiliary_bytes_deser) =
                        deserilized_with_orig_bytes(raw, |raw| AuxiliaryData::deserialize(raw))
                            .map_err(|e| e.annotate("auxiliary_data"))?;
                    auxiliary_data = Some(auxiliary_data_deser);
                    auxiliary_bytes = Some(auxiliary_bytes_deser);
                    // if no special symbol was detected, it must have auxiliary data
                    checked_auxiliary_data = true;
                    return Ok(true);
                }
            }
        })()
        .map_err(|e| e.annotate("is_valid"))?;
        if !checked_auxiliary_data {
            // this branch is reached, if the 3rd argument was a bool. then it simply follows the rules for checking auxiliary data
            (auxiliary_data, auxiliary_bytes) = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != CBORType::Special {
                    true => {
                        let (auxiliary_data_deser, auxiliary_bytes_deser) =
                            deserilized_with_orig_bytes(raw, |raw| {
                                AuxiliaryData::deserialize(raw)
                            })?;
                        (Some(auxiliary_data_deser), Some(auxiliary_bytes_deser))
                    }
                    false => {
                        if raw.special()? != CBORSpecial::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        (None, None)
                    }
                })
            })()
            .map_err(|e| e.annotate("auxiliary_data"))?;
        }
        Ok(FixedTransaction {
            body,
            body_bytes,
            witness_set,
            witness_bytes,
            is_valid,
            auxiliary_data,
            auxiliary_bytes,
        })
    }
}
