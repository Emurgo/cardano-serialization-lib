use super::*;
use address::*;
use crypto::*;
use error::*;
use crate::utils::*;
use std::io::{Seek, SeekFrom};

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

impl cbor_event::se::Serialize for UnitInterval {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(30u64)?;
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.numerator.serialize(serializer)?;
        self.denominator.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for UnitInterval {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let tag = raw.tag()?;
            if tag != 30 {
                return Err(DeserializeError::new("UnitInterval", DeserializeFailure::TagMismatch{ found: tag, expected: 30 }));
            }
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("UnitInterval"))
    }
}

impl DeserializeEmbeddedGroup for UnitInterval {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let numerator = (|| -> Result<_, DeserializeError> {
            Ok(BigNum::deserialize(raw)?)
        })().map_err(|e| e.annotate("numerator"))?;
        let denominator = (|| -> Result<_, DeserializeError> {
            Ok(BigNum::deserialize(raw)?)
        })().map_err(|e| e.annotate("denominator"))?;
        Ok(UnitInterval {
            numerator,
            denominator,
        })
    }
}

impl cbor_event::se::Serialize for Transaction {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.body.serialize(serializer)?;
        self.witness_set.serialize(serializer)?;
        match &self.metadata {
            Some(x) => {
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for Transaction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Transaction"))
    }
}

impl DeserializeEmbeddedGroup for Transaction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let body = (|| -> Result<_, DeserializeError> {
            Ok(TransactionBody::deserialize(raw)?)
        })().map_err(|e| e.annotate("body"))?;
        let witness_set = (|| -> Result<_, DeserializeError> {
            Ok(TransactionWitnessSet::deserialize(raw)?)
        })().map_err(|e| e.annotate("witness_set"))?;
        let metadata = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(TransactionMetadata::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("metadata"))?;
        Ok(Transaction {
            body,
            witness_set,
            metadata,
        })
    }
}

impl cbor_event::se::Serialize for TransactionInputs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionInputs {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionInput::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionInputs"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionOutputs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionOutputs {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionOutput::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionOutputs"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Certificates {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Certificates {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Certificate::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Certificates"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionBody {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(4 + match &self.certs { Some(x) => 1, None => 0 } + match &self.withdrawals { Some(x) => 1, None => 0 } + match &self.metadata_hash { Some(x) => 1, None => 0 }))?;
        serializer.write_unsigned_integer(0)?;
        self.inputs.serialize(serializer)?;
        serializer.write_unsigned_integer(1)?;
        self.outputs.serialize(serializer)?;
        serializer.write_unsigned_integer(2)?;
        self.fee.serialize(serializer)?;
        serializer.write_unsigned_integer(3)?;
        self.ttl.serialize(serializer)?;
        if let Some(field) = &self.certs {
            serializer.write_unsigned_integer(4)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.withdrawals {
            serializer.write_unsigned_integer(5)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.update {
            serializer.write_unsigned_integer(6)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.metadata_hash {
            serializer.write_unsigned_integer(7)?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            Self::deserialize_as_embedded_group(raw, len)
        })().map_err(|e| e.annotate("TransactionBody"))
    }
}

impl DeserializeEmbeddedGroup for TransactionBody {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let mut inputs = None;
        let mut outputs = None;
        let mut fee = None;
        let mut ttl = None;
        let mut certs = None;
        let mut withdrawals = None;
        let mut update = None;
        let mut metadata_hash = None;
        let mut read = 0;
        while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
            match raw.cbor_type()? {
                CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                    0 =>  {
                        if inputs.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                        }
                        inputs = Some((|| -> Result<_, DeserializeError> {
                            Ok(TransactionInputs::deserialize(raw)?)
                        })().map_err(|e| e.annotate("inputs"))?);
                    },
                    1 =>  {
                        if outputs.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                        }
                        outputs = Some((|| -> Result<_, DeserializeError> {
                            Ok(TransactionOutputs::deserialize(raw)?)
                        })().map_err(|e| e.annotate("outputs"))?);
                    },
                    2 =>  {
                        if fee.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                        }
                        fee = Some((|| -> Result<_, DeserializeError> {
                            Ok(Coin::deserialize(raw)?)
                        })().map_err(|e| e.annotate("fee"))?);
                    },
                    3 =>  {
                        if ttl.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                        }
                        ttl = Some((|| -> Result<_, DeserializeError> {
                            Ok(u32::deserialize(raw)?)
                        })().map_err(|e| e.annotate("ttl"))?);
                    },
                    4 =>  {
                        if certs.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                        }
                        certs = Some((|| -> Result<_, DeserializeError> {
                            Ok(Certificates::deserialize(raw)?)
                        })().map_err(|e| e.annotate("certs"))?);
                    },
                    5 =>  {
                        if withdrawals.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                        }
                        withdrawals = Some((|| -> Result<_, DeserializeError> {
                            Ok(Withdrawals::deserialize(raw)?)
                        })().map_err(|e| e.annotate("withdrawals"))?);
                    },
                    6 =>  {
                        if update.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                        }
                        update = Some((|| -> Result<_, DeserializeError> {
                            Ok(Update::deserialize(raw)?)
                        })().map_err(|e| e.annotate("update"))?);
                    },
                    7 =>  {
                        if metadata_hash.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                        }
                        metadata_hash = Some((|| -> Result<_, DeserializeError> {
                            Ok(MetadataHash::deserialize(raw)?)
                        })().map_err(|e| e.annotate("metadata_hash"))?);
                    },
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                },
                CBORType::Text => match raw.text()?.as_str() {
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                },
                CBORType::Special => match len {
                    cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                    cbor_event::Len::Indefinite => match raw.special()? {
                        CBORSpecial::Break => break,
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                },
                other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
            }
            read += 1;
        }
        let inputs = match inputs {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(0)).into()),
        };
        let outputs = match outputs {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
        };
        let fee = match fee {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(2)).into()),
        };
        let ttl = match ttl {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(3)).into()),
        };
        Ok(Self {
            inputs,
            outputs,
            fee,
            ttl,
            certs,
            withdrawals,
            update,
            metadata_hash,
        })
    }
}

