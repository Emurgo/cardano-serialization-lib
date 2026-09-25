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
        serialize_metadatum_enum(self, serializer)
    }
}

impl Deserialize for TransactionMetadatumEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(TransactionMetadatum::deserialize(raw)?.into_enum())
    }
}

impl cbor_event::se::Serialize for TransactionMetadatum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serialize_metadatum_enum(&self.0, serializer)
    }
}

impl Deserialize for TransactionMetadatum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        deserialize_metadatum_tree(raw)
    }
}

struct MetadatumSerFrame<'a> {
    rest: std::vec::IntoIter<&'a TransactionMetadatum>,
}

fn serialize_metadatum_enum<'se, W: Write>(
    root: &TransactionMetadatumEnum,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    enum Root<'a> {
        Enum(&'a TransactionMetadatumEnum),
        Node(&'a TransactionMetadatum),
    }
    let mut stack: Vec<MetadatumSerFrame<'_>> = Vec::new();
    let mut current = Root::Enum(root);
    loop {
        let children = match current {
            Root::Enum(e) => write_metadatum_header(e, serializer)?,
            Root::Node(n) => write_metadatum_header(&n.0, serializer)?,
        };
        let mut rest = children.into_iter();
        if let Some(first) = rest.next() {
            stack.push(MetadatumSerFrame { rest });
            current = Root::Node(first);
            continue;
        }
        loop {
            match stack.last_mut() {
                None => return Ok(serializer),
                Some(frame) => {
                    if let Some(next) = frame.rest.next() {
                        current = Root::Node(next);
                        break;
                    } else {
                        stack.pop();
                    }
                }
            }
        }
    }
}

fn write_metadatum_header<'a, W: Write>(
    datum: &'a TransactionMetadatumEnum,
    serializer: &mut Serializer<W>,
) -> cbor_event::Result<Vec<&'a TransactionMetadatum>> {
    match datum {
        TransactionMetadatumEnum::Int(x) => {
            x.serialize(serializer)?;
            Ok(Vec::new())
        }
        TransactionMetadatumEnum::Bytes(x) => {
            serializer.write_bytes(x)?;
            Ok(Vec::new())
        }
        TransactionMetadatumEnum::Text(x) => {
            serializer.write_text(x)?;
            Ok(Vec::new())
        }
        TransactionMetadatumEnum::MetadataList(list) => {
            serializer.write_array(cbor_event::Len::Len(list.0.len() as u64))?;
            Ok(list.0.iter().collect())
        }
        TransactionMetadatumEnum::MetadataMap(map) => {
            serializer.write_map(cbor_event::Len::Len(map.0.len() as u64))?;
            let mut children = Vec::new();
            for (key, value) in &map.0 {
                children.push(key);
                children.push(value);
            }
            Ok(children)
        }
    }
}

enum MetadatumDeKind {
    List,
    Map {
        pending_key: Option<TransactionMetadatum>,
        map: MetadataMap,
    },
}

struct MetadatumDeFrame {
    kind: MetadatumDeKind,
    remaining: crate::stack_safe::Remaining,
    acc: Vec<TransactionMetadatum>,
}

enum MetadatumDeNode {
    Complete(TransactionMetadatum),
    Open(MetadatumDeFrame),
}

fn deserialize_metadatum_tree<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<TransactionMetadatum, DeserializeError> {
    let mut stack: Vec<MetadatumDeFrame> = Vec::new();
    loop {
        let produced = match deserialize_metadatum_node(raw)? {
            MetadatumDeNode::Complete(data) => data,
            MetadatumDeNode::Open(frame) => {
                if metadatum_children_finished(raw, &frame)? {
                    finish_metadatum_frame(frame)?
                } else {
                    stack.push(frame);
                    continue;
                }
            }
        };
        let mut completed = produced;
        loop {
            match stack.last_mut() {
                None => return Ok(completed),
                Some(frame) => {
                    match &mut frame.kind {
                        MetadatumDeKind::Map { pending_key, map } => {
                            if pending_key.is_none() {
                                *pending_key = Some(completed);
                                break;
                            } else {
                                let key = pending_key.take().unwrap();
                                map.insert_move(key, completed);
                                frame.remaining.decrement();
                            }
                        }
                        MetadatumDeKind::List => {
                            frame.acc.push(completed);
                            frame.remaining.decrement();
                        }
                    }
                    if metadatum_children_finished(raw, frame)? {
                        let frame = stack.pop().unwrap();
                        completed = finish_metadatum_frame(frame)?;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn metadatum_children_finished<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    frame: &MetadatumDeFrame,
) -> Result<bool, DeserializeError> {
    if let MetadatumDeKind::Map { pending_key, .. } = &frame.kind {
        if pending_key.is_some() {
            return Ok(false);
        }
    }
    match frame.remaining {
        crate::stack_safe::Remaining::Definite(0) => Ok(true),
        crate::stack_safe::Remaining::Definite(_) => Ok(false),
        crate::stack_safe::Remaining::Indefinite => {
            let loc = match &frame.kind {
                MetadatumDeKind::Map { .. } => "MetadataMap",
                MetadatumDeKind::List => "MetadataList",
            };
            is_break_tag(raw, loc)
        }
    }
}

fn deserialize_metadatum_node<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<MetadatumDeNode, DeserializeError> {
    match raw.cbor_type()? {
        CBORType::Array => {
            let len = raw.array()?;
            Ok(MetadatumDeNode::Open(MetadatumDeFrame {
                kind: MetadatumDeKind::List,
                remaining: crate::stack_safe::Remaining::from_cbor_len(len),
                acc: Vec::new(),
            }))
        }
        CBORType::Map => {
            let len = raw.map()?;
            Ok(MetadatumDeNode::Open(MetadatumDeFrame {
                kind: MetadatumDeKind::Map {
                    pending_key: None,
                    map: MetadataMap::new(),
                },
                remaining: crate::stack_safe::Remaining::from_cbor_len(len),
                acc: Vec::new(),
            }))
        }
        CBORType::Bytes => {
            let m = TransactionMetadatum::new_bytes(raw.bytes()?)
                .map_err(DeserializeFailure::Metadata)?;
            Ok(MetadatumDeNode::Complete(m))
        }
        CBORType::Text => {
            let m = TransactionMetadatum::new_text(raw.text()?)
                .map_err(DeserializeFailure::Metadata)?;
            Ok(MetadatumDeNode::Complete(m))
        }
        CBORType::UnsignedInteger | CBORType::NegativeInteger => {
            Ok(MetadatumDeNode::Complete(TransactionMetadatum(
                TransactionMetadatumEnum::Int(Int::deserialize(raw)?),
            )))
        }
        _ => Err(DeserializeError::new(
            "TransactionMetadatumEnum",
            DeserializeFailure::NoVariantMatched.into(),
        )),
    }
}

fn finish_metadatum_frame(
    frame: MetadatumDeFrame,
) -> Result<TransactionMetadatum, DeserializeError> {
    match frame.kind {
        MetadatumDeKind::List => Ok(TransactionMetadatum::from_list_moved(
            MetadataList::from_vec(frame.acc),
        )),
        MetadatumDeKind::Map { map, .. } => Ok(TransactionMetadatum::from_map_moved(map)),
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
                    plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v1, &Language::new_plutus_v1());
                    plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v2, &Language::new_plutus_v2());
                    plutus_scripts = merge_option_plutus_list(plutus_scripts, plutus_scripts_v3, &Language::new_plutus_v3());

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