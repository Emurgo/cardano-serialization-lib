use crate::*;
use hashlink::LinkedHashMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

const MD_MAX_LEN: usize = 64;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    pub(crate) fn insert_move(
        &mut self,
        key: TransactionMetadatum,
        value: TransactionMetadatum,
    ) -> Option<TransactionMetadatum> {
        self.0.insert(key, value)
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

    pub(crate) fn from_vec(elems: Vec<TransactionMetadatum>) -> Self {
        Self(elems)
    }
}

impl_vec_wrapper!(MetadataList, TransactionMetadatum);

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

impl TransactionMetadatum {
    pub(crate) fn from_map_moved(map: MetadataMap) -> Self {
        Self(TransactionMetadatumEnum::MetadataMap(map))
    }

    pub(crate) fn from_list_moved(list: MetadataList) -> Self {
        Self(TransactionMetadatumEnum::MetadataList(list))
    }

    pub(crate) fn into_enum(self) -> TransactionMetadatumEnum {
        let this = std::mem::ManuallyDrop::new(self);
        unsafe { std::ptr::read(&this.0) }
    }

    fn take_children(&mut self, stack: &mut Vec<TransactionMetadatum>) {
        match &mut self.0 {
            TransactionMetadatumEnum::MetadataList(list) => {
                stack.extend(std::mem::take(&mut list.0));
            }
            TransactionMetadatumEnum::MetadataMap(map) => {
                for (k, v) in std::mem::take(&mut map.0) {
                    stack.push(k);
                    stack.push(v);
                }
            }
            _ => {}
        }
    }

    fn kind_index(&self) -> u8 {
        match &self.0 {
            TransactionMetadatumEnum::MetadataMap(_) => 0,
            TransactionMetadatumEnum::MetadataList(_) => 1,
            TransactionMetadatumEnum::Int(_) => 2,
            TransactionMetadatumEnum::Bytes(_) => 3,
            TransactionMetadatumEnum::Text(_) => 4,
        }
    }
}

impl Drop for TransactionMetadatum {
    fn drop(&mut self) {
        crate::stack_safe::iterative_drop(self, |node, stack| node.take_children(stack));
    }
}

enum MetadatumCloneKind {
    List,
    Map,
}

struct MetadatumCloneFrame<'a> {
    kind: MetadatumCloneKind,
    rest: std::vec::IntoIter<&'a TransactionMetadatum>,
    acc: Vec<TransactionMetadatum>,
}

fn finish_metadatum_clone(
    kind: MetadatumCloneKind,
    acc: Vec<TransactionMetadatum>,
) -> TransactionMetadatum {
    match kind {
        MetadatumCloneKind::List => {
            TransactionMetadatum::from_list_moved(MetadataList::from_vec(acc))
        }
        MetadatumCloneKind::Map => {
            let mut map = MetadataMap::new();
            let mut iter = acc.into_iter();
            while let Some(key) = iter.next() {
                let value = iter.next().expect("metadata map clone k/v pairs");
                map.insert_move(key, value);
            }
            TransactionMetadatum::from_map_moved(map)
        }
    }
}