impl cbor_event::se::Serialize for TransactionInput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("TransactionInput"))
    }
}

impl DeserializeEmbeddedGroup for TransactionInput {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let transaction_id = (|| -> Result<_, DeserializeError> {
            Ok(TransactionHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("transaction_id"))?;
        let index = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("index"))?;
        Ok(TransactionInput {
            transaction_id,
            index,
        })
    }
}

impl cbor_event::se::Serialize for TransactionOutput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.address.serialize(serializer)?;
        self.amount.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for TransactionOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("TransactionOutput"))
    }
}

impl DeserializeEmbeddedGroup for TransactionOutput {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let address = (|| -> Result<_, DeserializeError> {
            Ok(Address::deserialize(raw)?)
        })().map_err(|e| e.annotate("address"))?;
        let amount = (|| -> Result<_, DeserializeError> {
            Ok(Coin::deserialize(raw)?)
        })().map_err(|e| e.annotate("amount"))?;
        Ok(TransactionOutput {
            address,
            amount,
        })
    }
}

impl cbor_event::se::Serialize for StakeRegistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for StakeRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0u64)?;
        self.stake_credential.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeRegistration"))
    }
}

impl DeserializeEmbeddedGroup for StakeRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        Ok(StakeRegistration {
            stake_credential,
        })
    }
}

impl cbor_event::se::Serialize for StakeDeregistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for StakeDeregistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1u64)?;
        self.stake_credential.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeDeregistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeDeregistration"))
    }
}

impl DeserializeEmbeddedGroup for StakeDeregistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        Ok(StakeDeregistration {
            stake_credential,
        })
    }
}

impl cbor_event::se::Serialize for StakeDelegation {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for StakeDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2u64)?;
        self.stake_credential.serialize(serializer)?;
        self.pool_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeDelegation"))
    }
}

impl DeserializeEmbeddedGroup for StakeDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        let pool_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("pool_keyhash"))?;
        Ok(StakeDelegation {
            stake_credential,
            pool_keyhash,
        })
    }
}

impl cbor_event::se::Serialize for Ed25519KeyHashes {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Ed25519KeyHashes {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Ed25519KeyHash::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Ed25519KeyHashes"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Relays {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Relays {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Relay::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Relays"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for PoolParams {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(9))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for PoolParams {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.operator.serialize(serializer)?;
        self.vrf_keyhash.serialize(serializer)?;
        self.pledge.serialize(serializer)?;
        self.cost.serialize(serializer)?;
        self.margin.serialize(serializer)?;
        self.reward_account.serialize(serializer)?;
        self.pool_owners.serialize(serializer)?;
        self.relays.serialize(serializer)?;
        match &self.pool_metadata {
            Some(x) => {
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for PoolParams {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolParams"))
    }
}

impl DeserializeEmbeddedGroup for PoolParams {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let operator = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("operator"))?;
        let vrf_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(VRFKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("vrf_keyhash"))?;
        let pledge = (|| -> Result<_, DeserializeError> {
            Ok(Coin::deserialize(raw)?)
        })().map_err(|e| e.annotate("pledge"))?;
        let cost = (|| -> Result<_, DeserializeError> {
            Ok(Coin::deserialize(raw)?)
        })().map_err(|e| e.annotate("cost"))?;
        let margin = (|| -> Result<_, DeserializeError> {
            Ok(UnitInterval::deserialize(raw)?)
        })().map_err(|e| e.annotate("margin"))?;
        let reward_account = (|| -> Result<_, DeserializeError> {
            Ok(RewardAddress::deserialize(raw)?)
        })().map_err(|e| e.annotate("reward_account"))?;
        let pool_owners = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHashes::deserialize(raw)?)
        })().map_err(|e| e.annotate("pool_owners"))?;
        let relays = (|| -> Result<_, DeserializeError> {
            Ok(Relays::deserialize(raw)?)
        })().map_err(|e| e.annotate("relays"))?;
        let pool_metadata = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(PoolMetadata::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("pool_metadata"))?;
        Ok(PoolParams {
            operator,
            vrf_keyhash,
            pledge,
            cost,
            margin,
            reward_account,
            pool_owners,
            relays,
            pool_metadata,
        })
    }
}

