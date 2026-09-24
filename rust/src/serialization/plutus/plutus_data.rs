use crate::*;
use std::io::SeekFrom;
use crate::serialization::utils::{check_len, is_break_tag, skip_set_tag};

impl cbor_event::se::Serialize for ConstrPlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if let Some(compact_tag) =
            Self::alternative_to_compact_cbor_tag(self.alternative.into())
        {
            // compact form
            serializer.write_tag(compact_tag as u64)?;
            self.data.serialize(serializer)
        } else {
            // general form
            serializer.write_tag(Self::GENERAL_FORM_TAG)?;
            serializer.write_array(cbor_event::Len::Len(2))?;
            self.alternative.serialize(serializer)?;
            self.data.serialize(serializer)
        }
    }
}

impl Deserialize for ConstrPlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (alternative, data) = match raw.tag()? {
                // general form
                Self::GENERAL_FORM_TAG => {
                    let len = raw.array()?;
                    let mut read_len = CBORReadLen::new(len);
                    read_len.read_elems(2)?;
                    let alternative = BigNum::deserialize(raw)?;
                    let data =
                        (|| -> Result<_, DeserializeError> { Ok(PlutusList::deserialize(raw)?) })()
                            .map_err(|e| e.annotate("datas"))?;
                    match len {
                        cbor_event::Len::Len(_) => (),
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => (),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    (alternative, data)
                }
                // concise form
                tag => {
                    if let Some(alternative) = Self::compact_cbor_tag_to_alternative(tag) {
                        (alternative.into(), PlutusList::deserialize(raw)?)
                    } else {
                        return Err(DeserializeFailure::TagMismatch {
                            found: tag,
                            expected: Self::GENERAL_FORM_TAG,
                        }
                            .into());
                    }
                }
            };
            Ok(ConstrPlutusData { alternative, data })
        })()
            .map_err(|e| e.annotate("ConstrPlutusData"))
    }
}

impl cbor_event::se::Serialize for PlutusMap {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.total_len() as u64))?;
        for (key, values) in &self.0 {
            for value in &values.elems {
                key.serialize(serializer)?;
                value.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for PlutusMap {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut plutus_map = PlutusMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut total = 0;
            while match len {
                cbor_event::Len::Len(n) => total < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "PlutusMap")? {
                    break;
                }
                let key = PlutusData::deserialize(raw)?;
                let value = PlutusData::deserialize(raw)?;
                plutus_map.add_value_move(key, value);
                total += 1;
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("PlutusMap"))?;
        Ok(plutus_map)
    }
}

impl cbor_event::se::Serialize for PlutusDataEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serialize_plutus_datum(self, None, serializer)
    }
}

impl Deserialize for PlutusDataEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(PlutusData::deserialize(raw)?.into_datum())
    }
}

impl cbor_event::se::Serialize for PlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serialize_plutus_datum(&self.datum, self.original_bytes.as_deref(), serializer)
    }
}

impl Deserialize for PlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        deserialize_plutus_data_tree(raw)
    }
}

struct PlutusSerFrame<'a> {
    rest: std::vec::IntoIter<&'a PlutusData>,
    close_indefinite: bool,
}

fn serialize_plutus_datum<'se, W: Write>(
    datum: &PlutusDataEnum,
    original_bytes: Option<&[u8]>,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    if let Some(bytes) = original_bytes {
        return serializer.write_raw_bytes(bytes);
    }
    serialize_plutus_data_ref(datum, serializer)
}

fn serialize_plutus_data_ref<'se, W: Write>(
    root_datum: &PlutusDataEnum,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    // We serialize a borrowed datum as if it were PlutusData without original_bytes.
    // Child nodes still go through PlutusData::serialize so original_bytes on children apply.
    enum Root<'a> {
        Datum(&'a PlutusDataEnum),
        Data(&'a PlutusData),
    }
    let mut stack: Vec<PlutusSerFrame<'_>> = Vec::new();
    let mut current = Root::Datum(root_datum);
    loop {
        let (close_indefinite, children) = match current {
            Root::Data(data) => {
                if let Some(bytes) = &data.original_bytes {
                    serializer.write_raw_bytes(bytes)?;
                    (false, Vec::new())
                } else {
                    write_plutus_header(&data.datum, serializer)?
                }
            }
            Root::Datum(datum) => write_plutus_header(datum, serializer)?,
        };
        let mut rest = children.into_iter();
        if let Some(first) = rest.next() {
            stack.push(PlutusSerFrame {
                rest,
                close_indefinite,
            });
            current = Root::Data(first);
            continue;
        }
        if close_indefinite {
            serializer.write_special(cbor_event::Special::Break)?;
        }
        loop {
            match stack.last_mut() {
                None => return Ok(serializer),
                Some(frame) => {
                    if let Some(next) = frame.rest.next() {
                        current = Root::Data(next);
                        break;
                    } else {
                        if frame.close_indefinite {
                            serializer.write_special(cbor_event::Special::Break)?;
                        }
                        stack.pop();
                    }
                }
            }
        }
    }
}

