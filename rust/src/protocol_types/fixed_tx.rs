use crate::error::JsError;
use crate::*;
use std::io::{Seek, SeekFrom};

#[wasm_bindgen]
pub struct FixedTransaction {
    body: TransactionBody,
    body_bytes: Vec<u8>,

    witness_set: TransactionWitnessSet,
    witness_bytes: Vec<u8>,

    is_valid: bool,

    auxiliary_data: Option<AuxiliaryData>,
    auxiliary_bytes: Option<Vec<u8>>,
}

to_from_bytes!(FixedTransaction);


#[wasm_bindgen]
impl FixedTransaction {
    pub fn new(
        raw_body: &[u8],
        raw_witness_set: &[u8],
        is_valid: bool,
    ) -> Result<FixedTransaction, JsError> {
        let body = TransactionBody::from_bytes(raw_body.to_vec())?;
        let witness_set = TransactionWitnessSet::from_bytes(raw_witness_set.to_vec())?;

        Ok(FixedTransaction {
            body,
            body_bytes: raw_body.to_vec(),
            witness_set,
            witness_bytes: raw_witness_set.to_vec(),
            is_valid,
            auxiliary_data: None,
            auxiliary_bytes: None,
        })
    }

    pub fn new_with_auxiliary(
        raw_body: &[u8],
        raw_witness_set: &[u8],
        raw_auxiliary_data: &[u8],
        is_valid: bool,
    ) -> Result<FixedTransaction, JsError> {
        let body = TransactionBody::from_bytes(raw_body.to_vec())?;
        let witness_set = TransactionWitnessSet::from_bytes(raw_witness_set.to_vec())?;
        let auxiliary_data = Some(AuxiliaryData::from_bytes(raw_auxiliary_data.to_vec())?);

        Ok(FixedTransaction {
            body,
            body_bytes: raw_body.to_vec(),
            witness_set,
            witness_bytes: raw_witness_set.to_vec(),
            is_valid,
            auxiliary_data,
            auxiliary_bytes: Some(raw_auxiliary_data.to_vec())
        })
    }

    pub fn body(&self) -> TransactionBody {
        self.body.clone()
    }

    pub fn raw_body(&self) -> Vec<u8> {
        self.body_bytes.clone()
    }

    pub fn set_body(&mut self, raw_body: &[u8]) -> Result<(), JsError> {
        let body = TransactionBody::from_bytes(raw_body.to_vec())?;
        self.body = body;
        self.body_bytes = raw_body.to_vec();
        Ok(())
    }

    pub fn set_witness_set(&mut self, raw_witness_set: &[u8]) -> Result<(), JsError> {
        let witness_set = TransactionWitnessSet::from_bytes(raw_witness_set.to_vec())?;
        self.witness_set = witness_set;
        self.witness_bytes = raw_witness_set.to_vec();
        Ok(())
    }

    pub fn witness_set(&self) -> TransactionWitnessSet {
        self.witness_set.clone()
    }

    pub fn raw_witness_set(&self) -> Vec<u8> {
        self.witness_bytes.clone()
    }

    pub fn set_is_valid(&mut self, valid: bool) {
        self.is_valid = valid
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid.clone()
    }

    pub fn set_auxiliary_data(&mut self, raw_auxiliary_data: &[u8]) -> Result<(), JsError> {
        let auxiliary_data = AuxiliaryData::from_bytes(raw_auxiliary_data.to_vec())?;
        self.auxiliary_data = Some(auxiliary_data);
        self.auxiliary_bytes = Some(raw_auxiliary_data.to_vec());
        Ok(())
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.auxiliary_data.clone()
    }

    pub fn raw_auxiliary_data(&self) -> Option<Vec<u8>> {
        self.auxiliary_bytes.clone()
    }
}

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
                            deserilized_with_orig_bytes(raw, |raw| AuxiliaryData::deserialize(raw))?;
                        (Some(auxiliary_data_deser), Some(auxiliary_bytes_deser))

                    },
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
            auxiliary_bytes
        })
    }
}

fn deserilized_with_orig_bytes<R: BufRead + Seek, T>(
    raw: &mut Deserializer<R>,
    deserilizator: fn(&mut Deserializer<R>) -> Result<T, DeserializeError>,
) -> Result<(T, Vec<u8>), DeserializeError> {
    let before = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
    let value = deserilizator(raw)?;
    let after = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
    let bytes_read = (after - before) as usize;
    raw.as_mut_ref().seek(SeekFrom::Start(before)).unwrap();
    let original_bytes = raw.as_mut_ref().fill_buf().unwrap()[..bytes_read].to_vec();
    raw.as_mut_ref().seek(SeekFrom::Start(after)).unwrap();
    Ok((value, original_bytes))
}
