use crate::*;
use hashlink::LinkedHashMap;

const MD_MAX_LEN: usize = 64;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetadataMap(pub(crate) LinkedHashMap<TransactionMetadatum, TransactionMetadatum>);

to_from_bytes!(MetadataMap);

#[wasm_bindgen]
impl MetadataMap {
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
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

    // convenience function for inserting as a string key
    pub fn insert_str(
        &mut self,
        key: &str,
        value: &TransactionMetadatum,
    ) -> Result<Option<TransactionMetadatum>, JsError> {
        Ok(self.insert(&TransactionMetadatum::new_text(key.to_owned())?, value))
    }

    // convenience function for inserting 32-bit integers - for higher-precision integers use insert() with an Int struct
    pub fn insert_i32(
        &mut self,
        key: i32,
        value: &TransactionMetadatum,
    ) -> Option<TransactionMetadatum> {
        self.insert(&TransactionMetadatum::new_int(&Int::new_i32(key)), value)
    }

    pub fn get(&self, key: &TransactionMetadatum) -> Result<TransactionMetadatum, JsError> {
        self.0
            .get(key)
            .map(|v| v.clone())
            .ok_or_else(|| JsError::from_str(&format!("key {:?} not found", key)))
    }

    // convenience function for retrieving a string key
    pub fn get_str(&self, key: &str) -> Result<TransactionMetadatum, JsError> {
        self.get(&TransactionMetadatum::new_text(key.to_owned())?)
    }

    // convenience function for retrieving 32-bit integer keys - for higher-precision integers use get() with an Int struct
    pub fn get_i32(&self, key: i32) -> Result<TransactionMetadatum, JsError> {
        self.get(&TransactionMetadatum::new_int(&Int::new_i32(key)))
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
pub struct MetadataList(pub(crate) Vec<TransactionMetadatum>);

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
pub(crate) enum TransactionMetadatumEnum {
    MetadataMap(MetadataMap),
    MetadataList(MetadataList),
    Int(Int),
    Bytes(Vec<u8>),
    Text(String),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransactionMetadatum(pub(crate) TransactionMetadatumEnum);

to_from_bytes!(TransactionMetadatum);

#[wasm_bindgen]
impl TransactionMetadatum {
    pub fn new_map(map: &MetadataMap) -> Self {
        Self(TransactionMetadatumEnum::MetadataMap(map.clone()))
    }

    pub fn new_list(list: &MetadataList) -> Self {
        Self(TransactionMetadatumEnum::MetadataList(list.clone()))
    }

    pub fn new_int(int: &Int) -> Self {
        Self(TransactionMetadatumEnum::Int(int.clone()))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Result<TransactionMetadatum, JsError> {
        if bytes.len() > MD_MAX_LEN {
            Err(JsError::from_str(&format!(
                "Max metadata bytes too long: {}, max = {}",
                bytes.len(),
                MD_MAX_LEN
            )))
        } else {
            Ok(Self(TransactionMetadatumEnum::Bytes(bytes)))
        }
    }

    pub fn new_text(text: String) -> Result<TransactionMetadatum, JsError> {
        if text.len() > MD_MAX_LEN {
            Err(JsError::from_str(&format!(
                "Max metadata string too long: {}, max = {}",
                text.len(),
                MD_MAX_LEN
            )))
        } else {
            Ok(Self(TransactionMetadatumEnum::Text(text)))
        }
    }

    pub fn kind(&self) -> TransactionMetadatumKind {
        match &self.0 {
            TransactionMetadatumEnum::MetadataMap(_) => TransactionMetadatumKind::MetadataMap,
            TransactionMetadatumEnum::MetadataList(_) => TransactionMetadatumKind::MetadataList,
            TransactionMetadatumEnum::Int(_) => TransactionMetadatumKind::Int,
            TransactionMetadatumEnum::Bytes(_) => TransactionMetadatumKind::Bytes,
            TransactionMetadatumEnum::Text(_) => TransactionMetadatumKind::Text,
        }
    }

    pub fn as_map(&self) -> Result<MetadataMap, JsError> {
        match &self.0 {
            TransactionMetadatumEnum::MetadataMap(x) => Ok(x.clone()),
            _ => Err(JsError::from_str("not a map")),
        }
    }

    pub fn as_list(&self) -> Result<MetadataList, JsError> {
        match &self.0 {
            TransactionMetadatumEnum::MetadataList(x) => Ok(x.clone()),
            _ => Err(JsError::from_str("not a list")),
        }
    }

    pub fn as_int(&self) -> Result<Int, JsError> {
        match &self.0 {
            TransactionMetadatumEnum::Int(x) => Ok(x.clone()),
            _ => Err(JsError::from_str("not an int")),
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, JsError> {
        match &self.0 {
            TransactionMetadatumEnum::Bytes(x) => Ok(x.clone()),
            _ => Err(JsError::from_str("not bytes")),
        }
    }

    pub fn as_text(&self) -> Result<String, JsError> {
        match &self.0 {
            TransactionMetadatumEnum::Text(x) => Ok(x.clone()),
            _ => Err(JsError::from_str("not text")),
        }
    }
}

impl serde::Serialize for TransactionMetadatum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json_str = decode_metadatum_to_json_str(self, MetadataJsonSchema::DetailedSchema)
            .map_err(|e| serde::ser::Error::custom(&format!("{:?}", e)))?;
        serializer.serialize_str(&json_str)
    }
}

impl<'de> serde::de::Deserialize<'de> for TransactionMetadatum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        encode_json_str_to_metadatum(s.clone(), MetadataJsonSchema::DetailedSchema).map_err(|e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &format!("{:?}", e).as_str(),
            )
        })
    }
}