fn write_plutus_header<'a, W: Write>(
    datum: &'a PlutusDataEnum,
    serializer: &mut Serializer<W>,
) -> cbor_event::Result<(bool, Vec<&'a PlutusData>)> {
    match datum {
        PlutusDataEnum::Integer(x) => {
            x.serialize(serializer)?;
            Ok((false, Vec::new()))
        }
        PlutusDataEnum::Bytes(x) => {
            write_bounded_bytes(serializer, x)?;
            Ok((false, Vec::new()))
        }
        PlutusDataEnum::List(list) => write_plutus_list_header(list, serializer),
        PlutusDataEnum::Map(map) => {
            serializer.write_map(cbor_event::Len::Len(map.total_len() as u64))?;
            let mut children = Vec::new();
            for (key, values) in &map.0 {
                for value in &values.elems {
                    children.push(key);
                    children.push(value);
                }
            }
            Ok((false, children))
        }
        PlutusDataEnum::ConstrPlutusData(constr) => {
            if let Some(compact_tag) =
                ConstrPlutusData::alternative_to_compact_cbor_tag(constr.alternative.into())
            {
                serializer.write_tag(compact_tag)?;
                write_plutus_list_header(&constr.data, serializer)
            } else {
                serializer.write_tag(ConstrPlutusData::GENERAL_FORM_TAG)?;
                serializer.write_array(cbor_event::Len::Len(2))?;
                constr.alternative.serialize(serializer)?;
                write_plutus_list_header(&constr.data, serializer)
            }
        }
    }
}

fn write_plutus_list_header<'a, W: Write>(
    list: &'a PlutusList,
    serializer: &mut Serializer<W>,
) -> cbor_event::Result<(bool, Vec<&'a PlutusData>)> {
    // Match `PlutusList::serialize`: set tag 258 is only written by
    // `serialize_as_set`. Nested lists inside PlutusData keep original_bytes
    // when they were deserialized from tagged CBOR.
    let use_definite_encoding = match list.definite_encoding {
        Some(definite) => definite,
        None => list.elems.is_empty(),
    };
    if use_definite_encoding {
        serializer.write_array(cbor_event::Len::Len(list.elems.len() as u64))?;
    } else {
        serializer.write_array(cbor_event::Len::Indefinite)?;
    }
    Ok((!use_definite_encoding, list.elems.iter().collect()))
}

enum PlutusDeKind {
    List {
        definite_encoding: Option<bool>,
        set_type: Option<CborSetType>,
    },
    Constr {
        alternative: BigNum,
        definite_encoding: Option<bool>,
        set_type: Option<CborSetType>,
        general_outer: Option<cbor_event::Len>,
    },
    Map {
        pending_key: Option<PlutusData>,
        map: PlutusMap,
    },
}

struct PlutusDeFrame {
    kind: PlutusDeKind,
    remaining: crate::stack_safe::Remaining,
    acc: Vec<PlutusData>,
    bytes_start: u64,
}

