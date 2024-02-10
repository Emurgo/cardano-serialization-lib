use crate::*;
use std::io::{Seek, SeekFrom};
use crate::serialization::utils::merge_option_plutus_list;
use hashlink::LinkedHashMap;

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

impl cbor_event::se::Serialize for UnitInterval {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
                return Err(DeserializeError::new(
                    "UnitInterval",
                    DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 30,
                    },
                ));
            }
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
        .map_err(|e| e.annotate("UnitInterval"))
    }
}

impl DeserializeEmbeddedGroup for UnitInterval {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let numerator = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
            .map_err(|e| e.annotate("numerator"))?;
        let denominator = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
            .map_err(|e| e.annotate("denominator"))?;
        Ok(UnitInterval {
            numerator,
            denominator,
        })
    }
}

impl cbor_event::se::Serialize for Transaction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.body.serialize(serializer)?;
        self.witness_set.serialize(serializer)?;
        serializer.write_special(CBORSpecial::Bool(self.is_valid))?;
        match &self.auxiliary_data {
            Some(x) => x.serialize(serializer),
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

impl DeserializeEmbeddedGroup for Transaction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let body = (|| -> Result<_, DeserializeError> { Ok(TransactionBody::deserialize(raw)?) })()
            .map_err(|e| e.annotate("body"))?;
        let witness_set =
            (|| -> Result<_, DeserializeError> { Ok(TransactionWitnessSet::deserialize(raw)?) })()
                .map_err(|e| e.annotate("witness_set"))?;
        let mut checked_auxiliary_data = false;
        let mut auxiliary_data = None;
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
                    // if no special symbol was detected, it must have auxiliary data
                    auxiliary_data = (|| -> Result<_, DeserializeError> {
                        Ok(Some(AuxiliaryData::deserialize(raw)?))
                    })()
                    .map_err(|e| e.annotate("auxiliary_data"))?;
                    checked_auxiliary_data = true;
                    return Ok(true);
                }
            }
        })()
        .map_err(|e| e.annotate("is_valid"))?;
        if !checked_auxiliary_data {
            // this branch is reached, if the 3rd argument was a bool. then it simply follows the rules for checking auxiliary data
            auxiliary_data = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != CBORType::Special {
                    true => Some(AuxiliaryData::deserialize(raw)?),
                    false => {
                        if raw.special()? != CBORSpecial::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("auxiliary_data"))?;
        }
        Ok(Transaction {
            body,
            witness_set,
            is_valid,
            auxiliary_data,
        })
    }
}

impl cbor_event::se::Serialize for TransactionOutputs {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionOutput::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("TransactionOutputs"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionOutput {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.has_plutus_data() || self.has_script_ref() {
            //post alonzo output
            let map_len = 2 + opt64(&self.plutus_data) + opt64(&self.script_ref);
            serializer.write_map(cbor_event::Len::Len(map_len))?;
            serializer.write_unsigned_integer(0)?;
            self.address.serialize(serializer)?;
            serializer.write_unsigned_integer(1)?;
            self.amount.serialize(serializer)?;
            if let Some(field) = &self.plutus_data {
                serializer.write_unsigned_integer(2)?;
                field.serialize(serializer)?;
            }
            if let Some(field) = &self.script_ref {
                serializer.write_unsigned_integer(3)?;
                field.serialize(serializer)?;
            }
        } else {
            //lagacy output
            let data_hash = &self.data_hash();
            serializer.write_array(cbor_event::Len::Len(2 + opt64(&data_hash)))?;
            self.address.serialize(serializer)?;
            self.amount.serialize(serializer)?;
            if let Some(pure_data_hash) = data_hash {
                pure_data_hash.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

// this is used when deserializing it on its own, but the more likely case
// is when it's done via TransactionOutputs
impl Deserialize for TransactionOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                CBORType::Array => {
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
                }
                CBORType::Map => deserialize_as_postalonzo_output(raw),
                cbor_type => Err(DeserializeFailure::UnexpectedKeyType(cbor_type).into()),
            }
        })()
        .map_err(|e| e.annotate("TransactionOutput"))
    }
}

