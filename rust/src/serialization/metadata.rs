use hashlink::LinkedHashMap;
use crate::*;
use crate::serialization::utils::{is_break_tag, merge_option_plutus_list};

impl cbor_event::se::Serialize for MetadataMap {
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

impl Deserialize for MetadataMap {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = LinkedHashMap::new();
        let mut entries: Vec<(TransactionMetadatum, TransactionMetadatum)> = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => entries.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "MetadataMap")? {
                    break;
                }
                let key = TransactionMetadatum::deserialize(raw)?;
                let value = TransactionMetadatum::deserialize(raw)?;
                entries.push((key.clone(), value));
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("MetadataMap"))?;
        entries.iter().for_each(|(k, v)| {
            if table.insert(k.clone(), v.clone()).is_some() {
                // Turns out this is totally possible on the actual blockchain
                // return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
            }
        });
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for MetadataList {
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

impl Deserialize for MetadataList {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "MetadataList")? {
                    break;
                }
                arr.push(TransactionMetadatum::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("MetadataList"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionMetadatumEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            TransactionMetadatumEnum::MetadataMap(x) => x.serialize(serializer),
            TransactionMetadatumEnum::MetadataList(x) => x.serialize(serializer),
            TransactionMetadatumEnum::Int(x) => x.serialize(serializer),
            TransactionMetadatumEnum::Bytes(x) => serializer.write_bytes(&x),
            TransactionMetadatumEnum::Text(x) => serializer.write_text(&x),
        }
    }
}

impl Deserialize for TransactionMetadatumEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        match raw.cbor_type()? {
            CBORType::Array => {
                MetadataList::deserialize(raw).map(TransactionMetadatumEnum::MetadataList)
            }
            CBORType::Map => {
                MetadataMap::deserialize(raw).map(TransactionMetadatumEnum::MetadataMap)
            }
            CBORType::Bytes => TransactionMetadatum::new_bytes(raw.bytes()?)
                .map(|m| m.0)
                .map_err(|e| DeserializeFailure::Metadata(e).into()),
            CBORType::Text => TransactionMetadatum::new_text(raw.text()?)
                .map(|m| m.0)
                .map_err(|e| DeserializeFailure::Metadata(e).into()),
            CBORType::UnsignedInteger | CBORType::NegativeInteger => {
                Int::deserialize(raw).map(TransactionMetadatumEnum::Int)
            }
            _ => Err(DeserializeError::new(
                "TransactionMetadatumEnum",
                DeserializeFailure::NoVariantMatched.into(),
            )),
        }
    }
}