fn deserialize_plutus_data_tree<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<PlutusData, DeserializeError> {
    let mut stack: Vec<PlutusDeFrame> = Vec::new();
    loop {
        let produced = match deserialize_plutus_node(raw)? {
            PlutusDeNode::Complete(data) => data,
            PlutusDeNode::Open(frame) => {
                if plutus_children_finished(raw, &frame)? {
                    finish_plutus_frame(raw, frame)?
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
                        PlutusDeKind::Map {
                            pending_key,
                            map,
                        } => {
                            if pending_key.is_none() {
                                *pending_key = Some(completed);
                                break;
                            } else {
                                let key = pending_key.take().unwrap();
                                map.add_value_move(key, completed);
                                frame.remaining.decrement();
                            }
                        }
                        _ => {
                            frame.acc.push(completed);
                            frame.remaining.decrement();
                        }
                    }
                    if plutus_children_finished(raw, frame)? {
                        let frame = stack.pop().unwrap();
                        completed = finish_plutus_frame(raw, frame)?;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

enum PlutusDeNode {
    Complete(PlutusData),
    Open(PlutusDeFrame),
}

fn plutus_children_finished<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    frame: &PlutusDeFrame,
) -> Result<bool, DeserializeError> {
    if let PlutusDeKind::Map { pending_key, .. } = &frame.kind {
        if pending_key.is_some() {
            return Ok(false);
        }
    }
    match frame.remaining {
        crate::stack_safe::Remaining::Definite(0) => Ok(true),
        crate::stack_safe::Remaining::Definite(_) => Ok(false),
        crate::stack_safe::Remaining::Indefinite => {
            let loc = match &frame.kind {
                PlutusDeKind::Map { .. } => "PlutusMap",
                _ => "PlutusList",
            };
            is_break_tag(raw, loc)
        }
    }
}

fn copy_cbor_span<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    start: u64,
) -> Vec<u8> {
    let after = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
    let bytes_read = (after - start) as usize;
    raw.as_mut_ref().seek(SeekFrom::Start(start)).unwrap();
    let original_bytes = raw.as_mut_ref().fill_buf().unwrap()[..bytes_read].to_vec();
    raw.as_mut_ref().consume(bytes_read);
    original_bytes
}

fn with_original_bytes<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    start: u64,
    datum: PlutusDataEnum,
) -> PlutusData {
    PlutusData {
        datum,
        original_bytes: Some(copy_cbor_span(raw, start)),
    }
}

fn deserialize_plutus_node<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<PlutusDeNode, DeserializeError> {
    let start = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
    match raw.cbor_type()? {
        CBORType::UnsignedInteger | CBORType::NegativeInteger => {
            let integer = BigInt::deserialize(raw)?;
            Ok(PlutusDeNode::Complete(with_original_bytes(
                raw,
                start,
                PlutusDataEnum::Integer(integer),
            )))
        }
        CBORType::Bytes => {
            let bytes = read_bounded_bytes(raw)?;
            Ok(PlutusDeNode::Complete(with_original_bytes(
                raw,
                start,
                PlutusDataEnum::Bytes(bytes),
            )))
        }
        CBORType::Array => Ok(PlutusDeNode::Open(open_plutus_list_frame(
            raw,
            start,
            PlutusDeListRole::List,
        )?)),
        CBORType::Map => {
            let len = raw.map()?;
            Ok(PlutusDeNode::Open(PlutusDeFrame {
                kind: PlutusDeKind::Map {
                    pending_key: None,
                    map: PlutusMap::new(),
                },
                remaining: crate::stack_safe::Remaining::from_cbor_len(len),
                acc: Vec::new(),
                bytes_start: start,
            }))
        }
        CBORType::Tag => {
            let tag = raw.tag()?;
            match tag {
                2 | 3 => {
                    raw.as_mut_ref().seek(SeekFrom::Start(start)).unwrap();
                    let integer = BigInt::deserialize(raw)?;
                    Ok(PlutusDeNode::Complete(with_original_bytes(
                        raw,
                        start,
                        PlutusDataEnum::Integer(integer),
                    )))
                }
                ConstrPlutusData::GENERAL_FORM_TAG => {
                    let outer = raw.array()?;
                    check_len(outer, 2, "ConstrPlutusData")?;
                    let alternative = BigNum::deserialize(raw)?;
                    let mut frame = open_plutus_list_frame(raw, start, PlutusDeListRole::Constr {
                        alternative,
                    })?;
                    if let PlutusDeKind::Constr { general_outer, .. } = &mut frame.kind {
                        *general_outer = Some(outer);
                    }
                    Ok(PlutusDeNode::Open(frame))
                }
                258 => {
                    raw.as_mut_ref().seek(SeekFrom::Start(start)).unwrap();
                    Ok(PlutusDeNode::Open(open_plutus_list_frame(
                        raw,
                        start,
                        PlutusDeListRole::List,
                    )?))
                }
                tag => {
                    if let Some(alternative) = ConstrPlutusData::compact_cbor_tag_to_alternative(tag)
                    {
                        Ok(PlutusDeNode::Open(open_plutus_list_frame(
                            raw,
                            start,
                            PlutusDeListRole::Constr {
                                alternative: alternative.into(),
                            },
                        )?))
                    } else {
                        Err(DeserializeFailure::TagMismatch {
                            found: tag,
                            expected: ConstrPlutusData::GENERAL_FORM_TAG,
                        }
                        .into())
                    }
                }
            }
        }
        ty => Err(DeserializeFailure::ExpectedType(
            "plutus data".to_string(),
            ty,
        )
        .into()),
    }
}

enum PlutusDeListRole {
    List,
    Constr { alternative: BigNum },
}

fn open_plutus_list_frame<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    bytes_start: u64,
    role: PlutusDeListRole,
) -> Result<PlutusDeFrame, DeserializeError> {
    let has_set_tag = skip_set_tag(raw)?;
    let len = raw.array()?;
    let set_type = Some(if has_set_tag {
        CborSetType::Tagged
    } else {
        CborSetType::Untagged
    });
    let definite_encoding = Some(len != cbor_event::Len::Indefinite);
    let kind = match role {
        PlutusDeListRole::List => PlutusDeKind::List {
            definite_encoding,
            set_type,
        },
        PlutusDeListRole::Constr { alternative } => PlutusDeKind::Constr {
            alternative,
            definite_encoding,
            set_type,
            general_outer: None,
        },
    };
    Ok(PlutusDeFrame {
        kind,
        remaining: crate::stack_safe::Remaining::from_cbor_len(len),
        acc: Vec::new(),
        bytes_start,
    })
}

fn finish_plutus_frame<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    frame: PlutusDeFrame,
) -> Result<PlutusData, DeserializeError> {
    let datum = match frame.kind {
        PlutusDeKind::List {
            definite_encoding,
            set_type,
        } => PlutusDataEnum::List(PlutusList {
            elems: frame.acc,
            definite_encoding,
            cbor_set_type: set_type,
        }),
        PlutusDeKind::Constr {
            alternative,
            definite_encoding,
            set_type,
            general_outer,
        } => {
            if let Some(outer) = general_outer {
                crate::serialization::utils::check_len_indefinite(raw, outer)?;
            }
            PlutusDataEnum::ConstrPlutusData(ConstrPlutusData {
                alternative,
                data: PlutusList {
                    elems: frame.acc,
                    definite_encoding,
                    cbor_set_type: set_type,
                },
            })
        }
        PlutusDeKind::Map { map, .. } => PlutusDataEnum::Map(map),
    };
    Ok(with_original_bytes(raw, frame.bytes_start, datum))
}