// this is used by both TransactionOutput (on its own)'s deserialize
// but also for TransactionOutputs
// This implementation was hand-coded since cddl-codegen doesn't support deserialization
// with array-encoded types with optional fields, due to the complexity.
// This is made worse as this is a plain group...
impl DeserializeEmbeddedGroup for TransactionOutput {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let address = (|| -> Result<_, DeserializeError> { Ok(Address::deserialize(raw)?) })()
            .map_err(|e| e.annotate("address"))?;
        let amount = (|| -> Result<_, DeserializeError> { Ok(Value::deserialize(raw)?) })()
            .map_err(|e| e.annotate("amount"))?;
        // there are only two cases so far where this is used:
        // 1) on its own inside of TransactionOutput's Deserialize trait (only used if someone calls to_bytes() on it)
        // 2) from TransactionOutput's deserialization
        // in 1) we would encounter an array-end (or track it for definite deserialization - which we don't do right now)
        // and in 2) we would encounter the same OR we would encounter the next TransactionOutput in the array
        // Unfortunately, both address and data hash are bytes type, so we can't just check the type, but instead
        // must check the length, and backtrack if that wasn't the case.
        let data_hash = match raw.cbor_type() {
            Ok(cbor_event::Type::Bytes) => {
                let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
                let bytes = raw.bytes().unwrap();
                if bytes.len() == DataHash::BYTE_COUNT {
                    Some(DataOption::DataHash(DataHash(bytes[..DataHash::BYTE_COUNT].try_into().unwrap())))
                } else {
                    // This is an address of the next output in sequence, which luckily is > 32 bytes so there's no confusion
                    // Go to previous place in array then carry on
                    raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap();
                    None
                }
            },
            // not possibly a data hash
            Ok(_) |
            // end of input
            Err(_) => None,
        };
        Ok(TransactionOutput {
            address,
            amount,
            plutus_data: data_hash,
            script_ref: None,
            serialization_format: Some(CborContainerType::Array),
        })
    }
}

fn deserialize_as_postalonzo_output<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<TransactionOutput, DeserializeError> {
    (|| -> Result<_, DeserializeError> {
        let len = raw.map()?;
        let mut read_len = CBORReadLen::new(len);
        let mut address = None;
        let mut amount = None;
        let mut data = None;
        let mut script_ref = None;
        let mut read = 0;
        while match len {
            cbor_event::Len::Len(n) => read < n as usize,
            cbor_event::Len::Indefinite => true,
        } {
            match raw.cbor_type()? {
                CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                    0 => {
                        if address.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                        }
                        address = Some(
                            (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(Address::deserialize(raw)?)
                            })()
                            .map_err(|e| e.annotate("address"))?,
                        );
                    }
                    1 => {
                        if amount.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                        }
                        amount = Some(
                            (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(Value::deserialize(raw)?)
                            })()
                            .map_err(|e| e.annotate("amount"))?,
                        );
                    }
                    2 => {
                        if data.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                        }
                        data = Some(
                            (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(DataOption::deserialize(raw)?)
                            })()
                            .map_err(|e| e.annotate("data"))?,
                        );
                    }
                    3 => {
                        if script_ref.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                        }
                        script_ref = Some(
                            (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(ScriptRef::deserialize(raw)?)
                            })()
                            .map_err(|e| e.annotate("script_ref"))?,
                        );
                    }
                    unknown_key => {
                        return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into())
                    }
                },
                other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
            }
            read += 1;
        }
        let address = match address {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(0)).into()),
        };
        let amount = match amount {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
        };

        read_len.finish()?;
        Ok(TransactionOutput {
            address,
            amount,
            plutus_data: data,
            script_ref,
            serialization_format: Some(CborContainerType::Map),
        })
    })()
    .map_err(|e| e.annotate("TransactionOutput"))
}

impl Deserialize for DataOption {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "[id, datum_or_hash]",
                    ))
                    .into());
                }
            }
            let datum = match raw.unsigned_integer()? {
                0 => DataOption::DataHash(DataHash::deserialize(raw)?),
                1 => {
                    match raw.tag()? {
                        //bytes string tag
                        24 => {
                            let data = (|| -> Result<_, DeserializeError> {
                                Ok(from_bytes(&raw.bytes()?)?)
                            })()
                            .map_err(|e| e.annotate("PlutusData"))?;
                            DataOption::Data(data)
                        }
                        tag => {
                            return Err(DeserializeFailure::TagMismatch {
                                found: tag,
                                expected: 24,
                            }
                            .into());
                        }
                    }
                }
                n => {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(n),
                        expected: Key::Uint(0),
                    }
                    .into())
                }
            };
            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }
            Ok(datum)
        })()
        .map_err(|e| e.annotate("DataOption"))
    }
}