// just for now we'll do json-in-json until I can figure this out better
// TODO: maybe not generate this? or how do we do this?
impl JsonSchema for TransactionMetadatum {
    fn schema_name() -> String {
        String::from("TransactionMetadatum")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

pub type TransactionMetadatumLabel = BigNum;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionMetadatumLabels(pub(crate) Vec<TransactionMetadatumLabel>);

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
pub struct GeneralTransactionMetadata(
    pub(crate) LinkedHashMap<TransactionMetadatumLabel, TransactionMetadatum>,
);

impl_to_from!(GeneralTransactionMetadata);

#[wasm_bindgen]
impl GeneralTransactionMetadata {
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
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

impl serde::Serialize for GeneralTransactionMetadata {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = self.0.iter().collect::<std::collections::BTreeMap<_, _>>();
        map.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for GeneralTransactionMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let map = <std::collections::BTreeMap<_, _> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self(map.into_iter().collect()))
    }
}

impl JsonSchema for GeneralTransactionMetadata {
    fn schema_name() -> String {
        String::from("GeneralTransactionMetadata")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        std::collections::BTreeMap::<TransactionMetadatumLabel, TransactionMetadatum>::json_schema(
            gen,
        )
    }
    fn is_referenceable() -> bool {
        std::collections::BTreeMap::<TransactionMetadatumLabel, TransactionMetadatum>::is_referenceable()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Ord, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct AuxiliaryData {
    pub(crate) metadata: Option<GeneralTransactionMetadata>,
    pub(crate) native_scripts: Option<NativeScripts>,
    pub(crate) plutus_scripts: Option<PlutusScripts>,
    pub(crate) prefer_alonzo_format: bool,
}

impl std::cmp::PartialEq<Self> for AuxiliaryData {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.eq(&other.metadata)
            && self.native_scripts.eq(&other.native_scripts)
            && self.plutus_scripts.eq(&other.plutus_scripts)
    }
}

impl std::cmp::Eq for AuxiliaryData {}

impl_to_from!(AuxiliaryData);

#[wasm_bindgen]
impl AuxiliaryData {
    pub fn new() -> Self {
        Self {
            metadata: None,
            native_scripts: None,
            plutus_scripts: None,
            prefer_alonzo_format: false,
        }
    }

    pub fn metadata(&self) -> Option<GeneralTransactionMetadata> {
        self.metadata.clone()
    }

    pub fn set_metadata(&mut self, metadata: &GeneralTransactionMetadata) {
        self.metadata = Some(metadata.clone());
    }

    pub fn native_scripts(&self) -> Option<NativeScripts> {
        self.native_scripts.clone()
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScripts) {
        self.native_scripts = Some(native_scripts.clone())
    }

    pub fn plutus_scripts(&self) -> Option<PlutusScripts> {
        self.plutus_scripts.clone()
    }

    pub fn set_plutus_scripts(&mut self, plutus_scripts: &PlutusScripts) {
        self.plutus_scripts = Some(plutus_scripts.clone())
    }

    pub fn prefer_alonzo_format(&self) -> bool {
        self.prefer_alonzo_format.clone()
    }