impl cbor_event::se::Serialize for PlutusList {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let use_definite_encoding = match self.definite_encoding {
            Some(definite) => definite,
            None => self.elems.is_empty(),
        };
        if use_definite_encoding {
            serializer.write_array(cbor_event::Len::Len(self.elems.len() as u64))?;
        } else {
            serializer.write_array(cbor_event::Len::Indefinite)?;
        }
        for element in &self.elems {
            element.serialize(serializer)?;
        }
        if !use_definite_encoding {
            serializer.write_special(cbor_event::Special::Break)?;
        }
        Ok(serializer)
    }
}

impl PlutusList {
    pub(crate) fn serialize_as_set<'se, W: Write>(
        &self,
        need_deduplication: bool,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(258)?;
        let use_definite_encoding = match self.definite_encoding {
            Some(definite) => definite,
            None => self.elems.is_empty(),
        };
        if use_definite_encoding {
            serializer.write_array(cbor_event::Len::Len(self.elems.len() as u64))?;
        } else {
            serializer.write_array(cbor_event::Len::Indefinite)?;
        }
        if need_deduplication {
            for element in self.deduplicated_view() {
                element.serialize(serializer)?;
            }
        } else {
            for element in &self.elems {
                element.serialize(serializer)?;
            }
        }
        if !use_definite_encoding {
            serializer.write_special(cbor_event::Special::Break)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for PlutusList {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let has_set_tag = skip_set_tag(raw)?;
        let mut arr = Vec::new();
        let len = (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "PlutusList")? {
                    break;
                }
                arr.push(PlutusData::deserialize(raw)?);
            }
            Ok(len)
        })()
            .map_err(|e| e.annotate("PlutusList"))?;

        let set_tag = if has_set_tag {
            Some(CborSetType::Tagged)
        } else {
            Some(CborSetType::Untagged)
        };

        Ok(Self {
            elems: arr,
            definite_encoding: Some(len != cbor_event::Len::Indefinite),
            cbor_set_type: set_tag
        })
    }
}