impl cbor_event::se::Serialize for DataOption {
    fn serialize<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match &self {
            DataOption::DataHash(data_hash) => {
                serializer.write_unsigned_integer(0)?;
                data_hash.serialize(serializer)?;
            }
            DataOption::Data(data) => {
                serializer.write_unsigned_integer(1)?;
                let bytes = data.to_bytes();
                serializer.write_tag(24)?.write_bytes(&bytes)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for ScriptRefEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "[id, native_or_putus_script]",
                    ))
                    .into());
                }
            }
            let script_ref = match raw.unsigned_integer()? {
                0 => ScriptRefEnum::NativeScript(NativeScript::deserialize(raw)?),
                1 => ScriptRefEnum::PlutusScript(PlutusScript::deserialize(raw)?),
                2 => ScriptRefEnum::PlutusScript(
                    PlutusScript::deserialize(raw)?.clone_as_version(&Language::new_plutus_v2()),
                ),
                3 => ScriptRefEnum::PlutusScript(
                    PlutusScript::deserialize(raw)?.clone_as_version(&Language::new_plutus_v3()),
                ),
                n => {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(n),
                        expected: Key::Uint(0),
                    }
                    .into())
                }
            };
            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }
            Ok(script_ref)
        })()
        .map_err(|e| e.annotate("ScriptRefEnum"))
    }
}

impl cbor_event::se::Serialize for ScriptRefEnum {
    fn serialize<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match &self {
            ScriptRefEnum::NativeScript(native_script) => {
                serializer.write_unsigned_integer(0)?;
                native_script.serialize(serializer)?;
            }
            ScriptRefEnum::PlutusScript(plutus_script) => {
                serializer.write_unsigned_integer(plutus_script.script_namespace() as u64)?;
                plutus_script.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for ScriptRef {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.tag()? {
                //bytes string tag
                24 => Ok(ScriptRef(from_bytes(&raw.bytes()?)?)),
                tag => {
                    return Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                    .into());
                }
            }
        })()
        .map_err(|e| e.annotate("ScriptRef"))
    }
}

impl cbor_event::se::Serialize for ScriptRef {
    fn serialize<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        let bytes = to_bytes(&self.0);
        serializer.write_tag(24)?.write_bytes(&bytes)?;
        Ok(serializer)
    }
}

impl cbor_event::se::Serialize for Ipv4 {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for Ipv4 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::new_impl(raw.bytes()?)
    }
}

impl cbor_event::se::Serialize for Ipv6 {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for Ipv6 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::new_impl(raw.bytes()?)
    }
}

impl cbor_event::se::Serialize for DNSRecordAorAAAA {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text(&self.0)
    }
}

impl Deserialize for DNSRecordAorAAAA {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::new_impl(raw.text()?)
    }
}

impl cbor_event::se::Serialize for DNSRecordSRV {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text(&self.0)
    }
}

impl Deserialize for DNSRecordSRV {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::new_impl(raw.text()?)
    }
}

impl cbor_event::se::Serialize for URL {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text(&self.0)
    }
}

impl Deserialize for URL {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::new_impl(raw.text()?)
    }
}

impl cbor_event::se::Serialize for SingleHostAddr {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for SingleHostAddr {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0u64)?;
        match &self.port {
            Some(x) => x.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        match &self.ipv4 {
            Some(x) => x.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        match &self.ipv6 {
            Some(x) => x.serialize(serializer),
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
        .map_err(|e| e.annotate("SingleHostAddr"))
    }
}

impl DeserializeEmbeddedGroup for SingleHostAddr {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(0),
                }
                .into());
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("index_0"))?;
        let port = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => Some(Port::deserialize(raw)?),
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })()
        .map_err(|e| e.annotate("port"))?;
        let ipv4 = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => Some(Ipv4::deserialize(raw)?),
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })()
        .map_err(|e| e.annotate("ipv4"))?;
        let ipv6 = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => Some(Ipv6::deserialize(raw)?),
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })()
        .map_err(|e| e.annotate("ipv6"))?;
        Ok(SingleHostAddr { port, ipv4, ipv6 })
    }
}