impl cbor_event::se::Serialize for TransactionMetadatum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for TransactionMetadatum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(TransactionMetadatumEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for TransactionMetadatumLabels {
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

impl Deserialize for TransactionMetadatumLabels {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "TransactionMetadatumLabels")? {
                    break;
                }
                arr.push(TransactionMetadatumLabel::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("TransactionMetadatumLabels"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for GeneralTransactionMetadata {
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

impl Deserialize for GeneralTransactionMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "GeneralTransactionMetadata")? {
                    break;
                }
                let key = TransactionMetadatumLabel::deserialize(raw)?;
                let value = TransactionMetadatum::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                        .into());
                }
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("GeneralTransactionMetadata"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for AuxiliaryData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        // we still serialize using the shelley-mary era format as it is still supported
        // and it takes up less space on-chain so this should be better for scaling.
        // Plus the code was already written for shelley-mary anyway
        if !self.prefer_alonzo_format && self.metadata.is_some() && self.plutus_scripts.is_none() {
            match &self.native_scripts() {
                Some(native_scripts) => {
                    serializer.write_array(cbor_event::Len::Len(2))?;
                    self.metadata.as_ref().unwrap().serialize(serializer)?;
                    native_scripts.serialize(serializer)
                }
                None => self.metadata.as_ref().unwrap().serialize(serializer),
            }
        } else {
            let mut has_plutus_v2 = false;
            let mut has_plutus_v3 = false;
            let plutus_added_length = match &self.plutus_scripts {
                Some(scripts) => {
                    has_plutus_v2 = scripts.has_version(&Language::new_plutus_v2());
                    has_plutus_v3 = scripts.has_version(&Language::new_plutus_v3());
                    1 + (has_plutus_v2 as u64) + (has_plutus_v3 as u64)
                },
                _ => 0,
            };

            // new format with plutus support
            serializer.write_tag(259u64)?;
            serializer.write_map(cbor_event::Len::Len(
                opt64(&self.metadata) + opt64(&self.native_scripts) + plutus_added_length,
            ))?;
            if let Some(metadata) = &self.metadata {
                serializer.write_unsigned_integer(0)?;
                metadata.serialize(serializer)?;
            }
            if let Some(native_scripts) = &self.native_scripts {
                serializer.write_unsigned_integer(1)?;
                native_scripts.serialize(serializer)?;
            }
            if let Some(plutus_scripts) = &self.plutus_scripts {
                serializer.write_unsigned_integer(2)?;
                plutus_scripts.serialize_by_version(&Language::new_plutus_v1(), serializer)?;
                if has_plutus_v2 {
                    serializer.write_unsigned_integer(3)?;
                    plutus_scripts.serialize_by_version(&Language::new_plutus_v2(), serializer)?;
                }
                if has_plutus_v3 {
                    serializer.write_unsigned_integer(4)?;
                    plutus_scripts.serialize_by_version(&Language::new_plutus_v3(), serializer)?;
                }
            }
            Ok(serializer)
        }
    }
}

impl Deserialize for AuxiliaryData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                // alonzo format
                CBORType::Tag => {
                    let tag = raw.tag()?;
                    if tag != 259 {
                        return Err(DeserializeError::new(
                            "AuxiliaryData",
                            DeserializeFailure::TagMismatch {
                                found: tag,
                                expected: 259,
                            },
                        ));
                    }
                    let len = raw.map()?;
                    let mut read_len = CBORReadLen::new(len);
                    let mut metadata = None;
                    let mut native_scripts = None;
                    let mut plutus_scripts_v1 = None;
                    let mut plutus_scripts_v2 = None;
                    let mut plutus_scripts_v3 = None;
                    let mut read = 0;
                    while match len {
                        cbor_event::Len::Len(n) => read < n as usize,
                        cbor_event::Len::Indefinite => true,
                    } {
                        match raw.cbor_type()? {
                            CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                                0 => {
                                    if metadata.is_some() {
                                        return Err(
                                            DeserializeFailure::DuplicateKey(Key::Uint(0)).into()
                                        );
                                    }
                                    metadata = Some(
                                        (|| -> Result<_, DeserializeError> {
                                            read_len.read_elems(1)?;
                                            Ok(GeneralTransactionMetadata::deserialize(raw)?)
                                        })()
                                            .map_err(|e| e.annotate("metadata"))?,
                                    );
                                }
                                1 => {
                                    if native_scripts.is_some() {
                                        return Err(
                                            DeserializeFailure::DuplicateKey(Key::Uint(1)).into()
                                        );
                                    }
                                    native_scripts = Some(
                                        (|| -> Result<_, DeserializeError> {
                                            read_len.read_elems(1)?;
                                            Ok(NativeScripts::deserialize(raw)?)
                                        })()
                                            .map_err(|e| e.annotate("native_scripts"))?,
                                    );
                                }
                                2 => {
                                    if plutus_scripts_v1.is_some() {
                                        return Err(
                                            DeserializeFailure::DuplicateKey(Key::Uint(2)).into()
                                        );
                                    }
                                    plutus_scripts_v1 = Some(
                                        (|| -> Result<_, DeserializeError> {
                                            read_len.read_elems(1)?;
                                            Ok(PlutusScripts::deserialize(raw)?)
                                        })()
                                            .map_err(|e| e.annotate("plutus_scripts_v1"))?,
                                    );
                                }
                                3 => {
                                    if plutus_scripts_v2.is_some() {
                                        return Err(
                                            DeserializeFailure::DuplicateKey(Key::Uint(3)).into()
                                        );
                                    }
                                    plutus_scripts_v2 = Some(
                                        (|| -> Result<_, DeserializeError> {
                                            read_len.read_elems(1)?;
                                            Ok(PlutusScripts::deserialize_with_version(raw, &Language::new_plutus_v2())?)
                                        })()
                                            .map_err(|e| e.annotate("plutus_scripts_v2"))?,
                                    );
                                }
                                4 => {
                                    if plutus_scripts_v3.is_some() {
                                        return Err(
                                            DeserializeFailure::DuplicateKey(Key::Uint(3)).into()
                                        );
                                    }
                                    plutus_scripts_v3 = Some(
                                        (|| -> Result<_, DeserializeError> {
                                            read_len.read_elems(1)?;
                                            Ok(PlutusScripts::deserialize_with_version(raw, &Language::new_plutus_v3())?)
                                        })()
                                            .map_err(|e| e.annotate("plutus_scripts_v3"))?,
                                    );
                                }
                                unknown_key => {
                                    return Err(DeserializeFailure::UnknownKey(Key::Uint(
                                        unknown_key,
                                    ))
                                        .into())
                                }
                            },
                            CBORType::Text => match raw.text()?.as_str() {
                                unknown_key => {
                                    return Err(DeserializeFailure::UnknownKey(Key::Str(
                                        unknown_key.to_owned(),
                                    ))
                                        .into())
                                }
                            },
                            CBORType::Special => match len {
                                cbor_event::Len::Len(_) => {
                                    return Err(DeserializeFailure::BreakInDefiniteLen.into())
                                }
                                cbor_event::Len::Indefinite => match raw.special()? {
                                    CBORSpecial::Break => break,
                                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                                },
                            },
                            other_type => {
                                return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                            }
                        }
                        read += 1;
                    }
                    read_len.finish()?;
                    let mut  plutus_scripts = None;
                    plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v1);
                    plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v2);
                    plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v3);

                    Ok(Self {
                        metadata,
                        native_scripts,
                        plutus_scripts,
                        prefer_alonzo_format: true,
                    })
                }
                // shelley mary format (still valid for alonzo)
                CBORType::Array => {
                    let len = raw.array()?;
                    let mut read_len = CBORReadLen::new(len);
                    read_len.read_elems(2)?;
                    let metadata = (|| -> Result<_, DeserializeError> {
                        Ok(GeneralTransactionMetadata::deserialize(raw)?)
                    })()
                        .map_err(|e| e.annotate("metadata"))?;
                    let native_scripts = (|| -> Result<_, DeserializeError> {
                        Ok(NativeScripts::deserialize(raw)?)
                    })()
                        .map_err(|e| e.annotate("native_scripts"))?;
                    match len {
                        cbor_event::Len::Len(_) => (),
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => (),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    Ok(Self {
                        metadata: Some(metadata),
                        native_scripts: Some(native_scripts),
                        plutus_scripts: None,
                        prefer_alonzo_format: false,
                    })
                }
                // shelley pre-mary format (still valid for alonzo + mary)
                CBORType::Map => Ok(Self {
                    metadata: Some(
                        GeneralTransactionMetadata::deserialize(raw)
                            .map_err(|e| e.annotate("metadata"))?,
                    ),
                    native_scripts: None,
                    plutus_scripts: None,
                    prefer_alonzo_format: false,
                }),
                _ => return Err(DeserializeFailure::NoVariantMatched)?,
            }
        })()
            .map_err(|e| e.annotate("AuxiliaryData"))
    }
}