impl cbor_event::se::Serialize for PoolRegistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(10))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for PoolRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3u64)?;
        self.pool_params.serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for PoolRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolRegistration"))
    }
}

impl DeserializeEmbeddedGroup for PoolRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 3 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(3) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let pool_params = (|| -> Result<_, DeserializeError> {
            Ok(PoolParams::deserialize_as_embedded_group(raw, len)?)
        })().map_err(|e| e.annotate("pool_params"))?;
        Ok(PoolRegistration {
            pool_params,
        })
    }
}

impl cbor_event::se::Serialize for PoolRetirement {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for PoolRetirement {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(4u64)?;
        self.pool_keyhash.serialize(serializer)?;
        self.epoch.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for PoolRetirement {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolRetirement"))
    }
}

impl DeserializeEmbeddedGroup for PoolRetirement {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 4 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(4) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let pool_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("pool_keyhash"))?;
        let epoch = (|| -> Result<_, DeserializeError> {
            Ok(Epoch::deserialize(raw)?)
        })().map_err(|e| e.annotate("epoch"))?;
        Ok(PoolRetirement {
            pool_keyhash,
            epoch,
        })
    }
}

impl cbor_event::se::Serialize for GenesisKeyDelegation {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for GenesisKeyDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(5u64)?;
        self.genesishash.serialize(serializer)?;
        self.genesis_delegate_hash.serialize(serializer)?;
        self.vrf_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for GenesisKeyDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("GenesisKeyDelegation"))
    }
}

impl DeserializeEmbeddedGroup for GenesisKeyDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 5 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(5) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let genesishash = (|| -> Result<_, DeserializeError> {
            Ok(GenesisHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("genesishash"))?;
        let genesis_delegate_hash = (|| -> Result<_, DeserializeError> {
            Ok(GenesisDelegateHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("genesis_delegate_hash"))?;
        let vrf_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(VRFKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("vrf_keyhash"))?;
        Ok(GenesisKeyDelegation {
            genesishash,
            genesis_delegate_hash,
            vrf_keyhash,
        })
    }
}

impl cbor_event::se::Serialize for MoveInstantaneousRewardsCert {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for MoveInstantaneousRewardsCert {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(6u64)?;
        self.move_instantaneous_reward.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MoveInstantaneousRewardsCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MoveInstantaneousRewardsCert"))
    }
}

impl DeserializeEmbeddedGroup for MoveInstantaneousRewardsCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 6 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(6) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let move_instantaneous_reward = (|| -> Result<_, DeserializeError> {
            Ok(MoveInstantaneousReward::deserialize(raw)?)
        })().map_err(|e| e.annotate("move_instantaneous_reward"))?;
        Ok(MoveInstantaneousRewardsCert {
            move_instantaneous_reward,
        })
    }
}

impl cbor_event::se::Serialize for CertificateEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            CertificateEnum::StakeRegistration(x) => x.serialize(serializer),
            CertificateEnum::StakeDeregistration(x) => x.serialize(serializer),
            CertificateEnum::StakeDelegation(x) => x.serialize(serializer),
            CertificateEnum::PoolRegistration(x) => x.serialize(serializer),
            CertificateEnum::PoolRetirement(x) => x.serialize(serializer),
            CertificateEnum::GenesisKeyDelegation(x) => x.serialize(serializer),
            CertificateEnum::MoveInstantaneousRewardsCert(x) => x.serialize(serializer),
        }
    }
}

impl Deserialize for CertificateEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("CertificateEnum"))
    }
}