impl cbor_event::se::Serialize for SingleHostName {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for SingleHostName {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1u64)?;
        match &self.port {
            Some(x) => x.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.dns_name.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for SingleHostName {
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
        .map_err(|e| e.annotate("SingleHostName"))
    }
}

impl DeserializeEmbeddedGroup for SingleHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(1),
                }
                .into());
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("index_0"))?;
        let port = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => Some(Port::deserialize(raw)?),
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })()
        .map_err(|e| e.annotate("port"))?;
        let dns_name =
            (|| -> Result<_, DeserializeError> { Ok(DNSRecordAorAAAA::deserialize(raw)?) })()
                .map_err(|e| e.annotate("dns_name"))?;
        Ok(SingleHostName { port, dns_name })
    }
}

impl cbor_event::se::Serialize for MultiHostName {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for MultiHostName {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2u64)?;
        self.dns_name.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MultiHostName {
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
        .map_err(|e| e.annotate("MultiHostName"))
    }
}

impl DeserializeEmbeddedGroup for MultiHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(2),
                }
                .into());
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("index_0"))?;
        let dns_name =
            (|| -> Result<_, DeserializeError> { Ok(DNSRecordSRV::deserialize(raw)?) })()
                .map_err(|e| e.annotate("dns_name"))?;
        Ok(MultiHostName { dns_name })
    }
}

impl cbor_event::se::Serialize for RelayEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
        .map_err(|e| e.annotate("RelayEnum"))
    }
}

impl DeserializeEmbeddedGroup for RelayEnum {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(SingleHostAddr::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::SingleHostAddr(variant)),
            Err(_) => raw
                .as_mut_ref()
                .seek(SeekFrom::Start(initial_position))
                .unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(SingleHostName::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::SingleHostName(variant)),
            Err(_) => raw
                .as_mut_ref()
                .seek(SeekFrom::Start(initial_position))
                .unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MultiHostName::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::MultiHostName(variant)),
            Err(_) => raw
                .as_mut_ref()
                .seek(SeekFrom::Start(initial_position))
                .unwrap(),
        };
        Err(DeserializeError::new(
            "RelayEnum",
            DeserializeFailure::NoVariantMatched.into(),
        ))
    }
}