    pub fn set_prefer_alonzo_format(&mut self, prefer: bool) {
        self.prefer_alonzo_format = prefer
    }
}

// encodes arbitrary bytes into chunks of 64 bytes (the limit for bytes) as a list to be valid Metadata
#[wasm_bindgen]
pub fn encode_arbitrary_bytes_as_metadatum(bytes: &[u8]) -> TransactionMetadatum {
    let mut list = MetadataList::new();
    for chunk in bytes.chunks(MD_MAX_LEN) {
        // this should never fail as we are already chunking it
        list.add(&TransactionMetadatum::new_bytes(chunk.to_vec()).unwrap());
    }
    TransactionMetadatum::new_list(&list)
}

// decodes from chunks of bytes in a list to a byte vector if that is the metadata format, otherwise returns None
#[wasm_bindgen]
pub fn decode_arbitrary_bytes_from_metadatum(
    metadata: &TransactionMetadatum,
) -> Result<Vec<u8>, JsError> {
    let mut bytes = Vec::new();
    for elem in metadata.as_list()?.0 {
        bytes.append(&mut elem.as_bytes()?);
    }
    Ok(bytes)
}

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq)]
// Different schema methods for mapping between JSON and the metadata CBOR.
// This conversion should match TxMetadataJsonSchema in cardano-node defined (at time of writing) here:
// https://github.com/input-output-hk/cardano-node/blob/master/cardano-api/src/Cardano/Api/MetaData.hs
// but has 2 additional schemas for more or less conversionse
// Note: Byte/Strings (including keys) in any schema must be at most 64 bytes in length
pub enum MetadataJsonSchema {
    // Does zero implicit conversions.
    // Round-trip conversions are 100% consistent
    // Treats maps DIRECTLY as maps in JSON in a natural way e.g. {"key1": 47, "key2": [0, 1]]}
    // From JSON:
    // * null/true/false NOT supported.
    // * keys treated as strings only
    // To JSON
    // * Bytes, non-string keys NOT supported.
    // Stricter than any TxMetadataJsonSchema in cardano-node but more natural for JSON -> Metadata
    NoConversions,
    // Does some implicit conversions.
    // Round-trip conversions MD -> JSON -> MD is NOT consistent, but JSON -> MD -> JSON is.
    // Without using bytes
    // Maps are treated as an array of k-v pairs as such: [{"key1": 47}, {"key2": [0, 1]}, {"key3": "0xFFFF"}]
    // From JSON:
    // * null/true/false NOT supported.
    // * Strings parseable as bytes (0x starting hex) or integers are converted.
    // To JSON:
    // * Non-string keys partially supported (bytes as 0x starting hex string, integer converted to string).
    // * Bytes are converted to hex strings starting with 0x for both values and keys.
    // Corresponds to TxMetadataJsonSchema's TxMetadataJsonNoSchema in cardano-node
    BasicConversions,
    // Supports the annotated schema presented in cardano-node with tagged values e.g. {"int": 7}, {"list": [0, 1]}
    // Round-trip conversions are 100% consistent
    // Maps are treated as an array of k-v pairs as such: [{"key1": {"int": 47}}, {"key2": {"list": [0, 1]}}, {"key3": {"bytes": "0xFFFF"}}]
    // From JSON:
    // * null/true/false NOT supported.
    // * Strings parseable as bytes (hex WITHOUT 0x prefix) or integers converted.
    // To JSON:
    // * Non-string keys are supported. Any key parseable as JSON is encoded as metadata instead of a string
    // Corresponds to TxMetadataJsonSchema's TxMetadataJsonDetailedSchema in cardano-node
    DetailedSchema,
}

fn supports_tagged_values(schema: MetadataJsonSchema) -> bool {
    match schema {
        MetadataJsonSchema::NoConversions | MetadataJsonSchema::BasicConversions => false,
        MetadataJsonSchema::DetailedSchema => true,
    }
}