impl Clone for TransactionMetadatum {
    fn clone(&self) -> Self {
        let mut stack: Vec<MetadatumCloneFrame<'_>> = Vec::new();
        let mut node = self;
        loop {
            let mut completed = match &node.0 {
                TransactionMetadatumEnum::Int(x) => TransactionMetadatum::new_int(x),
                TransactionMetadatumEnum::Bytes(x) => {
                    TransactionMetadatum::new_bytes(x.clone()).expect("cloned bytes within limit")
                }
                TransactionMetadatumEnum::Text(x) => {
                    TransactionMetadatum::new_text(x.clone()).expect("cloned text within limit")
                }
                TransactionMetadatumEnum::MetadataList(list) => {
                    let mut rest = list.0.iter();
                    if let Some(first) = rest.next() {
                        stack.push(MetadatumCloneFrame {
                            kind: MetadatumCloneKind::List,
                            rest: rest.collect::<Vec<_>>().into_iter(),
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_metadatum_clone(MetadatumCloneKind::List, Vec::new())
                }
                TransactionMetadatumEnum::MetadataMap(map) => {
                    let mut children = Vec::new();
                    for (k, v) in map.0.iter() {
                        children.push(k);
                        children.push(v);
                    }
                    let mut rest = children.into_iter();
                    if let Some(first) = rest.next() {
                        stack.push(MetadatumCloneFrame {
                            kind: MetadatumCloneKind::Map,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_metadatum_clone(MetadatumCloneKind::Map, Vec::new())
                }
            };
            loop {
                match stack.last_mut() {
                    None => return completed,
                    Some(frame) => {
                        frame.acc.push(completed);
                        if let Some(next) = frame.rest.next() {
                            node = next;
                            break;
                        } else {
                            let frame = stack.pop().unwrap();
                            completed = finish_metadatum_clone(frame.kind, frame.acc);
                        }
                    }
                }
            }
        }
    }
}

impl PartialEq for TransactionMetadatum {
    fn eq(&self, other: &Self) -> bool {
        let mut stack = vec![(self, other)];
        while let Some((left, right)) = stack.pop() {
            match (&left.0, &right.0) {
                (TransactionMetadatumEnum::Int(x), TransactionMetadatumEnum::Int(y)) => {
                    if x != y {
                        return false;
                    }
                }
                (TransactionMetadatumEnum::Bytes(x), TransactionMetadatumEnum::Bytes(y)) => {
                    if x != y {
                        return false;
                    }
                }
                (TransactionMetadatumEnum::Text(x), TransactionMetadatumEnum::Text(y)) => {
                    if x != y {
                        return false;
                    }
                }
                (
                    TransactionMetadatumEnum::MetadataList(x),
                    TransactionMetadatumEnum::MetadataList(y),
                ) => {
                    if x.0.len() != y.0.len() {
                        return false;
                    }
                    stack.extend(x.0.iter().zip(y.0.iter()));
                }
                (
                    TransactionMetadatumEnum::MetadataMap(x),
                    TransactionMetadatumEnum::MetadataMap(y),
                ) => {
                    if x.0.len() != y.0.len() {
                        return false;
                    }
                    for ((lk, lv), (rk, rv)) in x.0.iter().zip(y.0.iter()) {
                        stack.push((lv, rv));
                        stack.push((lk, rk));
                    }
                }
                _ => return false,
            }
        }
        true
    }
}

impl Eq for TransactionMetadatum {}

impl Hash for TransactionMetadatum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        enum Job<'a> {
            Node(&'a TransactionMetadatum),
            Disc(u8),
            Len(usize),
            Int(Int),
            Bytes(&'a [u8]),
            Text(&'a str),
        }
        let mut stack = vec![Job::Node(self)];
        while let Some(job) = stack.pop() {
            match job {
                Job::Disc(d) => d.hash(state),
                Job::Len(n) => n.hash(state),
                Job::Int(i) => i.hash(state),
                Job::Bytes(b) => b.hash(state),
                Job::Text(s) => s.hash(state),
                Job::Node(node) => match &node.0 {
                    TransactionMetadatumEnum::MetadataMap(map) => {
                        for (k, v) in map.0.iter().rev() {
                            stack.push(Job::Node(v));
                            stack.push(Job::Node(k));
                        }
                        stack.push(Job::Disc(0));
                    }
                    TransactionMetadatumEnum::MetadataList(list) => {
                        for elem in list.0.iter().rev() {
                            stack.push(Job::Node(elem));
                        }
                        stack.push(Job::Len(list.0.len()));
                        stack.push(Job::Disc(1));
                    }
                    TransactionMetadatumEnum::Int(x) => {
                        stack.push(Job::Int(*x));
                        stack.push(Job::Disc(2));
                    }
                    TransactionMetadatumEnum::Bytes(x) => {
                        stack.push(Job::Bytes(x));
                        stack.push(Job::Disc(3));
                    }
                    TransactionMetadatumEnum::Text(x) => {
                        stack.push(Job::Text(x));
                        stack.push(Job::Disc(4));
                    }
                },
            }
        }
    }
}

enum MetadatumCmpJob<'a> {
    Node(&'a TransactionMetadatum, &'a TransactionMetadatum),
    Kind(u8, u8),
    Lens(usize, usize),
    Ints(Int, Int),
    Bytes(&'a [u8], &'a [u8]),
    Text(&'a str, &'a str),
}

impl PartialOrd for TransactionMetadatum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TransactionMetadatum {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut stack = vec![MetadatumCmpJob::Node(self, other)];
        while let Some(job) = stack.pop() {
            let ord = match job {
                MetadatumCmpJob::Kind(a, b) => a.cmp(&b),
                MetadatumCmpJob::Lens(a, b) => a.cmp(&b),
                MetadatumCmpJob::Ints(a, b) => a.cmp(&b),
                MetadatumCmpJob::Bytes(a, b) => a.cmp(&b),
                MetadatumCmpJob::Text(a, b) => a.cmp(&b),
                MetadatumCmpJob::Node(a, b) => {
                    match (&a.0, &b.0) {
                        (TransactionMetadatumEnum::Int(x), TransactionMetadatumEnum::Int(y)) => {
                            stack.push(MetadatumCmpJob::Ints(*x, *y));
                        }
                        (
                            TransactionMetadatumEnum::Bytes(x),
                            TransactionMetadatumEnum::Bytes(y),
                        ) => {
                            stack.push(MetadatumCmpJob::Bytes(x, y));
                        }
                        (TransactionMetadatumEnum::Text(x), TransactionMetadatumEnum::Text(y)) => {
                            stack.push(MetadatumCmpJob::Text(x, y));
                        }
                        (
                            TransactionMetadatumEnum::MetadataList(x),
                            TransactionMetadatumEnum::MetadataList(y),
                        ) => {
                            stack.push(MetadatumCmpJob::Lens(x.0.len(), y.0.len()));
                            let min_len = x.0.len().min(y.0.len());
                            for i in (0..min_len).rev() {
                                stack.push(MetadatumCmpJob::Node(&x.0[i], &y.0[i]));
                            }
                        }
                        (
                            TransactionMetadatumEnum::MetadataMap(x),
                            TransactionMetadatumEnum::MetadataMap(y),
                        ) => {
                            let xs: Vec<_> = x.0.iter().collect();
                            let ys: Vec<_> = y.0.iter().collect();
                            stack.push(MetadatumCmpJob::Lens(xs.len(), ys.len()));
                            let min_len = xs.len().min(ys.len());
                            for i in (0..min_len).rev() {
                                let (lk, lv) = xs[i];
                                let (rk, rv) = ys[i];
                                stack.push(MetadatumCmpJob::Node(lv, rv));
                                stack.push(MetadatumCmpJob::Node(lk, rk));
                            }
                        }
                        _ => {}
                    }
                    stack.push(MetadatumCmpJob::Kind(a.kind_index(), b.kind_index()));
                    Ordering::Equal
                }
            };
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    }
}

impl std::fmt::Debug for TransactionMetadatum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            TransactionMetadatumEnum::Int(x) => f.debug_tuple("TransactionMetadatum").field(x).finish(),
            TransactionMetadatumEnum::Bytes(x) => {
                f.debug_tuple("TransactionMetadatum").field(x).finish()
            }
            TransactionMetadatumEnum::Text(x) => {
                f.debug_tuple("TransactionMetadatum").field(x).finish()
            }
            TransactionMetadatumEnum::MetadataList(x) => f
                .debug_tuple("TransactionMetadatum")
                .field(&format_args!("List(len={})", x.0.len()))
                .finish(),
            TransactionMetadatumEnum::MetadataMap(x) => f
                .debug_tuple("TransactionMetadatum")
                .field(&format_args!("Map(len={})", x.0.len()))
                .finish(),
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

impl_vec_wrapper!(TransactionMetadatumLabels, TransactionMetadatumLabel);

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Debug, Default, Ord, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema)]
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
                        match raw_key.parse::<i128>().ok().and_then(|x| Int::new_checked(x).ok()) {
                            Some(i) => TransactionMetadatum::new_int(&i),
                            None => encode_string(raw_key, schema)?,
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
    let value = metadatum_to_json_val(metadatum, schema)?;
    Ok(crate::stack_safe::stringify_json(&value, false))
}

pub fn decode_metadatum_to_json_value(
    metadatum: &TransactionMetadatum,
    schema: MetadataJsonSchema,
) -> Result<serde_json::Value, JsError> {
    let value = metadatum_to_json_val(metadatum, schema)?;
    Ok(crate::stack_safe::json_val_to_serde(value))
}

fn decode_metadatum_key_string(
    key: &TransactionMetadatum,
    schema: MetadataJsonSchema,
) -> Result<String, JsError> {
    use std::convert::TryFrom;
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
        _ => Err(JsError::from_str(&format!(
            "key type {:?} not allowed in JSON under specified schema",
            key.0
        ))),
    }
}

fn metadatum_int_json(x: &Int) -> Result<crate::stack_safe::JsonVal, JsError> {
    use std::convert::TryFrom;
    if x.0 >= 0 {
        Ok(crate::stack_safe::JsonVal::Number(
            u64::try_from(x.0)
                .map_err(|e| JsError::from_str(&e.to_string()))?
                .to_string(),
        ))
    } else {
        Ok(crate::stack_safe::JsonVal::Number(
            i64::try_from(x.0)
                .map_err(|e| JsError::from_str(&e.to_string()))?
                .to_string(),
        ))
    }
}

fn wrap_metadatum_json(
    type_key: &str,
    value: crate::stack_safe::JsonVal,
    schema: MetadataJsonSchema,
) -> crate::stack_safe::JsonVal {
    if supports_tagged_values(schema) {
        crate::stack_safe::JsonVal::Object(vec![(type_key.to_string(), value)])
    } else {
        value
    }
}

enum MetadatumToJsonKind {
    List,
    MapBasic { keys: std::vec::IntoIter<String> },
    MapDetailed,
}

struct MetadatumToJsonFrame<'a> {
    kind: MetadatumToJsonKind,
    rest: std::vec::IntoIter<&'a TransactionMetadatum>,
    acc: Vec<crate::stack_safe::JsonVal>,
}

fn finish_metadatum_to_json(
    kind: MetadatumToJsonKind,
    acc: Vec<crate::stack_safe::JsonVal>,
    schema: MetadataJsonSchema,
) -> crate::stack_safe::JsonVal {
    use crate::stack_safe::JsonVal;
    match kind {
        MetadatumToJsonKind::List => wrap_metadatum_json("list", JsonVal::Array(acc), schema),
        MetadatumToJsonKind::MapBasic { keys } => {
            let mut obj = Vec::new();
            let mut keys = keys;
            for v in acc {
                obj.push((keys.next().expect("metadata basic map json keys"), v));
            }
            // `serde_json::Map` semantics as in 16.0.0: keys sorted, a repeated key keeps its last value.
            wrap_metadatum_json("map", JsonVal::object_sorted(obj), schema)
        }
        MetadatumToJsonKind::MapDetailed => {
            let mut entries = Vec::new();
            let mut iter = acc.into_iter();
            while let Some(k) = iter.next() {
                let v = iter.next().expect("metadata detailed map k/v pairs");
                entries.push(JsonVal::Object(vec![
                    ("k".to_string(), k),
                    ("v".to_string(), v),
                ]));
            }
            wrap_metadatum_json("map", JsonVal::Array(entries), schema)
        }
    }
}

fn metadatum_to_json_val(
    root: &TransactionMetadatum,
    schema: MetadataJsonSchema,
) -> Result<crate::stack_safe::JsonVal, JsError> {
    use crate::stack_safe::JsonVal;
    let mut stack: Vec<MetadatumToJsonFrame<'_>> = Vec::new();
    let mut node = root;
    loop {
        let mut completed = match &node.0 {
            TransactionMetadatumEnum::Int(x) => {
                wrap_metadatum_json("int", metadatum_int_json(x)?, schema)
            }
            TransactionMetadatumEnum::Bytes(bytes) => {
                let s = match schema {
                    MetadataJsonSchema::NoConversions => {
                        return Err(JsError::from_str(
                            "bytes not allowed in JSON in specified schema",
                        ))
                    }
                    MetadataJsonSchema::BasicConversions => bytes_to_hex_string(bytes.as_ref()),
                    MetadataJsonSchema::DetailedSchema => hex::encode(bytes),
                };
                wrap_metadatum_json("bytes", JsonVal::String(s), schema)
            }
            TransactionMetadatumEnum::Text(s) => {
                wrap_metadatum_json("string", JsonVal::String(s.clone()), schema)
            }
            TransactionMetadatumEnum::MetadataList(arr) => {
                let mut rest = arr.0.iter();
                if let Some(first) = rest.next() {
                    stack.push(MetadatumToJsonFrame {
                        kind: MetadatumToJsonKind::List,
                        rest: rest.collect::<Vec<_>>().into_iter(),
                        acc: Vec::new(),
                    });
                    node = first;
                    continue;
                }
                finish_metadatum_to_json(MetadatumToJsonKind::List, Vec::new(), schema)
            }
            TransactionMetadatumEnum::MetadataMap(map) => match schema {
                MetadataJsonSchema::NoConversions | MetadataJsonSchema::BasicConversions => {
                    let mut keys = Vec::new();
                    let mut children = Vec::new();
                    for (key, value) in map.0.iter() {
                        keys.push(decode_metadatum_key_string(key, schema)?);
                        children.push(value);
                    }
                    let kind = MetadatumToJsonKind::MapBasic {
                        keys: keys.into_iter(),
                    };
                    let mut rest = children.into_iter();
                    if let Some(first) = rest.next() {
                        stack.push(MetadatumToJsonFrame {
                            kind,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_metadatum_to_json(kind, Vec::new(), schema)
                }
                MetadataJsonSchema::DetailedSchema => {
                    let mut children = Vec::new();
                    for (key, value) in map.0.iter() {
                        children.push(key);
                        children.push(value);
                    }
                    let kind = MetadatumToJsonKind::MapDetailed;
                    let mut rest = children.into_iter();
                    if let Some(first) = rest.next() {
                        stack.push(MetadatumToJsonFrame {
                            kind,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_metadatum_to_json(kind, Vec::new(), schema)
                }
            },
        };

        loop {
            match stack.last_mut() {
                None => return Ok(completed),
                Some(frame) => {
                    frame.acc.push(completed);
                    if let Some(next) = frame.rest.next() {
                        node = next;
                        break;
                    } else {
                        let frame = stack.pop().unwrap();
                        completed = finish_metadatum_to_json(frame.kind, frame.acc, schema);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
impl MetadataList {
    pub(crate) fn push_move(&mut self, elem: TransactionMetadatum) {
        self.0.push(elem);
    }
}