impl cbor_event::se::Serialize for Relay {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for Relay {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(RelayEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for PoolMetadata {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.url.serialize(serializer)?;
        self.pool_metadata_hash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for PoolMetadata {
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
        .map_err(|e| e.annotate("PoolMetadata"))
    }
}

impl DeserializeEmbeddedGroup for PoolMetadata {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let url = (|| -> Result<_, DeserializeError> { Ok(URL::deserialize(raw)?) })()
            .map_err(|e| e.annotate("url"))?;
        let pool_metadata_hash =
            (|| -> Result<_, DeserializeError> { Ok(PoolMetadataHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("pool_metadata_hash"))?;
        Ok(PoolMetadata {
            url,
            pool_metadata_hash,
        })
    }
}

impl cbor_event::se::Serialize for RewardAddresses {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(RewardAddress::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("RewardAddresses"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Withdrawals {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
        let mut table = LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = RewardAddress::deserialize(raw)?;
                let value = Coin::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Withdrawals"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for Update {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.proposed_protocol_parameter_updates
            .serialize(serializer)?;
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
        .map_err(|e| e.annotate("Update"))
    }
}

impl DeserializeEmbeddedGroup for Update {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let proposed_protocol_parameter_updates = (|| -> Result<_, DeserializeError> {
            Ok(ProposedProtocolParameterUpdates::deserialize(raw)?)
        })()
        .map_err(|e| e.annotate("proposed_protocol_parameter_updates"))?;
        let epoch = (|| -> Result<_, DeserializeError> { Ok(Epoch::deserialize(raw)?) })()
            .map_err(|e| e.annotate("epoch"))?;
        Ok(Update {
            proposed_protocol_parameter_updates,
            epoch,
        })
    }
}

impl cbor_event::se::Serialize for GenesisHashes {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(GenesisHash::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Genesishashes"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for ScriptHashes {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for ScriptHashes {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(ScriptHash::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("ScriptHashes"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for ProposedProtocolParameterUpdates {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
        let mut table = LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = GenesisHash::deserialize(raw)?;
                let value = ProtocolParamUpdate::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("ProposedProtocolParameterUpdates"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for ProtocolVersion {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ProtocolVersion {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
        .map_err(|e| e.annotate("ProtocolVersion"))
    }
}

impl DeserializeEmbeddedGroup for ProtocolVersion {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let major = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("major"))?;
        let minor = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("minor"))?;
        Ok(ProtocolVersion { major, minor })
    }
}

impl cbor_event::se::Serialize for TransactionBodies {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionBody::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("TransactionBodies"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for AuxiliaryDataSet {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for AuxiliaryDataSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = TransactionIndex::deserialize(raw)?;
                let value = AuxiliaryData::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("AuxiliaryDataSet"))?;
        Ok(Self(table))
    }
}

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
                        if raw.cbor_type()? == CBORType::Special {
                            assert_eq!(raw.special()?, CBORSpecial::Break);
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

impl cbor_event::se::Serialize for Header {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
        .map_err(|e| e.annotate("Header"))
    }
}

impl DeserializeEmbeddedGroup for Header {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let header_body =
            (|| -> Result<_, DeserializeError> { Ok(HeaderBody::deserialize(raw)?) })()
                .map_err(|e| e.annotate("header_body"))?;
        let body_signature =
            (|| -> Result<_, DeserializeError> { Ok(KESSignature::deserialize(raw)?) })()
                .map_err(|e| e.annotate("body_signature"))?;
        Ok(Header {
            header_body,
            body_signature,
        })
    }
}

impl cbor_event::se::Serialize for OperationalCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for OperationalCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
        .map_err(|e| e.annotate("OperationalCert"))
    }
}

impl DeserializeEmbeddedGroup for OperationalCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let hot_vkey = (|| -> Result<_, DeserializeError> { Ok(KESVKey::deserialize(raw)?) })()
            .map_err(|e| e.annotate("hot_vkey"))?;
        let sequence_number = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("sequence_number"))?;
        let kes_period = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("kes_period"))?;
        let sigma =
            (|| -> Result<_, DeserializeError> { Ok(Ed25519Signature::deserialize(raw)?) })()
                .map_err(|e| e.annotate("sigma"))?;
        Ok(OperationalCert {
            hot_vkey,
            sequence_number,
            kes_period,
            sigma,
        })
    }
}

impl cbor_event::se::Serialize for HeaderBody {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(15))?;
        self.block_number.serialize(serializer)?;
        self.slot.serialize(serializer)?;
        match &self.prev_hash {
            Some(x) => x.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.issuer_vkey.serialize(serializer)?;
        self.vrf_vkey.serialize(serializer)?;
        match &self.leader_cert {
            HeaderLeaderCertEnum::NonceAndLeader(nonce_vrf, leader_vrf) => {
                nonce_vrf.serialize(serializer)?;
                leader_vrf.serialize(serializer)?;
            }
            HeaderLeaderCertEnum::VrfResult(vrf_cert) => {
                vrf_cert.serialize(serializer)?;
            }
        }
        self.block_body_size.serialize(serializer)?;
        self.block_body_hash.serialize(serializer)?;
        self.operational_cert
            .serialize_as_embedded_group(serializer)?;
        self.protocol_version
            .serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for HeaderBody {
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
        .map_err(|e| e.annotate("HeaderBody"))
    }
}