impl DeserializeEmbeddedGroup for CertificateEnum {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(StakeRegistration::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::StakeRegistration(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(StakeDeregistration::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::StakeDeregistration(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(StakeDelegation::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::StakeDelegation(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(PoolRegistration::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::PoolRegistration(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(PoolRetirement::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::PoolRetirement(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(GenesisKeyDelegation::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::GenesisKeyDelegation(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MoveInstantaneousRewardsCert::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::MoveInstantaneousRewardsCert(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        Err(DeserializeError::new("CertificateEnum", DeserializeFailure::NoVariantMatched.into()))
    }
}

impl cbor_event::se::Serialize for Certificate {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for Certificate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(CertificateEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for StakeCredentials {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for StakeCredentials {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(StakeCredential::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("StakeCredentials"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for MoveInstantaneousReward {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match self.pot {
            MIRPot::Reserves => serializer.write_unsigned_integer(0u64),
            MIRPot::Treasury => serializer.write_unsigned_integer(1u64),
        }?;
        serializer.write_map(cbor_event::Len::Len(self.rewards.len() as u64))?;
        for (key, value) in &self.rewards {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MoveInstantaneousReward {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = linked_hash_map::LinkedHashMap::new();
        let pot = (|| -> Result<_, DeserializeError> {
            let outer_len = raw.array()?;
            let pot = match raw.unsigned_integer()? {
                0 => MIRPot::Reserves,
                1 => MIRPot::Treasury,
                n => return Err(DeserializeFailure::UnknownKey(Key::Uint(n)).into()),
            };
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = StakeCredential::deserialize(raw)?;
                let value = Coin::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            match outer_len {
                cbor_event::Len::Len(n) => if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(n, outer_len, "MoveInstantaneousReward")).into())
                },
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            };
            Ok(pot)
        })().map_err(|e| e.annotate("MoveInstantaneousReward"))?;
        Ok(Self {
            pot,
            rewards: table
        })
    }
}

impl cbor_event::se::Serialize for Ipv4 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for Ipv4 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for Ipv6 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for Ipv6 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for SingleHostAddr {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for SingleHostAddr {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0u64)?;
        match &self.port {
            Some(x) => {
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        match &self.ipv4 {
            Some(x) => {
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        match &self.ipv6 {
            Some(x) => {
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for SingleHostAddr {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SingleHostAddr"))
    }
}

impl DeserializeEmbeddedGroup for SingleHostAddr {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let port = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(Port::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("port"))?;
        let ipv4 = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(Ipv4::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("ipv4"))?;
        let ipv6 = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(Ipv6::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("ipv6"))?;
        Ok(SingleHostAddr {
            port,
            ipv4,
            ipv6,
        })
    }
}

impl cbor_event::se::Serialize for SingleHostName {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for SingleHostName {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1u64)?;
        match &self.port {
            Some(x) => {
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        serializer.write_text(&self.dns_name)?;
        Ok(serializer)
    }
}

impl Deserialize for SingleHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SingleHostName"))
    }
}

impl DeserializeEmbeddedGroup for SingleHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let port = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(Port::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("port"))?;
        let dns_name = (|| -> Result<_, DeserializeError> {
            Ok(String::deserialize(raw)?)
        })().map_err(|e| e.annotate("dns_name"))?;
        Ok(SingleHostName {
            port,
            dns_name,
        })
    }
}

impl cbor_event::se::Serialize for MultiHostName {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for MultiHostName {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2u64)?;
        serializer.write_text(&self.dns_name)?;
        Ok(serializer)
    }
}

impl Deserialize for MultiHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MultiHostName"))
    }
}

impl DeserializeEmbeddedGroup for MultiHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let dns_name = (|| -> Result<_, DeserializeError> {
            Ok(String::deserialize(raw)?)
        })().map_err(|e| e.annotate("dns_name"))?;
        Ok(MultiHostName {
            dns_name,
        })
    }
}

impl cbor_event::se::Serialize for RelayEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            RelayEnum::SingleHostAddr(x) => x.serialize(serializer),
            RelayEnum::SingleHostName(x) => x.serialize(serializer),
            RelayEnum::MultiHostName(x) => x.serialize(serializer),
        }
    }
}

impl Deserialize for RelayEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("RelayEnum"))
    }
}

impl DeserializeEmbeddedGroup for RelayEnum {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(SingleHostAddr::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::SingleHostAddr(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(SingleHostName::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::SingleHostName(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MultiHostName::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::MultiHostName(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        Err(DeserializeError::new("RelayEnum", DeserializeFailure::NoVariantMatched.into()))
    }
}

impl cbor_event::se::Serialize for Relay {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for Relay {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(RelayEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for PoolMetadata {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_text(&self.url)?;
        self.metadata_hash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for PoolMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolMetadata"))
    }
}

impl DeserializeEmbeddedGroup for PoolMetadata {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let url = (|| -> Result<_, DeserializeError> {
            Ok(String::deserialize(raw)?)
        })().map_err(|e| e.annotate("url"))?;
        let metadata_hash = (|| -> Result<_, DeserializeError> {
            Ok(MetadataHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("metadata_hash"))?;
        Ok(PoolMetadata {
            url,
            metadata_hash,
        })
    }
}


impl cbor_event::se::Serialize for RewardAddresses {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for RewardAddresses {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(RewardAddress::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("RewardAddresses"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Withdrawals {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Withdrawals {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = linked_hash_map::LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = RewardAddress::deserialize(raw)?;
                let value = Coin::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("Withdrawals"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for MultisigScripts {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MultisigScripts {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(MultisigScript::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("MultisigScripts"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionWitnessSet {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(match &self.vkeys { Some(x) => 1, None => 0 } + match &self.scripts { Some(x) => 1, None => 0 } + match &self.bootstraps { Some(x) => 1, None => 0 }))?;
        if let Some(field) = &self.vkeys {
            serializer.write_unsigned_integer(0)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.scripts {
            serializer.write_unsigned_integer(1)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.bootstraps {
            serializer.write_unsigned_integer(2)?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionWitnessSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            Self::deserialize_as_embedded_group(raw, len)
        })().map_err(|e| e.annotate("TransactionWitnessSet"))
    }
}

impl DeserializeEmbeddedGroup for TransactionWitnessSet {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let mut vkeys = None;
        let mut scripts = None;
        let mut bootstraps = None;
        let mut read = 0;
        while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
            match raw.cbor_type()? {
                CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                    0 =>  {
                        if vkeys.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                        }
                        vkeys = Some((|| -> Result<_, DeserializeError> {
                            Ok(Vkeywitnesses::deserialize(raw)?)
                        })().map_err(|e| e.annotate("vkeys"))?);
                    },
                    1 =>  {
                        if scripts.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                        }
                        scripts = Some((|| -> Result<_, DeserializeError> {
                            Ok(MultisigScripts::deserialize(raw)?)
                        })().map_err(|e| e.annotate("scripts"))?);
                    },
                    2 =>  {
                        if bootstraps.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                        }
                        bootstraps = Some((|| -> Result<_, DeserializeError> {
                            Ok(BootstrapWitnesses::deserialize(raw)?)
                        })().map_err(|e| e.annotate("bootstraps"))?);
                    },
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                },
                CBORType::Text => match raw.text()?.as_str() {
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                },
                CBORType::Special => match len {
                    cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                    cbor_event::Len::Indefinite => match raw.special()? {
                        CBORSpecial::Break => break,
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                },
                other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
            }
            read += 1;
        }
        Ok(Self {
            vkeys,
            scripts,
            bootstraps,
        })
    }
}

impl cbor_event::se::Serialize for MsigPubkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for MsigPubkey {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0u64)?;
        self.addr_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MsigPubkey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MsigPubkey"))
    }
}

impl DeserializeEmbeddedGroup for MsigPubkey {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let addr_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("addr_keyhash"))?;
        Ok(MsigPubkey {
            addr_keyhash,
        })
    }
}

impl cbor_event::se::Serialize for MsigAll {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for MsigAll {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1u64)?;
        self.multisig_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MsigAll {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MsigAll"))
    }
}

impl DeserializeEmbeddedGroup for MsigAll {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let multisig_scripts = (|| -> Result<_, DeserializeError> {
            Ok(MultisigScripts::deserialize(raw)?)
        })().map_err(|e| e.annotate("multisig_scripts"))?;
        Ok(MsigAll {
            multisig_scripts,
        })
    }
}

impl cbor_event::se::Serialize for MsigAny {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for MsigAny {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2u64)?;
        self.multisig_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MsigAny {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MsigAny"))
    }
}

impl DeserializeEmbeddedGroup for MsigAny {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let multisig_scripts = (|| -> Result<_, DeserializeError> {
            Ok(MultisigScripts::deserialize(raw)?)
        })().map_err(|e| e.annotate("multisig_scripts"))?;
        Ok(MsigAny {
            multisig_scripts,
        })
    }
}

impl cbor_event::se::Serialize for MsigNOfK {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for MsigNOfK {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3u64)?;
        self.n.serialize(serializer)?;
        self.multisig_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MsigNOfK {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MsigNOfK"))
    }
}

impl DeserializeEmbeddedGroup for MsigNOfK {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 3 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(3) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let n = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("n"))?;
        let multisig_scripts = (|| -> Result<_, DeserializeError> {
            Ok(MultisigScripts::deserialize(raw)?)
        })().map_err(|e| e.annotate("multisig_scripts"))?;
        Ok(MsigNOfK {
            n,
            multisig_scripts,
        })
    }
}

impl cbor_event::se::Serialize for MultisigScriptEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            MultisigScriptEnum::MsigPubkey(x) => x.serialize(serializer),
            MultisigScriptEnum::MsigAll(x) => x.serialize(serializer),
            MultisigScriptEnum::MsigAny(x) => x.serialize(serializer),
            MultisigScriptEnum::MsigNOfK(x) => x.serialize(serializer),
        }
    }
}

impl Deserialize for MultisigScriptEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MultisigScriptEnum"))
    }
}

impl DeserializeEmbeddedGroup for MultisigScriptEnum {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MsigPubkey::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(MultisigScriptEnum::MsigPubkey(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MsigAll::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(MultisigScriptEnum::MsigAll(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MsigAny::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(MultisigScriptEnum::MsigAny(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MsigNOfK::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(MultisigScriptEnum::MsigNOfK(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        Err(DeserializeError::new("MultisigScriptEnum", DeserializeFailure::NoVariantMatched.into()))
    }
}

impl cbor_event::se::Serialize for MultisigScript {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for MultisigScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(MultisigScriptEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for Update {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.proposed_protocol_parameter_updates.serialize(serializer)?;
        self.epoch.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Update {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Update"))
    }
}

impl DeserializeEmbeddedGroup for Update {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let proposed_protocol_parameter_updates = (|| -> Result<_, DeserializeError> {
            Ok(ProposedProtocolParameterUpdates::deserialize(raw)?)
        })().map_err(|e| e.annotate("proposed_protocol_parameter_updates"))?;
        let epoch = (|| -> Result<_, DeserializeError> {
            Ok(Epoch::deserialize(raw)?)
        })().map_err(|e| e.annotate("epoch"))?;
        Ok(Update {
            proposed_protocol_parameter_updates,
            epoch,
        })
    }
}

impl cbor_event::se::Serialize for GenesisHashes {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for GenesisHashes {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(GenesisHash::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Genesishashes"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for ProposedProtocolParameterUpdates {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for ProposedProtocolParameterUpdates {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = linked_hash_map::LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = GenesisHash::deserialize(raw)?;
                let value = ProtocolParamUpdate::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("ProposedProtocolParameterUpdates"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for ProtocolVersion {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ProtocolVersion {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.major.serialize(serializer)?;
        self.minor.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ProtocolVersion {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("ProtocolVersion"))
    }
}

impl DeserializeEmbeddedGroup for ProtocolVersion {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let major = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("major"))?;
        let minor = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("minor"))?;
        Ok(ProtocolVersion {
            major,
            minor,
        })
    }
}

impl cbor_event::se::Serialize for ProtocolVersions {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize_as_embedded_group(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for ProtocolVersions {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(ProtocolVersion::deserialize_as_embedded_group(raw, len)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("ProtocolVersions"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for ProtocolParamUpdate {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(match &self.minfee_a { Some(x) => 1, None => 0 } + match &self.minfee_b { Some(x) => 1, None => 0 } + match &self.max_block_body_size { Some(x) => 1, None => 0 } + match &self.max_tx_size { Some(x) => 1, None => 0 } + match &self.max_block_header_size { Some(x) => 1, None => 0 } + match &self.key_deposit { Some(x) => 1, None => 0 } + match &self.pool_deposit { Some(x) => 1, None => 0 } + match &self.max_epoch { Some(x) => 1, None => 0 } + match &self.n_opt { Some(x) => 1, None => 0 } + match &self.pool_pledge_influence { Some(x) => 1, None => 0 } + match &self.expansion_rate { Some(x) => 1, None => 0 } + match &self.treasury_growth_rate { Some(x) => 1, None => 0 } + match &self.d { Some(x) => 1, None => 0 } + match &self.extra_entropy { Some(x) => 1, None => 0 } + match &self.protocol_version { Some(x) => 1, None => 0 } + match &self.min_utxo_value { Some(x) => 1, None => 0 }))?;
        if let Some(field) = &self.minfee_a {
            serializer.write_unsigned_integer(0)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.minfee_b {
            serializer.write_unsigned_integer(1)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_block_body_size {
            serializer.write_unsigned_integer(2)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_tx_size {
            serializer.write_unsigned_integer(3)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_block_header_size {
            serializer.write_unsigned_integer(4)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.key_deposit {
            serializer.write_unsigned_integer(5)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.pool_deposit {
            serializer.write_unsigned_integer(6)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_epoch {
            serializer.write_unsigned_integer(7)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.n_opt {
            serializer.write_unsigned_integer(8)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.pool_pledge_influence {
            serializer.write_unsigned_integer(9)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.expansion_rate {
            serializer.write_unsigned_integer(10)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.treasury_growth_rate {
            serializer.write_unsigned_integer(11)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.d {
            serializer.write_unsigned_integer(12)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.extra_entropy {
            serializer.write_unsigned_integer(13)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.protocol_version {
            serializer.write_unsigned_integer(14)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.min_utxo_value {
            serializer.write_unsigned_integer(15)?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for ProtocolParamUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            Self::deserialize_as_embedded_group(raw, len)
        })().map_err(|e| e.annotate("ProtocolParamUpdate"))
    }
}

impl DeserializeEmbeddedGroup for ProtocolParamUpdate {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let mut minfee_a = None;
        let mut minfee_b = None;
        let mut max_block_body_size = None;
        let mut max_tx_size = None;
        let mut max_block_header_size = None;
        let mut key_deposit = None;
        let mut pool_deposit = None;
        let mut max_epoch = None;
        let mut n_opt = None;
        let mut pool_pledge_influence = None;
        let mut expansion_rate = None;
        let mut treasury_growth_rate = None;
        let mut d = None;
        let mut extra_entropy = None;
        let mut protocol_version = None;
        let mut min_utxo_value = None;
        let mut read = 0;
        while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
            match raw.cbor_type()? {
                CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                    0 =>  {
                        if minfee_a.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                        }
                        minfee_a = Some((|| -> Result<_, DeserializeError> {
                            Ok(Coin::deserialize(raw)?)
                        })().map_err(|e| e.annotate("minfee_a"))?);
                    },
                    1 =>  {
                        if minfee_b.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                        }
                        minfee_b = Some((|| -> Result<_, DeserializeError> {
                            Ok(Coin::deserialize(raw)?)
                        })().map_err(|e| e.annotate("minfee_b"))?);
                    },
                    2 =>  {
                        if max_block_body_size.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                        }
                        max_block_body_size = Some((|| -> Result<_, DeserializeError> {
                            Ok(u32::deserialize(raw)?)
                        })().map_err(|e| e.annotate("max_block_body_size"))?);
                    },
                    3 =>  {
                        if max_tx_size.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                        }
                        max_tx_size = Some((|| -> Result<_, DeserializeError> {
                            Ok(u32::deserialize(raw)?)
                        })().map_err(|e| e.annotate("max_tx_size"))?);
                    },
                    4 =>  {
                        if max_block_header_size.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                        }
                        max_block_header_size = Some((|| -> Result<_, DeserializeError> {
                            Ok(u32::deserialize(raw)?)
                        })().map_err(|e| e.annotate("max_block_header_size"))?);
                    },
                    5 =>  {
                        if key_deposit.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                        }
                        key_deposit = Some((|| -> Result<_, DeserializeError> {
                            Ok(Coin::deserialize(raw)?)
                        })().map_err(|e| e.annotate("key_deposit"))?);
                    },
                    6 =>  {
                        if pool_deposit.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                        }
                        pool_deposit = Some((|| -> Result<_, DeserializeError> {
                            Ok(Coin::deserialize(raw)?)
                        })().map_err(|e| e.annotate("pool_deposit"))?);
                    },
                    7 =>  {
                        if max_epoch.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                        }
                        max_epoch = Some((|| -> Result<_, DeserializeError> {
                            Ok(Epoch::deserialize(raw)?)
                        })().map_err(|e| e.annotate("max_epoch"))?);
                    },
                    8 =>  {
                        if n_opt.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                        }
                        n_opt = Some((|| -> Result<_, DeserializeError> {
                            Ok(u32::deserialize(raw)?)
                        })().map_err(|e| e.annotate("n_opt"))?);
                    },
                    9 =>  {
                        if pool_pledge_influence.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                        }
                        pool_pledge_influence = Some((|| -> Result<_, DeserializeError> {
                            Ok(Rational::deserialize(raw)?)
                        })().map_err(|e| e.annotate("pool_pledge_influence"))?);
                    },
                    10 =>  {
                        if expansion_rate.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(10)).into());
                        }
                        expansion_rate = Some((|| -> Result<_, DeserializeError> {
                            Ok(UnitInterval::deserialize(raw)?)
                        })().map_err(|e| e.annotate("expansion_rate"))?);
                    },
                    11 =>  {
                        if treasury_growth_rate.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                        }
                        treasury_growth_rate = Some((|| -> Result<_, DeserializeError> {
                            Ok(UnitInterval::deserialize(raw)?)
                        })().map_err(|e| e.annotate("treasury_growth_rate"))?);
                    },
                    12 =>  {
                        if d.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(12)).into());
                        }
                        d = Some((|| -> Result<_, DeserializeError> {
                            Ok(UnitInterval::deserialize(raw)?)
                        })().map_err(|e| e.annotate("d"))?);
                    },
                    13 =>  {
                        if extra_entropy.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(13)).into());
                        }
                        extra_entropy = Some((|| -> Result<_, DeserializeError> {
                            Ok(Nonce::deserialize(raw)?)
                        })().map_err(|e| e.annotate("extra_entropy"))?);
                    },
                    14 =>  {
                        if protocol_version.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                        }
                        protocol_version = Some((|| -> Result<_, DeserializeError> {
                            Ok(ProtocolVersions::deserialize(raw)?)
                        })().map_err(|e| e.annotate("protocol_version"))?);
                    },
                    15 =>  {
                        if min_utxo_value.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(15)).into());
                        }
                        min_utxo_value = Some((|| -> Result<_, DeserializeError> {
                            Ok(Coin::deserialize(raw)?)
                        })().map_err(|e| e.annotate("min_utxo_value"))?);
                    },
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                },
                CBORType::Text => match raw.text()?.as_str() {
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                },
                CBORType::Special => match len {
                    cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                    cbor_event::Len::Indefinite => match raw.special()? {
                        CBORSpecial::Break => break,
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                },
                other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
            }
            read += 1;
        }
        Ok(Self {
            minfee_a,
            minfee_b,
            max_block_body_size,
            max_tx_size,
            max_block_header_size,
            key_deposit,
            pool_deposit,
            max_epoch,
            n_opt,
            pool_pledge_influence,
            expansion_rate,
            treasury_growth_rate,
            d,
            extra_entropy,
            protocol_version,
            min_utxo_value,
        })
    }
}

impl cbor_event::se::Serialize for TransactionBodies {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionBodies {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionBody::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionBodies"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionWitnessSets {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionWitnessSets {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionWitnessSet::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionWitnessSets"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for MapTransactionIndexToTransactionMetadata {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MapTransactionIndexToTransactionMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = linked_hash_map::LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = TransactionIndex::deserialize(raw)?;
                let value = TransactionMetadata::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("MapTransactionIndexToTransactionMetadata"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for Block {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.header.serialize(serializer)?;
        self.transaction_bodies.serialize(serializer)?;
        self.transaction_witness_sets.serialize(serializer)?;
        self.transaction_metadata_set.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Block {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Block"))
    }
}

impl DeserializeEmbeddedGroup for Block {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let header = (|| -> Result<_, DeserializeError> {
            Ok(Header::deserialize(raw)?)
        })().map_err(|e| e.annotate("header"))?;
        let transaction_bodies = (|| -> Result<_, DeserializeError> {
            Ok(TransactionBodies::deserialize(raw)?)
        })().map_err(|e| e.annotate("transaction_bodies"))?;
        let transaction_witness_sets = (|| -> Result<_, DeserializeError> {
            Ok(TransactionWitnessSets::deserialize(raw)?)
        })().map_err(|e| e.annotate("transaction_witness_sets"))?;
        let transaction_metadata_set = (|| -> Result<_, DeserializeError> {
            Ok(MapTransactionIndexToTransactionMetadata::deserialize(raw)?)
        })().map_err(|e| e.annotate("transaction_metadata_set"))?;
        Ok(Block {
            header,
            transaction_bodies,
            transaction_witness_sets,
            transaction_metadata_set,
        })
    }
}

impl cbor_event::se::Serialize for Header {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.header_body.serialize(serializer)?;
        self.body_signature.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Header {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Header"))
    }
}

impl DeserializeEmbeddedGroup for Header {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let header_body = (|| -> Result<_, DeserializeError> {
            Ok(HeaderBody::deserialize(raw)?)
        })().map_err(|e| e.annotate("header_body"))?;
        let body_signature = (|| -> Result<_, DeserializeError> {
            Ok(KESSignature::deserialize(raw)?)
        })().map_err(|e| e.annotate("body_signature"))?;
        Ok(Header {
            header_body,
            body_signature,
        })
    }
}

impl cbor_event::se::Serialize for OperationalCert {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for OperationalCert {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.hot_vkey.serialize(serializer)?;
        self.sequence_number.serialize(serializer)?;
        self.kes_period.serialize(serializer)?;
        self.sigma.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for OperationalCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("OperationalCert"))
    }
}

impl DeserializeEmbeddedGroup for OperationalCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let hot_vkey = (|| -> Result<_, DeserializeError> {
            Ok(KESVKey::deserialize(raw)?)
        })().map_err(|e| e.annotate("hot_vkey"))?;
        let sequence_number = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("sequence_number"))?;
        let kes_period = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("kes_period"))?;
        let sigma = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519Signature::deserialize(raw)?)
        })().map_err(|e| e.annotate("sigma"))?;
        Ok(OperationalCert {
            hot_vkey,
            sequence_number,
            kes_period,
            sigma,
        })
    }
}

