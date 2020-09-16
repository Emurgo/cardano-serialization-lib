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

    pub fn get(&self, key: &TransactionMetadatum) -> Result<TransactionMetadatum, JsValue> {
        self.0.get(key).map(|v| v.clone()).ok_or_else(|| JsValue::from_str(&format!("key {:?} not found", key)))
    }

    pub fn has(&self, key: &TransactionMetadatum) -> bool {
        self.0.contains_key(key)
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
    ) -> Result<MetadataMap, JsValue> {
        match &self.0 {
            TransactionMetadatumEnum::MetadataMap(x) => {
                Ok(x.clone())
            }
            _ => Err(JsValue::from_str("not a map")),
        }
    }

    pub fn as_list(&self) -> Result<MetadataList, JsValue> {
        match &self.0 {
            TransactionMetadatumEnum::MetadataList(x) => Ok(x.clone()),
            _ => Err(JsValue::from_str("not a list")),
        }
    }

    pub fn as_int(&self) -> Result<Int, JsValue> {
        match &self.0 {
            TransactionMetadatumEnum::Int(x) => Ok(x.clone()),
            _ => Err(JsValue::from_str("not an int")),
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, JsValue> {
        match &self.0 {
            TransactionMetadatumEnum::Bytes(x) => Ok(x.clone()),
            _ => Err(JsValue::from_str("not bytes")),
        }
    }

    pub fn as_text(&self) -> Result<String, JsValue> {
        match &self.0 {
            TransactionMetadatumEnum::Text(x) => Ok(x.clone()),
            _ => Err(JsValue::from_str("not text")),
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

// encodes arbitrary bytes into chunks of 64 bytes (the limit for bytes) as a list to be valid Metadata
#[wasm_bindgen]
pub fn encode_arbitrary_bytes_as_metadatum(bytes: &[u8]) -> TransactionMetadatum {
    let mut list = MetadataList::new();
    for chunk in bytes.chunks(64) {
        list.add(&TransactionMetadatum::new_bytes(chunk.to_vec()));
    }
    TransactionMetadatum::new_list(&list)
}

// decodes from chunks of bytes in a list to a byte vector if that is the metadata format, otherwise returns None
#[wasm_bindgen]
pub fn decode_arbitrary_bytes_from_metadatum(metadata: &TransactionMetadatum) -> Result<Vec<u8>, JsValue> {
    let mut bytes = Vec::new();
    for elem in metadata.as_list()?.0 {
        bytes.append(&mut elem.as_bytes()?);
    }
    Ok(bytes)
}

// encodes a JSON object represented as a string into metadatum if possible.
// Bytes are not supported due to ambiguity of representation in pure JSON
// as representing as a byte array could be confused with a list, and using
// a hex/b58etc string could be confused with a regular string
#[wasm_bindgen]
pub fn encode_json_str_to_metadatum(json: String) -> Result<TransactionMetadatum, JsValue> {
    encode_json_value_to_metadatum(serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?)
}

pub fn encode_json_value_to_metadatum(value: serde_json::Value) -> Result<TransactionMetadatum, JsValue> {
    use serde_json::Value;
    match value {
        Value::Null => Err(JsValue::from_str("null not allowed in metadata")),
        Value::Bool(_) => Err(JsValue::from_str("bools not allowed in metadata")),
        Value::Number(x) => {
            if let Some(x) = x.as_u64() {
                Ok(TransactionMetadatum::new_int(&Int::new(utils::to_bignum(x))))
            } else if let Some(x) = x.as_i64() {
                Ok(TransactionMetadatum::new_int(&Int::new_negative(utils::to_bignum(-x as u64))))
            } else {
                Err(JsValue::from_str("floats not allowed in metadata"))
            }
        },
        Value::String(s) => Ok(TransactionMetadatum::new_text(s)),
        Value::Array(json_arr) => {
            let mut arr = MetadataList::new();
            for value in json_arr {
                arr.add(&encode_json_value_to_metadatum(value)?);
            }
            Ok(TransactionMetadatum::new_list(&arr))
        },
        Value::Object(json_obj) => {
            let mut map = MetadataMap::new();
            for (key, value) in json_obj {
                map.insert(
                    &TransactionMetadatum::new_text(key),
                    &encode_json_value_to_metadatum(value)?);
            }
            Ok(TransactionMetadatum::new_map(&map))
        },
    }
}

// decodes a metadatum into a JSON object string if possible
// Bytes are not supported, see encoding comment.
#[wasm_bindgen]
pub fn decode_metadatum_to_json_str(metadatum: &TransactionMetadatum) -> Result<String, JsValue> {
    let value = decode_metadatum_to_json_value(metadatum)?;
    serde_json::to_string(&value).map_err(|e| JsValue::from_str(&e.to_string()))
}

pub fn decode_metadatum_to_json_value(metadatum: &TransactionMetadatum) -> Result<serde_json::Value, JsValue> {
    use serde_json::Value;
    use std::convert::TryFrom;
    match &metadatum.0 {
        TransactionMetadatumEnum::MetadataMap(map) => {
            let mut json_map = serde_json::map::Map::with_capacity(map.len());
            for (key, value) in map.0.iter() {
                json_map.insert(
                    match &key.0 {
                        TransactionMetadatumEnum::Text(s) => s.clone(),
                        _ => return Err(JsValue::from_str("non-string keys not allowed in JSON")),
                    },
                    decode_metadatum_to_json_value(value)?
                );
            }
            Ok(Value::from(json_map))
        },
        TransactionMetadatumEnum::MetadataList(arr) => {
            Ok(Value::from(arr.0.iter().map(decode_metadatum_to_json_value).collect::<Result<Vec<_>, JsValue>>()?))
        },
        TransactionMetadatumEnum::Int(x) => if x.0 >= 0 {
            Ok(Value::from(u64::try_from(x.0).map_err(|e| JsValue::from_str(&e.to_string()))?))
        } else {
            Ok(Value::from(i64::try_from(x.0).map_err(|e| JsValue::from_str(&e.to_string()))?))
        },
        TransactionMetadatumEnum::Bytes(_) => Err(JsValue::from_str("bytes not allowed in JSON")),
        TransactionMetadatumEnum::Text(s) => Ok(Value::from(s.clone())),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_encoding() {
        let input_bytes = (0..1000).map(|x| x as u8).collect::<Vec<u8>>();
        let metadata = encode_arbitrary_bytes_as_metadatum(input_bytes.as_ref());
        let output_bytes = decode_arbitrary_bytes_from_metadatum(&metadata).expect("decode failed");
        assert_eq!(input_bytes, output_bytes);
    }

    #[test]
    fn json_encoding() {
        let input_str = String::from("{\"receiver_id\": \"SJKdj34k3jjKFDKfjFUDfdjkfd\",\"sender_id\": \"jkfdsufjdk34h3Sdfjdhfduf873\",\"comment\": \"happy birthday\",\"tags\": [0, 264, -1024, 32]}");
        let metadata = encode_json_str_to_metadatum(input_str.clone()).expect("encode failed");
        let output_str = decode_metadatum_to_json_str(&metadata).expect("decode failed");
        let input_json: serde_json::Value = serde_json::from_str(&input_str).unwrap();
        let output_json: serde_json::Value= serde_json::from_str(&output_str).unwrap();
        assert_eq!(input_json, output_json);
    }
}