impl DeserializeEmbeddedGroup for HeaderBody {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let block_number = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("block_number"))?;
        let slot = (|| -> Result<_, DeserializeError> { Ok(SlotBigNum::deserialize(raw)?) })()
            .map_err(|e| e.annotate("slot"))?;
        let prev_hash = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => Some(BlockHash::deserialize(raw)?),
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })()
        .map_err(|e| e.annotate("prev_hash"))?;
        let issuer_vkey = (|| -> Result<_, DeserializeError> { Ok(Vkey::deserialize(raw)?) })()
            .map_err(|e| e.annotate("issuer_vkey"))?;
        let vrf_vkey = (|| -> Result<_, DeserializeError> { Ok(VRFVKey::deserialize(raw)?) })()
            .map_err(|e| e.annotate("vrf_vkey"))?;
        let leader_cert = {
            // NONCE VFR CERT, first of two certs
            // or a single VRF RESULT CERT
            // depending on the protocol version
            let first_vrf_cert =
                (|| -> Result<_, DeserializeError> { Ok(VRFCert::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("nonce_vrf"))?;
            let cbor_type: cbor_event::Type = raw.cbor_type()?;
            match cbor_type {
                cbor_event::Type::Array => {
                    // Legacy format, reading the second VRF cert
                    let leader_vrf =
                        (|| -> Result<_, DeserializeError> { Ok(VRFCert::deserialize(raw)?) })()
                            .map_err(|e| e.annotate("leader_vrf"))?;
                    HeaderLeaderCertEnum::NonceAndLeader(first_vrf_cert, leader_vrf)
                }
                cbor_event::Type::UnsignedInteger => {
                    // New format, no second VRF cert is present
                    HeaderLeaderCertEnum::VrfResult(first_vrf_cert)
                }
                t => {
                    return Err(DeserializeError::new(
                        "HeaderBody.leader_cert",
                        DeserializeFailure::UnexpectedKeyType(t),
                    ))
                }
            }
        };
        let block_body_size = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("block_body_size"))?;
        let block_body_hash =
            (|| -> Result<_, DeserializeError> { Ok(BlockHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("block_body_hash"))?;

        let operational_cert = (|| -> Result<_, DeserializeError> {
            if raw.cbor_type()? == CBORType::Array {
                Ok(OperationalCert::deserialize(raw)?)
            } else {
                Ok(OperationalCert::deserialize_as_embedded_group(raw, len)?)
            }
        })()
        .map_err(|e| e.annotate("operational_cert"))?;
        let protocol_version = (|| -> Result<_, DeserializeError> {
            if raw.cbor_type()? == CBORType::Array {
                Ok(ProtocolVersion::deserialize(raw)?)
            } else {
                Ok(ProtocolVersion::deserialize_as_embedded_group(raw, len)?)
            }
        })()
        .map_err(|e| e.annotate("protocol_version"))?;
        Ok(HeaderBody {
            block_number,
            slot,
            prev_hash,
            issuer_vkey,
            vrf_vkey,
            leader_cert,
            block_body_size,
            block_body_hash,
            operational_cert,
            protocol_version,
        })
    }
}

impl cbor_event::se::Serialize for AssetName {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for AssetName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Self::new_impl(raw.bytes()?)
    }
}

impl cbor_event::se::Serialize for AssetNames {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for AssetNames {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(AssetName::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("AssetNames"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Assets {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Assets {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = AssetName::deserialize(raw)?;
                let value = BigNum::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Assets"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for MultiAsset {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MultiAsset {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = PolicyID::deserialize(raw)?;
                let value = Assets::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("MultiAsset"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for MintAssets {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MintAssets {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = AssetName::deserialize(raw)?;
                let value = Int::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("MintAssets"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for Mint {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Mint {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut mints = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => mints.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = PolicyID::deserialize(raw)?;
                let value = MintAssets::deserialize(raw)?;
                mints.push((key.clone(), value));
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Mint"))?;
        Ok(Self(mints))
    }
}

impl cbor_event::se::Serialize for NetworkId {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self.0 {
            NetworkIdKind::Testnet => serializer.write_unsigned_integer(0u64),
            NetworkIdKind::Mainnet => serializer.write_unsigned_integer(1u64),
        }
    }
}

impl Deserialize for NetworkId {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.unsigned_integer()? {
                0 => Ok(NetworkId::testnet()),
                1 => Ok(NetworkId::mainnet()),
                _ => Err(DeserializeError::new(
                    "NetworkId",
                    DeserializeFailure::NoVariantMatched.into(),
                )),
            }
        })()
        .map_err(|e| e.annotate("NetworkId"))
    }
}