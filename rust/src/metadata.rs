use super::*;
use std::io::{Seek, SeekFrom};

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetadataMap(
    linked_hash_map::LinkedHashMap<TransactionMetadatum, TransactionMetadatum>,
);

to_from_bytes!(MetadataMap);

#[wasm_bindgen]
impl MetadataMap {
    pub fn new() -> Self {
        Self(linked_hash_map::LinkedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &TransactionMetadatum,
        value: &TransactionMetadatum,
    ) -> Option<TransactionMetadatum> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> MetadataList {
        MetadataList(
            self.0
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<TransactionMetadatum>>(),
        )
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetadataList(Vec<TransactionMetadatum>);

to_from_bytes!(MetadataList);

#[wasm_bindgen]
impl MetadataList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionMetadatum {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionMetadatum) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TransactionMetadatumKind {
    MetadataMap,
    MetadataList,
    Int,
    Bytes,
    Text,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum TransactionMetadatumEnum {
    MetadataMap(MetadataMap),
    MetadataList(MetadataList),
    Int(Int),
    Bytes(Vec<u8>),
    Text(String),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransactionMetadatum(TransactionMetadatumEnum);

to_from_bytes!(TransactionMetadatum);

#[wasm_bindgen]
impl TransactionMetadatum {
    pub fn new_map(
        map: &MetadataMap,
    ) -> Self {
        Self(
            TransactionMetadatumEnum::MetadataMap(
                map.clone(),
            ),
        )
    }

    pub fn new_list(
        list: &MetadataList,
    ) -> Self {
        Self(TransactionMetadatumEnum::MetadataList(
            list.clone(),
        ))
    }

    pub fn new_int(int: &Int) -> Self {
        Self(TransactionMetadatumEnum::Int(int.clone()))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self(TransactionMetadatumEnum::Bytes(bytes))
    }

    pub fn new_text(text: String) -> Self {
        Self(TransactionMetadatumEnum::Text(text))
    }

    pub fn kind(&self) -> TransactionMetadatumKind {
        match &self.0 {
            TransactionMetadatumEnum::MetadataMap(_) => {
                TransactionMetadatumKind::MetadataMap
            }
            TransactionMetadatumEnum::MetadataList(_) => {
                TransactionMetadatumKind::MetadataList
            }
            TransactionMetadatumEnum::Int(_) => TransactionMetadatumKind::Int,
            TransactionMetadatumEnum::Bytes(_) => TransactionMetadatumKind::Bytes,
            TransactionMetadatumEnum::Text(_) => TransactionMetadatumKind::Text,
        }
    }

    pub fn as_map(
        &self,
    ) -> Option<MetadataMap> {
        match &self.0 {
            TransactionMetadatumEnum::MetadataMap(x) => {
                Some(x.clone())
            }
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<MetadataList> {
        match &self.0 {
            TransactionMetadatumEnum::MetadataList(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        match &self.0 {
            TransactionMetadatumEnum::Int(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.0 {
            TransactionMetadatumEnum::Bytes(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<String> {
        match &self.0 {
            TransactionMetadatumEnum::Text(x) => Some(x.clone()),
            _ => None,
        }
    }
}

type TransactionMetadatumLabel = BigNum;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionMetadatumLabels(Vec<TransactionMetadatumLabel>);

to_from_bytes!(TransactionMetadatumLabels);

#[wasm_bindgen]
impl TransactionMetadatumLabels {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionMetadatumLabel {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionMetadatumLabel) {
        self.0.push(elem.clone());
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionMetadata(
    linked_hash_map::LinkedHashMap<TransactionMetadatumLabel, TransactionMetadatum>,
);

to_from_bytes!(TransactionMetadata);

#[wasm_bindgen]
impl TransactionMetadata {
    pub fn new() -> Self {
        Self(linked_hash_map::LinkedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &TransactionMetadatumLabel,
        value: &TransactionMetadatum,
    ) -> Option<TransactionMetadatum> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &TransactionMetadatumLabel) -> Option<TransactionMetadatum> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> TransactionMetadatumLabels {
        TransactionMetadatumLabels(
            self.0
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<TransactionMetadatumLabel>>(),
        )
    }
}

// serialization
impl cbor_event::se::Serialize for MetadataMap {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
        let mut table = linked_hash_map::LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = TransactionMetadatum::deserialize(raw)?;
                let value = TransactionMetadatum::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("MetadataMap"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for MetadataList {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionMetadatum::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("MetadataList"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionMetadatumEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            TransactionMetadatumEnum::MetadataMap(x) => {
                x.serialize(serializer)
            },
            TransactionMetadatumEnum::MetadataList(x) => {
                x.serialize(serializer)
            },
            TransactionMetadatumEnum::Int(x) => {
                x.serialize(serializer)
            },
            TransactionMetadatumEnum::Bytes(x) => {
                serializer.write_bytes(&x)
            },
            TransactionMetadatumEnum::Text(x) => {
                serializer.write_text(&x)
            },
        }
    }
}

impl Deserialize for TransactionMetadatumEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MetadataMap::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::MetadataMap(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MetadataList::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::MetadataList(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(Int::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::Int(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(raw.bytes()?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::Bytes(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(String::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::Text(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        Err(DeserializeError::new("TransactionMetadatumEnum", DeserializeFailure::NoVariantMatched.into()))
    }
}

impl cbor_event::se::Serialize for TransactionMetadatum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for TransactionMetadatum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(TransactionMetadatumEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for TransactionMetadatumLabels {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionMetadatumLabel::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionMetadatumLabels"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionMetadata {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = linked_hash_map::LinkedHashMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = TransactionMetadatumLabel::deserialize(raw)?;
                let value = TransactionMetadatum::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionMetadata"))?;
        Ok(Self(table))
    }
}