impl cbor_event::se::Serialize for HeaderBody {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(15))?;
        self.block_number.serialize(serializer)?;
        self.slot.serialize(serializer)?;
        match &self.prev_hash {
            Some(x) => {
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.issuer_vkey.serialize(serializer)?;
        self.vrf_vkey.serialize(serializer)?;
        self.nonce_vrf.serialize(serializer)?;
        self.leader_vrf.serialize(serializer)?;
        self.block_body_size.serialize(serializer)?;
        self.block_body_hash.serialize(serializer)?;
        self.operational_cert.serialize_as_embedded_group(serializer)?;
        self.protocol_version.serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for HeaderBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("HeaderBody"))
    }
}

impl DeserializeEmbeddedGroup for HeaderBody {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let block_number = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("block_number"))?;
        let slot = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("slot"))?;
        let prev_hash = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(BlockHash::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("prev_hash"))?;
        let issuer_vkey = (|| -> Result<_, DeserializeError> {
            Ok(Vkey::deserialize(raw)?)
        })().map_err(|e| e.annotate("issuer_vkey"))?;
        let vrf_vkey = (|| -> Result<_, DeserializeError> {
            Ok(VRFVKey::deserialize(raw)?)
        })().map_err(|e| e.annotate("vrf_vkey"))?;
        let nonce_vrf = (|| -> Result<_, DeserializeError> {
            Ok(VRFCert::deserialize(raw)?)
        })().map_err(|e| e.annotate("nonce_vrf"))?;
        let leader_vrf = (|| -> Result<_, DeserializeError> {
            Ok(VRFCert::deserialize(raw)?)
        })().map_err(|e| e.annotate("leader_vrf"))?;
        let block_body_size = (|| -> Result<_, DeserializeError> {
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("block_body_size"))?;
        let block_body_hash = (|| -> Result<_, DeserializeError> {
            Ok(BlockHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("block_body_hash"))?;
        let operational_cert = (|| -> Result<_, DeserializeError> {
            Ok(OperationalCert::deserialize_as_embedded_group(raw, len)?)
        })().map_err(|e| e.annotate("operational_cert"))?;
        let protocol_version = (|| -> Result<_, DeserializeError> {
            Ok(ProtocolVersion::deserialize_as_embedded_group(raw, len)?)
        })().map_err(|e| e.annotate("protocol_version"))?;
        Ok(HeaderBody {
            block_number,
            slot,
            prev_hash,
            issuer_vkey,
            vrf_vkey,
            nonce_vrf,
            leader_vrf,
            block_body_size,
            block_body_hash,
            operational_cert,
            protocol_version,
        })
    }
}