fn hex_string_to_bytes(hex: &str) -> Option<Vec<u8>> {
    if hex.starts_with("0x") {
        hex::decode(&hex[2..]).ok()
    } else {
        None
    }
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

// Converts JSON to Metadata according to MetadataJsonSchema
#[wasm_bindgen]
pub fn encode_json_str_to_metadatum(
    json: String,
    schema: MetadataJsonSchema,
) -> Result<TransactionMetadatum, JsError> {
    let value = serde_json::from_str(&json).map_err(|e| JsError::from_str(&e.to_string()))?;
    encode_json_value_to_metadatum(value, schema)
}

pub fn encode_json_value_to_metadatum(
    value: serde_json::Value,
    schema: MetadataJsonSchema,
) -> Result<TransactionMetadatum, JsError> {
    use serde_json::Value;
    fn encode_number(x: serde_json::Number) -> Result<TransactionMetadatum, JsError> {
        if let Some(x) = x.as_u64() {
            Ok(TransactionMetadatum::new_int(&Int::new(&x.into())))
        } else if let Some(x) = x.as_i64() {
            Ok(TransactionMetadatum::new_int(&Int::new_negative(
                &(-x as u64).into(),
            )))
        } else {
            Err(JsError::from_str("floats not allowed in metadata"))
        }
    }
    fn encode_string(
        s: String,
        schema: MetadataJsonSchema,
    ) -> Result<TransactionMetadatum, JsError> {
        if schema == MetadataJsonSchema::BasicConversions {
            match hex_string_to_bytes(&s) {
                Some(bytes) => TransactionMetadatum::new_bytes(bytes),
                None => TransactionMetadatum::new_text(s),
            }
        } else {
            TransactionMetadatum::new_text(s)
        }
    }
    fn encode_array(
        json_arr: Vec<Value>,
        schema: MetadataJsonSchema,
    ) -> Result<TransactionMetadatum, JsError> {
        let mut arr = MetadataList::new();
        for value in json_arr {
            arr.add(&encode_json_value_to_metadatum(value, schema)?);
        }
        Ok(TransactionMetadatum::new_list(&arr))
    }
    match schema {
        MetadataJsonSchema::NoConversions | MetadataJsonSchema::BasicConversions => match value {
            Value::Null => Err(JsError::from_str("null not allowed in metadata")),
            Value::Bool(_) => Err(JsError::from_str("bools not allowed in metadata")),
            Value::Number(x) => encode_number(x),
            Value::String(s) => encode_string(s, schema),
            Value::Array(json_arr) => encode_array(json_arr, schema),
            Value::Object(json_obj) => {
                let mut map = MetadataMap::new();
                for (raw_key, value) in json_obj {
                    let key = if schema == MetadataJsonSchema::BasicConversions {
                        match raw_key.parse::<i128>() {
                            Ok(x) => TransactionMetadatum::new_int(&Int(x)),
                            Err(_) => encode_string(raw_key, schema)?,
                        }
                    } else {
                        TransactionMetadatum::new_text(raw_key)?
                    };
                    map.insert(&key, &encode_json_value_to_metadatum(value, schema)?);
                }
                Ok(TransactionMetadatum::new_map(&map))
            }
        },
        // we rely on tagged objects to control parsing here instead
        MetadataJsonSchema::DetailedSchema => match value {
            Value::Object(obj) if obj.len() == 1 => {
                let (k, v) = obj.into_iter().next().unwrap();
                fn tag_mismatch() -> JsError {
                    JsError::from_str("key does not match type")
                }
                match k.as_str() {
                    "int" => match v {
                        Value::Number(x) => encode_number(x),
                        _ => Err(tag_mismatch()),
                    },
                    "string" => {
                        encode_string(v.as_str().ok_or_else(tag_mismatch)?.to_owned(), schema)
                    }
                    "bytes" => match hex::decode(v.as_str().ok_or_else(tag_mismatch)?) {
                        Ok(bytes) => TransactionMetadatum::new_bytes(bytes),
                        Err(_) => Err(JsError::from_str(
                            "invalid hex string in tagged byte-object",
                        )),
                    },
                    "list" => encode_array(v.as_array().ok_or_else(tag_mismatch)?.clone(), schema),
                    "map" => {
                        let mut map = MetadataMap::new();
                        fn map_entry_err() -> JsError {
                            JsError::from_str("entry format in detailed schema map object not correct. Needs to be of form {\"k\": \"key\", \"v\": value}")
                        }
                        for entry in v.as_array().ok_or_else(tag_mismatch)? {
                            let entry_obj = entry.as_object().ok_or_else(map_entry_err)?;
                            let raw_key = entry_obj.get("k").ok_or_else(map_entry_err)?;
                            let value = entry_obj.get("v").ok_or_else(map_entry_err)?;
                            let key = encode_json_value_to_metadatum(raw_key.clone(), schema)?;
                            map.insert(
                                &key,
                                &encode_json_value_to_metadatum(value.clone(), schema)?,
                            );
                        }
                        Ok(TransactionMetadatum::new_map(&map))
                    }
                    invalid_key => Err(JsError::from_str(&format!(
                        "key '{}' in tagged object not valid",
                        invalid_key
                    ))),
                }
            }
            _ => Err(JsError::from_str(
                "DetailedSchema requires types to be tagged objects",
            )),
        },
    }
}

// Converts Metadata to JSON according to MetadataJsonSchema
#[wasm_bindgen]
pub fn decode_metadatum_to_json_str(
    metadatum: &TransactionMetadatum,
    schema: MetadataJsonSchema,
) -> Result<String, JsError> {
    let value = decode_metadatum_to_json_value(metadatum, schema)?;
    serde_json::to_string(&value).map_err(|e| JsError::from_str(&e.to_string()))
}

pub fn decode_metadatum_to_json_value(
    metadatum: &TransactionMetadatum,
    schema: MetadataJsonSchema,
) -> Result<serde_json::Value, JsError> {
    use serde_json::Value;
    use std::convert::TryFrom;
    fn decode_key(
        key: &TransactionMetadatum,
        schema: MetadataJsonSchema,
    ) -> Result<String, JsError> {
        match &key.0 {
            TransactionMetadatumEnum::Text(s) => Ok(s.clone()),
            TransactionMetadatumEnum::Bytes(b) if schema != MetadataJsonSchema::NoConversions => {
                Ok(bytes_to_hex_string(b.as_ref()))
            }
            TransactionMetadatumEnum::Int(i) if schema != MetadataJsonSchema::NoConversions => {
                let int_str = if i.0 >= 0 {
                    u64::try_from(i.0).map(|x| x.to_string())
                } else {
                    i64::try_from(i.0).map(|x| x.to_string())
                };
                int_str.map_err(|e| JsError::from_str(&e.to_string()))
            }
            TransactionMetadatumEnum::MetadataList(list)
                if schema == MetadataJsonSchema::DetailedSchema =>
            {
                decode_metadatum_to_json_str(&TransactionMetadatum::new_list(&list), schema)
            }
            TransactionMetadatumEnum::MetadataMap(map)
                if schema == MetadataJsonSchema::DetailedSchema =>
            {
                decode_metadatum_to_json_str(&TransactionMetadatum::new_map(&map), schema)
            }
            _ => Err(JsError::from_str(&format!(
                "key type {:?} not allowed in JSON under specified schema",
                key.0
            ))),
        }
    }
    let (type_key, value) = match &metadatum.0 {
        TransactionMetadatumEnum::MetadataMap(map) => match schema {
            MetadataJsonSchema::NoConversions | MetadataJsonSchema::BasicConversions => {
                // treats maps directly as JSON maps
                let mut json_map = serde_json::map::Map::with_capacity(map.len());
                for (key, value) in map.0.iter() {
                    json_map.insert(
                        decode_key(key, schema)?,
                        decode_metadatum_to_json_value(value, schema)?,
                    );
                }
                ("map", Value::from(json_map))
            }

            MetadataJsonSchema::DetailedSchema => (
                "map",
                Value::from(
                    map.0
                        .iter()
                        .map(|(key, value)| {
                            // must encode maps as JSON lists of objects with k/v keys
                            // also in these schemas we support more key types than strings
                            let k = decode_metadatum_to_json_value(key, schema)?;
                            let v = decode_metadatum_to_json_value(value, schema)?;
                            let mut kv_obj = serde_json::map::Map::with_capacity(2);
                            kv_obj.insert(String::from("k"), Value::from(k));
                            kv_obj.insert(String::from("v"), v);
                            Ok(Value::from(kv_obj))
                        })
                        .collect::<Result<Vec<_>, JsError>>()?,
                ),
            ),
        },
        TransactionMetadatumEnum::MetadataList(arr) => (
            "list",
            Value::from(
                arr.0
                    .iter()
                    .map(|e| decode_metadatum_to_json_value(e, schema))
                    .collect::<Result<Vec<_>, JsError>>()?,
            ),
        ),
        TransactionMetadatumEnum::Int(x) => (
            "int",
            if x.0 >= 0 {
                Value::from(u64::try_from(x.0).map_err(|e| JsError::from_str(&e.to_string()))?)
            } else {
                Value::from(i64::try_from(x.0).map_err(|e| JsError::from_str(&e.to_string()))?)
            },
        ),
        TransactionMetadatumEnum::Bytes(bytes) => (
            "bytes",
            match schema {
                MetadataJsonSchema::NoConversions => Err(JsError::from_str(
                    "bytes not allowed in JSON in specified schema",
                )),
                // 0x prefix
                MetadataJsonSchema::BasicConversions => {
                    Ok(Value::from(bytes_to_hex_string(bytes.as_ref())))
                }
                // no prefix
                MetadataJsonSchema::DetailedSchema => Ok(Value::from(hex::encode(bytes))),
            }?,
        ),
        TransactionMetadatumEnum::Text(s) => ("string", Value::from(s.clone())),
    };
    // potentially wrap value in a keyed map to represent more types
    if supports_tagged_values(schema) {
        let mut wrapper = serde_json::map::Map::with_capacity(1);
        wrapper.insert(String::from(type_key), value);
        Ok(Value::from(wrapper))
    } else {
        Ok(value)
    }
}