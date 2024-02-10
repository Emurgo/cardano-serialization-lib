use crate::*;
use core::hash::Hasher;
use hashlink::LinkedHashMap;
use std::hash::Hash;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{
    self,
    de::Deserializer,
    se::{Serialize, Serializer},
};

use schemars::JsonSchema;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScript {
    pub(crate) bytes: Vec<u8>,
    pub(crate) language: LanguageKind,
}

to_from_bytes!(PlutusScript);

#[wasm_bindgen]
impl PlutusScript {
    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new(bytes: Vec<u8>) -> PlutusScript {
        Self::new_with_version(bytes, &Language::new_plutus_v1())
    }

    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new_v2(bytes: Vec<u8>) -> PlutusScript {
        Self::new_with_version(bytes, &Language::new_plutus_v2())
    }

    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new_v3(bytes: Vec<u8>) -> PlutusScript {
        Self::new_with_version(bytes, &Language::new_plutus_v3())
    }

    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new_with_version(bytes: Vec<u8>, language: &Language) -> PlutusScript {
        Self {
            bytes,
            language: language.0.clone(),
        }
    }

    /**
     * The raw bytes of this compiled Plutus script.
     * If you need "cborBytes" for cardano-cli use PlutusScript::to_bytes() instead.
     */
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    /// Same as `.from_bytes` but will consider the script as requiring the Plutus Language V2
    pub fn from_bytes_v2(bytes: Vec<u8>) -> Result<PlutusScript, JsError> {
        Self::from_bytes_with_version(bytes, &Language::new_plutus_v2())
    }

    /// Same as `.from_bytes` but will consider the script as requiring the Plutus Language V3
    pub fn from_bytes_v3(bytes: Vec<u8>) -> Result<PlutusScript, JsError> {
        Self::from_bytes_with_version(bytes, &Language::new_plutus_v3())
    }

    /// Same as `.from_bytes` but will consider the script as requiring the specified language version
    pub fn from_bytes_with_version(
        bytes: Vec<u8>,
        language: &Language,
    ) -> Result<PlutusScript, JsError> {
        Ok(Self::new_with_version(
            Self::from_bytes(bytes)?.bytes,
            language,
        ))
    }

    /// Same as .from_hex but will consider the script as requiring the specified language version
    pub fn from_hex_with_version(
        hex_str: &str,
        language: &Language,
    ) -> Result<PlutusScript, JsError> {
        Ok(Self::new_with_version(
            Self::from_hex(hex_str)?.bytes,
            language,
        ))
    }

    pub fn hash(&self) -> ScriptHash {
        let mut bytes = Vec::with_capacity(self.bytes.len() + 1);
        // https://github.com/input-output-hk/cardano-ledger/blob/master/eras/babbage/test-suite/cddl-files/babbage.cddl#L413
        bytes.extend_from_slice(&vec![self.script_namespace() as u8]);
        bytes.extend_from_slice(&self.bytes);
        ScriptHash::from(blake2b224(bytes.as_ref()))
    }

    pub fn language_version(&self) -> Language {
        Language(self.language.clone())
    }

    pub(crate) fn script_namespace(&self) -> ScriptHashNamespace {
        match self.language {
            LanguageKind::PlutusV1 => ScriptHashNamespace::PlutusScript,
            LanguageKind::PlutusV2 => ScriptHashNamespace::PlutusScriptV2,
            LanguageKind::PlutusV3 => ScriptHashNamespace::PlutusScriptV3,
        }
    }

    pub(crate) fn clone_as_version(&self, language: &Language) -> PlutusScript {
        Self::new_with_version(self.bytes.clone(), language)
    }
}

impl serde::Serialize for PlutusScript {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.bytes))
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusScript {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        hex::decode(&s)
            .map(|bytes| PlutusScript::new(bytes))
            .map_err(|_err| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&s),
                    &"PlutusScript as hex string e.g. F8AB28C2 (without CBOR bytes tag)",
                )
            })
    }
}

impl JsonSchema for PlutusScript {
    fn schema_name() -> String {
        String::from("PlutusScript")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct PlutusScripts(pub(crate) Vec<PlutusScript>);

impl_to_from!(PlutusScripts);

#[wasm_bindgen]
impl PlutusScripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusScript {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusScript) {
        self.0.push(elem.clone());
    }

    pub(crate) fn by_version(&self, language: &Language) -> PlutusScripts {
        PlutusScripts(
            self.0
                .iter()
                .filter(|s| s.language_version().eq(language))
                .map(|s| s.clone())
                .collect(),
        )
    }

    pub(crate) fn has_version(&self, language: &Language) -> bool {
        self.0.iter().any(|s| s.language_version().eq(language))
    }

    pub(crate) fn merge(&self, other: &PlutusScripts) -> PlutusScripts {
        let mut res = self.clone();
        for s in &other.0 {
            res.add(s);
        }
        res
    }

    pub(crate) fn map_as_version(&self, language: &Language) -> PlutusScripts {
        let mut res = PlutusScripts::new();
        for s in &self.0 {
            res.add(&s.clone_as_version(language));
        }
        res
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ConstrPlutusData {
    pub(crate) alternative: BigNum,
    pub(crate) data: PlutusList,
}

to_from_bytes!(ConstrPlutusData);

#[wasm_bindgen]
impl ConstrPlutusData {
    pub fn alternative(&self) -> BigNum {
        self.alternative.clone()
    }

    pub fn data(&self) -> PlutusList {
        self.data.clone()
    }

    pub fn new(alternative: &BigNum, data: &PlutusList) -> Self {
        Self {
            alternative: alternative.clone(),
            data: data.clone(),
        }
    }
}

impl ConstrPlutusData {
    // see: https://github.com/input-output-hk/plutus/blob/1f31e640e8a258185db01fa899da63f9018c0e85/plutus-core/plutus-core/src/PlutusCore/Data.hs#L61
    // We don't directly serialize the alternative in the tag, instead the scheme is:
    // - Alternatives 0-6 -> tags 121-127, followed by the arguments in a list
    // - Alternatives 7-127 -> tags 1280-1400, followed by the arguments in a list
    // - Any alternatives, including those that don't fit in the above -> tag 102 followed by a list containing
    //   an unsigned integer for the actual alternative, and then the arguments in a (nested!) list.
    pub(crate) const GENERAL_FORM_TAG: u64 = 102;

    // None -> needs general tag serialization, not compact
    pub(crate) fn alternative_to_compact_cbor_tag(alt: u64) -> Option<u64> {
        if alt <= 6 {
            Some(121 + alt)
        } else if alt >= 7 && alt <= 127 {
            Some(1280 - 7 + alt)
        } else {
            None
        }
    }

    // None -> General tag(=102) OR Invalid CBOR tag for this scheme
    pub(crate) fn compact_cbor_tag_to_alternative(cbor_tag: u64) -> Option<u64> {
        if cbor_tag >= 121 && cbor_tag <= 127 {
            Some(cbor_tag - 121)
        } else if cbor_tag >= 1280 && cbor_tag <= 1400 {
            Some(cbor_tag - 1280 + 7)
        } else {
            None
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct CostModel(pub(crate) Vec<Int>);

impl_to_from!(CostModel);

#[wasm_bindgen]
impl CostModel {
    /// Creates a new CostModels instance of an unrestricted length
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Sets the cost at the specified index to the specified value.
    /// In case the operation index is larger than the previous largest used index,
    /// it will fill any inbetween indexes with zeroes
    pub fn set(&mut self, operation: usize, cost: &Int) -> Result<Int, JsError> {
        let len = self.0.len();
        let idx = operation.clone();
        if idx >= len {
            for _ in 0..(idx - len + 1) {
                self.0.push(Int::new_i32(0));
            }
        }
        let old = self.0[idx].clone();
        self.0[idx] = cost.clone();
        Ok(old)
    }

    pub fn get(&self, operation: usize) -> Result<Int, JsError> {
        let max = self.0.len();
        if operation >= max {
            return Err(JsError::from_str(&format!(
                "CostModel operation {} out of bounds. Max is {}",
                operation, max
            )));
        }
        Ok(self.0[operation].clone())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<i128>> for CostModel {
    fn from(values: Vec<i128>) -> Self {
        CostModel(values.iter().map(|x| Int(*x)).collect())
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Costmdls(pub(crate) std::collections::BTreeMap<Language, CostModel>);

impl_to_from!(Costmdls);

#[wasm_bindgen]
impl Costmdls {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &Language, value: &CostModel) -> Option<CostModel> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &Language) -> Option<CostModel> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> Languages {
        Languages(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }

    pub(crate) fn language_views_encoding(&self) -> Vec<u8> {
        let mut serializer = Serializer::new_vec();
        fn key_len(l: &Language) -> usize {
            if l.kind() == LanguageKind::PlutusV1 {
                let mut serializer = Serializer::new_vec();
                serializer.write_bytes(l.to_bytes()).unwrap();
                return serializer.finalize().len();
            }
            l.to_bytes().len()
        }
        let mut keys: Vec<Language> = self.0.iter().map(|(k, _v)| k.clone()).collect();
        // keys must be in canonical ordering first
        keys.sort_by(|lhs, rhs| match key_len(lhs).cmp(&key_len(rhs)) {
            std::cmp::Ordering::Equal => lhs.cmp(&rhs),
            len_order => len_order,
        });
        serializer
            .write_map(cbor_event::Len::Len(self.0.len() as u64))
            .unwrap();
        for key in keys.iter() {
            if key.kind() == LanguageKind::PlutusV1 {
                serializer.write_bytes(key.to_bytes()).unwrap();
                let cost_model = self.0.get(&key).unwrap();
                // Due to a bug in the cardano-node input-output-hk/cardano-ledger-specs/issues/2512
                // we must use indefinite length serialization in this inner bytestring to match it
                let mut cost_model_serializer = Serializer::new_vec();
                cost_model_serializer
                    .write_array(cbor_event::Len::Indefinite)
                    .unwrap();
                for cost in &cost_model.0 {
                    cost.serialize(&mut cost_model_serializer).unwrap();
                }
                cost_model_serializer
                    .write_special(cbor_event::Special::Break)
                    .unwrap();
                serializer
                    .write_bytes(cost_model_serializer.finalize())
                    .unwrap();
            } else {
                serializer.serialize(key).unwrap();
                serializer.serialize(self.0.get(&key).unwrap()).unwrap();
            }
        }
        serializer.finalize()
    }

    pub fn retain_language_versions(&self, languages: &Languages) -> Costmdls {
        let mut result = Costmdls::new();
        for lang in &languages.0 {
            match self.get(&lang) {
                Some(costmodel) => {
                    result.insert(&lang, &costmodel);
                }
                _ => {}
            }
        }
        result
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct ExUnitPrices {
    pub(crate) mem_price: SubCoin,
    pub(crate) step_price: SubCoin,
}

impl_to_from!(ExUnitPrices);

#[wasm_bindgen]
impl ExUnitPrices {
    pub fn mem_price(&self) -> SubCoin {
        self.mem_price.clone()
    }

    pub fn step_price(&self) -> SubCoin {
        self.step_price.clone()
    }

    pub fn new(mem_price: &SubCoin, step_price: &SubCoin) -> Self {
        Self {
            mem_price: mem_price.clone(),
            step_price: step_price.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct ExUnits {
    pub(crate) mem: BigNum,
    pub(crate) steps: BigNum,
}

impl_to_from!(ExUnits);

#[wasm_bindgen]
impl ExUnits {
    pub fn mem(&self) -> BigNum {
        self.mem.clone()
    }

    pub fn steps(&self) -> BigNum {
        self.steps.clone()
    }

    pub fn new(mem: &BigNum, steps: &BigNum) -> Self {
        Self {
            mem: mem.clone(),
            steps: steps.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub enum LanguageKind {
    PlutusV1 = 0,
    PlutusV2 = 1,
    PlutusV3 = 2,
}

impl LanguageKind {
    pub(crate) fn from_u64(x: u64) -> Option<LanguageKind> {
        match x {
            0 => Some(LanguageKind::PlutusV1),
            1 => Some(LanguageKind::PlutusV2),
            2 => Some(LanguageKind::PlutusV3),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Language(pub(crate) LanguageKind);

impl_to_from!(Language);

#[wasm_bindgen]
impl Language {
    pub fn new_plutus_v1() -> Self {
        Self(LanguageKind::PlutusV1)
    }

    pub fn new_plutus_v2() -> Self {
        Self(LanguageKind::PlutusV2)
    }

    pub fn new_plutus_v3() -> Self {
        Self(LanguageKind::PlutusV3)
    }

    pub fn kind(&self) -> LanguageKind {
        self.0.clone()
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Languages(pub(crate) Vec<Language>);

#[wasm_bindgen]
impl Languages {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Language {
        self.0[index]
    }

    pub fn add(&mut self, elem: Language) {
        self.0.push(elem);
    }

    pub fn list() -> Languages {
        Languages(vec![Language::new_plutus_v1(), Language::new_plutus_v2()])
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct PlutusMap(pub(crate) LinkedHashMap<PlutusData, PlutusData>);

to_from_bytes!(PlutusMap);

#[wasm_bindgen]
impl PlutusMap {
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PlutusData, value: &PlutusData) -> Option<PlutusData> {
        self.0.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &PlutusData) -> Option<PlutusData> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> PlutusList {
        PlutusList {
            elems: self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>(),
            definite_encoding: None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlutusDataKind {
    ConstrPlutusData,
    Map,
    List,
    Integer,
    Bytes,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PlutusDataEnum {
    ConstrPlutusData(ConstrPlutusData),
    Map(PlutusMap),
    List(PlutusList),
    Integer(BigInt),
    Bytes(Vec<u8>),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Ord, PartialOrd)]
pub struct PlutusData {
    pub(crate) datum: PlutusDataEnum,
    // We should always preserve the original datums when deserialized as this is NOT canonicized
    // before computing datum hashes. So this field stores the original bytes to re-use.
    pub(crate) original_bytes: Option<Vec<u8>>,
}

impl std::cmp::PartialEq<Self> for PlutusData {
    fn eq(&self, other: &Self) -> bool {
        self.datum.eq(&other.datum)
    }
}

impl Hash for PlutusData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.datum.hash(state)
    }
}

impl std::cmp::Eq for PlutusData {}

to_from_bytes!(PlutusData);

#[wasm_bindgen]
impl PlutusData {
    pub fn new_constr_plutus_data(constr_plutus_data: &ConstrPlutusData) -> Self {
        Self {
            datum: PlutusDataEnum::ConstrPlutusData(constr_plutus_data.clone()),
            original_bytes: None,
        }
    }

    /// Same as `.new_constr_plutus_data` but creates constr with empty data list
    pub fn new_empty_constr_plutus_data(alternative: &BigNum) -> Self {
        Self::new_constr_plutus_data(&ConstrPlutusData::new(alternative, &PlutusList::new()))
    }

    pub fn new_single_value_constr_plutus_data(
        alternative: &BigNum,
        plutus_data: &PlutusData,
    ) -> Self {
        let mut list = PlutusList::new();
        list.add(plutus_data);
        Self::new_constr_plutus_data(&ConstrPlutusData::new(alternative, &list))
    }

    pub fn new_map(map: &PlutusMap) -> Self {
        Self {
            datum: PlutusDataEnum::Map(map.clone()),
            original_bytes: None,
        }
    }

    pub fn new_list(list: &PlutusList) -> Self {
        Self {
            datum: PlutusDataEnum::List(list.clone()),
            original_bytes: None,
        }
    }

    pub fn new_integer(integer: &BigInt) -> Self {
        Self {
            datum: PlutusDataEnum::Integer(integer.clone()),
            original_bytes: None,
        }
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self {
            datum: PlutusDataEnum::Bytes(bytes),
            original_bytes: None,
        }
    }

    pub fn kind(&self) -> PlutusDataKind {
        match &self.datum {
            PlutusDataEnum::ConstrPlutusData(_) => PlutusDataKind::ConstrPlutusData,
            PlutusDataEnum::Map(_) => PlutusDataKind::Map,
            PlutusDataEnum::List(_) => PlutusDataKind::List,
            PlutusDataEnum::Integer(_) => PlutusDataKind::Integer,
            PlutusDataEnum::Bytes(_) => PlutusDataKind::Bytes,
        }
    }

    pub fn as_constr_plutus_data(&self) -> Option<ConstrPlutusData> {
        match &self.datum {
            PlutusDataEnum::ConstrPlutusData(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<PlutusMap> {
        match &self.datum {
            PlutusDataEnum::Map(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<PlutusList> {
        match &self.datum {
            PlutusDataEnum::List(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<BigInt> {
        match &self.datum {
            PlutusDataEnum::Integer(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.datum {
            PlutusDataEnum::Bytes(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn to_json(&self, schema: PlutusDatumSchema) -> Result<String, JsError> {
        decode_plutus_datum_to_json_str(self, schema)
    }

    pub fn from_json(json: &str, schema: PlutusDatumSchema) -> Result<PlutusData, JsError> {
        encode_json_str_to_plutus_datum(json, schema)
    }

    pub fn from_address(address: &Address) -> Result<PlutusData, JsError> {
        let payment_cred = match &address.0 {
            AddrType::Base(addr) => Ok(addr.payment_cred()),
            AddrType::Enterprise(addr) => Ok(addr.payment_cred()),
            AddrType::Ptr(addr) => Ok(addr.payment_cred()),
            AddrType::Reward(addr) => Ok(addr.payment_cred()),
            AddrType::Byron(_) => Err(JsError::from_str(
                "Cannot convert Byron address to PlutusData",
            )),
        }?;

        let staking_data = match &address.0 {
            AddrType::Base(addr) => {
                let staking_bytes_data = PlutusData::from_stake_credential(&addr.stake_cred())?;
                Some(PlutusData::new_single_value_constr_plutus_data(
                    &BigNum::from(0u32),
                    &staking_bytes_data,
                ))
            }
            _ => None,
        };

        let pointer_data = match &address.0 {
            AddrType::Ptr(addr) => Some(PlutusData::from_pointer(&addr.stake_pointer())?),
            _ => None,
        };

        let payment_data = PlutusData::from_stake_credential(&payment_cred)?;
        let staking_optional_data = match (staking_data, pointer_data) {
            (Some(_), Some(_)) => Err(JsError::from_str(
                "Address can't have both staking and pointer data",
            )),
            (Some(staking_data), None) => Ok(Some(staking_data)),
            (None, Some(pointer_data)) => Ok(Some(pointer_data)),
            (None, None) => Ok(None),
        }?;

        let mut data_list = PlutusList::new();
        data_list.add(&payment_data);
        if let Some(staking_optional_data) = staking_optional_data {
            data_list.add(&PlutusData::new_single_value_constr_plutus_data(
                &BigNum::from(0u32),
                &staking_optional_data,
            ));
        } else {
            data_list.add(&PlutusData::new_empty_constr_plutus_data(&BigNum::from(
                1u32,
            )));
        }

        Ok(PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(
            &BigNum::from(0u32),
            &data_list,
        )))
    }

    fn from_stake_credential(stake_credential: &Credential) -> Result<PlutusData, JsError> {
        let (bytes_plutus_data, index) = match &stake_credential.0 {
            CredType::Key(key_hash) => (
                PlutusData::new_bytes(key_hash.to_bytes().to_vec()),
                BigNum::from(0u32),
            ),
            CredType::Script(script_hash) => (
                PlutusData::new_bytes(script_hash.to_bytes().to_vec()),
                BigNum::from(1u32),
            ),
        };

        Ok(PlutusData::new_single_value_constr_plutus_data(
            &index,
            &bytes_plutus_data,
        ))
    }

    fn from_pointer(pointer: &Pointer) -> Result<PlutusData, JsError> {
        let mut data_list = PlutusList::new();
        data_list.add(&PlutusData::new_integer(&pointer.slot_bignum().into()));
        data_list.add(&PlutusData::new_integer(&pointer.tx_index_bignum().into()));
        data_list.add(&PlutusData::new_integer(
            &pointer.cert_index_bignum().into(),
        ));

        Ok(PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(
            &BigNum::from(1u32),
            &data_list,
        )))
    }
}

//TODO: replace this by cardano-node schemas
impl JsonSchema for PlutusData {
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }

    fn schema_name() -> String {
        String::from("PlutusData")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

//TODO: need to figure out what schema to use here
impl serde::Serialize for PlutusData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json = decode_plutus_datum_to_json_str(self, PlutusDatumSchema::DetailedSchema)
            .map_err(|ser_err| {
                serde::ser::Error::custom(&format!("Serialization error: {:?}", ser_err))
            })?;
        serializer.serialize_str(&json)
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let datum_json = <String as serde::Deserialize>::deserialize(deserializer)?;
        encode_json_str_to_plutus_datum(&datum_json, PlutusDatumSchema::DetailedSchema).map_err(
            |ser_err| serde::de::Error::custom(&format!("Deserialization error: {:?}", ser_err)),
        )
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct PlutusList {
    pub(crate) elems: Vec<PlutusData>,
    // We should always preserve the original datums when deserialized as this is NOT canonicized
    // before computing datum hashes. This field will default to cardano-cli behavior if None
    // and will re-use the provided one if deserialized, unless the list is modified.
    pub(crate) definite_encoding: Option<bool>,
}

impl<'a> IntoIterator for &'a PlutusList {
    type Item = &'a PlutusData;
    type IntoIter = std::slice::Iter<'a, PlutusData>;

    fn into_iter(self) -> std::slice::Iter<'a, PlutusData> {
        self.elems.iter()
    }
}

impl std::cmp::PartialEq<Self> for PlutusList {
    fn eq(&self, other: &Self) -> bool {
        self.elems.eq(&other.elems)
    }
}

impl std::cmp::Eq for PlutusList {}

to_from_bytes!(PlutusList);

#[wasm_bindgen]
impl PlutusList {
    pub fn new() -> Self {
        Self {
            elems: Vec::new(),
            definite_encoding: None,
        }
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }

    pub fn get(&self, index: usize) -> PlutusData {
        self.elems[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusData) {
        self.elems.push(elem.clone());
        self.definite_encoding = None;
    }
}

impl From<Vec<PlutusData>> for PlutusList {
    fn from(elems: Vec<PlutusData>) -> Self {
        Self {
            elems,
            definite_encoding: None,
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Redeemer {
    pub(crate) tag: RedeemerTag,
    pub(crate) index: BigNum,
    pub(crate) data: PlutusData,
    pub(crate) ex_units: ExUnits,
}

impl_to_from!(Redeemer);

#[wasm_bindgen]
impl Redeemer {
    pub fn tag(&self) -> RedeemerTag {
        self.tag.clone()
    }

    pub fn index(&self) -> BigNum {
        self.index.clone()
    }

    pub fn data(&self) -> PlutusData {
        self.data.clone()
    }

    pub fn ex_units(&self) -> ExUnits {
        self.ex_units.clone()
    }

    pub fn new(tag: &RedeemerTag, index: &BigNum, data: &PlutusData, ex_units: &ExUnits) -> Self {
        Self {
            tag: tag.clone(),
            index: index.clone(),
            data: data.clone(),
            ex_units: ex_units.clone(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn clone_with_index(&self, index: &BigNum) -> Self {
        Self {
            tag: self.tag.clone(),
            index: index.clone(),
            data: self.data.clone(),
            ex_units: self.ex_units.clone(),
        }
    }

    pub(crate) fn clone_with_index_and_tag(&self, index: &BigNum, tag: &RedeemerTag) -> Self {
        Self {
            tag: tag.clone(),
            index: index.clone(),
            data: self.data.clone(),
            ex_units: self.ex_units.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub enum RedeemerTagKind {
    Spend,
    Mint,
    Cert,
    Reward,
    Vote,
    VotingProposal,
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct RedeemerTag(pub(crate) RedeemerTagKind);

impl_to_from!(RedeemerTag);

#[wasm_bindgen]
impl RedeemerTag {
    pub fn new_spend() -> Self {
        Self(RedeemerTagKind::Spend)
    }

    pub fn new_mint() -> Self {
        Self(RedeemerTagKind::Mint)
    }

    pub fn new_cert() -> Self {
        Self(RedeemerTagKind::Cert)
    }

    pub fn new_reward() -> Self {
        Self(RedeemerTagKind::Reward)
    }

    pub fn new_vote() -> Self {
        Self(RedeemerTagKind::Vote)
    }

    pub fn new_voting_proposal() -> Self {
        Self(RedeemerTagKind::VotingProposal)
    }

    pub fn kind(&self) -> RedeemerTagKind {
        self.0
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Redeemers(pub(crate) Vec<Redeemer>);

impl_to_from!(Redeemers);

#[wasm_bindgen]
impl Redeemers {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Redeemer {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Redeemer) {
        self.0.push(elem.clone());
    }

    pub fn total_ex_units(&self) -> Result<ExUnits, JsError> {
        let mut tot_mem = BigNum::zero();
        let mut tot_steps = BigNum::zero();
        for i in 0..self.0.len() {
            let r: &Redeemer = &self.0[i];
            tot_mem = tot_mem.checked_add(&r.ex_units().mem())?;
            tot_steps = tot_steps.checked_add(&r.ex_units().steps())?;
        }
        Ok(ExUnits::new(&tot_mem, &tot_steps))
    }
}

impl From<Vec<Redeemer>> for Redeemers {
    fn from(values: Vec<Redeemer>) -> Self {
        Self(values)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Strings(pub(crate) Vec<String>);

#[wasm_bindgen]
impl Strings {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> String {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: String) {
        self.0.push(elem);
    }
}

// json

/// JSON <-> PlutusData conversion schemas.
/// Follows ScriptDataJsonSchema in cardano-cli defined at:
/// https://github.com/input-output-hk/cardano-node/blob/master/cardano-api/src/Cardano/Api/ScriptData.hs#L254
///
/// All methods here have the following restrictions due to limitations on dependencies:
/// * JSON numbers above u64::MAX (positive) or below i64::MIN (negative) will throw errors
/// * Hex strings for bytes don't accept odd-length (half-byte) strings.
///      cardano-cli seems to support these however but it seems to be different than just 0-padding
///      on either side when tested so proceed with caution
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlutusDatumSchema {
    /// ScriptDataJsonNoSchema in cardano-node.
    ///
    /// This is the format used by --script-data-value in cardano-cli
    /// This tries to accept most JSON but does not support the full spectrum of Plutus datums.
    /// From JSON:
    /// * null/true/false/floats NOT supported
    /// * strings starting with 0x are treated as hex bytes. All other strings are encoded as their utf8 bytes.
    /// To JSON:
    /// * ConstrPlutusData not supported in ANY FORM (neither keys nor values)
    /// * Lists not supported in keys
    /// * Maps not supported in keys
    ////
    BasicConversions,
    /// ScriptDataJsonDetailedSchema in cardano-node.
    ///
    /// This is the format used by --script-data-file in cardano-cli
    /// This covers almost all (only minor exceptions) Plutus datums, but the JSON must conform to a strict schema.
    /// The schema specifies that ALL keys and ALL values must be contained in a JSON map with 2 cases:
    /// 1. For ConstrPlutusData there must be two fields "constructor" contianing a number and "fields" containing its fields
    ///    e.g. { "constructor": 2, "fields": [{"int": 2}, {"list": [{"bytes": "CAFEF00D"}]}]}
    /// 2. For all other cases there must be only one field named "int", "bytes", "list" or "map"
    ///    Integer's value is a JSON number e.g. {"int": 100}
    ///    Bytes' value is a hex string representing the bytes WITHOUT any prefix e.g. {"bytes": "CAFEF00D"}
    ///    Lists' value is a JSON list of its elements encoded via the same schema e.g. {"list": [{"bytes": "CAFEF00D"}]}
    ///    Maps' value is a JSON list of objects, one for each key-value pair in the map, with keys "k" and "v"
    ///          respectively with their values being the plutus datum encoded via this same schema
    ///          e.g. {"map": [
    ///              {"k": {"int": 2}, "v": {"int": 5}},
    ///              {"k": {"map": [{"k": {"list": [{"int": 1}]}, "v": {"bytes": "FF03"}}]}, "v": {"list": []}}
    ///          ]}
    /// From JSON:
    /// * null/true/false/floats NOT supported
    /// * the JSON must conform to a very specific schema
    /// To JSON:
    /// * all Plutus datums should be fully supported outside of the integer range limitations outlined above.
    ////
    DetailedSchema,
}

#[wasm_bindgen]
pub fn encode_json_str_to_plutus_datum(
    json: &str,
    schema: PlutusDatumSchema,
) -> Result<PlutusData, JsError> {
    let value = serde_json::from_str(json).map_err(|e| JsError::from_str(&e.to_string()))?;
    encode_json_value_to_plutus_datum(value, schema)
}

pub fn encode_json_value_to_plutus_datum(
    value: serde_json::Value,
    schema: PlutusDatumSchema,
) -> Result<PlutusData, JsError> {
    use serde_json::Value;
    fn encode_number(x: serde_json::Number) -> Result<PlutusData, JsError> {
        if let Some(x) = x.as_u64() {
            Ok(PlutusData::new_integer(&BigInt::from(x)))
        } else if let Some(x) = x.as_i64() {
            Ok(PlutusData::new_integer(&BigInt::from(x)))
        } else {
            Err(JsError::from_str("floats not allowed in plutus datums"))
        }
    }
    fn encode_string(
        s: &str,
        schema: PlutusDatumSchema,
        is_key: bool,
    ) -> Result<PlutusData, JsError> {
        if schema == PlutusDatumSchema::BasicConversions {
            if s.starts_with("0x") {
                // this must be a valid hex bytestring after
                hex::decode(&s[2..])
                    .map(|bytes| PlutusData::new_bytes(bytes))
                    .map_err(|err| JsError::from_str(&format!("Error decoding {}: {}", s, err)))
            } else if is_key {
                // try as an integer
                BigInt::from_str(s)
                    .map(|x| PlutusData::new_integer(&x))
                    // if not, we use the utf8 bytes of the string instead directly
                    .or_else(|_err| Ok(PlutusData::new_bytes(s.as_bytes().to_vec())))
            } else {
                // can only be UTF bytes if not in a key and not prefixed by 0x
                Ok(PlutusData::new_bytes(s.as_bytes().to_vec()))
            }
        } else {
            if s.starts_with("0x") {
                Err(JsError::from_str("Hex byte strings in detailed schema should NOT start with 0x and should just contain the hex characters"))
            } else {
                hex::decode(s)
                    .map(|bytes| PlutusData::new_bytes(bytes))
                    .map_err(|e| JsError::from_str(&e.to_string()))
            }
        }
    }
    fn encode_array(
        json_arr: Vec<Value>,
        schema: PlutusDatumSchema,
    ) -> Result<PlutusData, JsError> {
        let mut arr = PlutusList::new();
        for value in json_arr {
            arr.add(&encode_json_value_to_plutus_datum(value, schema)?);
        }
        Ok(PlutusData::new_list(&arr))
    }
    match schema {
        PlutusDatumSchema::BasicConversions => match value {
            Value::Null => Err(JsError::from_str("null not allowed in plutus datums")),
            Value::Bool(_) => Err(JsError::from_str("bools not allowed in plutus datums")),
            Value::Number(x) => encode_number(x),
            // no strings in plutus so it's all bytes (as hex or utf8 printable)
            Value::String(s) => encode_string(&s, schema, false),
            Value::Array(json_arr) => encode_array(json_arr, schema),
            Value::Object(json_obj) => {
                let mut map = PlutusMap::new();
                for (raw_key, raw_value) in json_obj {
                    let key = encode_string(&raw_key, schema, true)?;
                    let value = encode_json_value_to_plutus_datum(raw_value, schema)?;
                    map.insert(&key, &value);
                }
                Ok(PlutusData::new_map(&map))
            }
        },
        PlutusDatumSchema::DetailedSchema => match value {
            Value::Object(obj) => {
                if obj.len() == 1 {
                    // all variants except tagged constructors
                    let (k, v) = obj.into_iter().next().unwrap();
                    fn tag_mismatch() -> JsError {
                        JsError::from_str("key does not match type")
                    }
                    match k.as_str() {
                        "int" => match v {
                            Value::Number(x) => encode_number(x),
                            _ => Err(tag_mismatch()),
                        },
                        "bytes" => {
                            encode_string(v.as_str().ok_or_else(tag_mismatch)?, schema, false)
                        }
                        "list" => {
                            encode_array(v.as_array().ok_or_else(tag_mismatch)?.clone(), schema)
                        }
                        "map" => {
                            let mut map = PlutusMap::new();
                            fn map_entry_err() -> JsError {
                                JsError::from_str("entry format in detailed schema map object not correct. Needs to be of form {\"k\": {\"key_type\": key}, \"v\": {\"value_type\", value}}")
                            }
                            for entry in v.as_array().ok_or_else(tag_mismatch)? {
                                let entry_obj = entry.as_object().ok_or_else(map_entry_err)?;
                                let raw_key = entry_obj.get("k").ok_or_else(map_entry_err)?;
                                let value = entry_obj.get("v").ok_or_else(map_entry_err)?;
                                let key =
                                    encode_json_value_to_plutus_datum(raw_key.clone(), schema)?;
                                map.insert(
                                    &key,
                                    &encode_json_value_to_plutus_datum(value.clone(), schema)?,
                                );
                            }
                            Ok(PlutusData::new_map(&map))
                        }
                        invalid_key => Err(JsError::from_str(&format!(
                            "key '{}' in tagged object not valid",
                            invalid_key
                        ))),
                    }
                } else {
                    // constructor with tagged variant
                    if obj.len() != 2 {
                        return Err(JsError::from_str("detailed schemas must either have only one of the following keys: \"int\", \"bytes\", \"list\" or \"map\", or both of these 2 keys: \"constructor\" + \"fields\""));
                    }
                    let variant: BigNum = obj
                        .get("constructor")
                        .and_then(|v| Some(to_bignum(v.as_u64()?)))
                        .ok_or_else(|| JsError::from_str("tagged constructors must contain an unsigned integer called \"constructor\""))?;
                    let fields_json =
                        obj.get("fields")
                            .and_then(|f| f.as_array())
                            .ok_or_else(|| {
                                JsError::from_str(
                                    "tagged constructors must contian a list called \"fields\"",
                                )
                            })?;
                    let mut fields = PlutusList::new();
                    for field_json in fields_json {
                        let field = encode_json_value_to_plutus_datum(field_json.clone(), schema)?;
                        fields.add(&field);
                    }
                    Ok(PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(
                        &variant, &fields,
                    )))
                }
            }
            _ => Err(JsError::from_str(&format!(
                "DetailedSchema requires ALL JSON to be tagged objects, found: {}",
                value
            ))),
        },
    }
}

//TODO: move it to serialize impl
#[wasm_bindgen]
pub fn decode_plutus_datum_to_json_str(
    datum: &PlutusData,
    schema: PlutusDatumSchema,
) -> Result<String, JsError> {
    let value = decode_plutus_datum_to_json_value(datum, schema)?;
    serde_json::to_string(&value).map_err(|e| JsError::from_str(&e.to_string()))
}

//TODO: move it to deserialize impl
pub fn decode_plutus_datum_to_json_value(
    datum: &PlutusData,
    schema: PlutusDatumSchema,
) -> Result<serde_json::Value, JsError> {
    use serde_json::Value;
    let (type_tag, json_value) = match &datum.datum {
        PlutusDataEnum::ConstrPlutusData(constr) => {
            let mut obj = serde_json::map::Map::with_capacity(2);
            obj.insert(
                String::from("constructor"),
                Value::from(from_bignum(&constr.alternative))
            );
            let mut fields = Vec::new();
            for field in constr.data.elems.iter() {
                fields.push(decode_plutus_datum_to_json_value(field, schema)?);
            }
            obj.insert(
                String::from("fields"),
                Value::from(fields)
            );
            (None, Value::from(obj))
        },
        PlutusDataEnum::Map(map) => match schema {
            PlutusDatumSchema::BasicConversions => (None, Value::from(map.0.iter().map(|(key, value)| {
                let json_key: String = match &key.datum {
                    PlutusDataEnum::ConstrPlutusData(_) => Err(JsError::from_str("plutus data constructors are not allowed as keys in this schema. Use DetailedSchema.")),
                    PlutusDataEnum::Map(_) => Err(JsError::from_str("plutus maps are not allowed as keys in this schema. Use DetailedSchema.")),
                    PlutusDataEnum::List(_) => Err(JsError::from_str("plutus lists are not allowed as keys in this schema. Use DetailedSchema.")),
                    PlutusDataEnum::Integer(x) => Ok(x.to_str()),
                    PlutusDataEnum::Bytes(bytes) => String::from_utf8(bytes.clone()).or_else(|_err| Ok(format!("0x{}", hex::encode(bytes))))
                }?;
                let json_value = decode_plutus_datum_to_json_value(value, schema)?;
                Ok((json_key, Value::from(json_value)))
            }).collect::<Result<serde_json::map::Map<String, Value>, JsError>>()?)),
            PlutusDatumSchema::DetailedSchema => (Some("map"), Value::from(map.0.iter().map(|(key, value)| {
                let k = decode_plutus_datum_to_json_value(key, schema)?;
                let v = decode_plutus_datum_to_json_value(value, schema)?;
                let mut kv_obj = serde_json::map::Map::with_capacity(2);
                kv_obj.insert(String::from("k"), k);
                kv_obj.insert(String::from("v"), v);
                Ok(Value::from(kv_obj))
            }).collect::<Result<Vec<_>, JsError>>()?)),
        },
        PlutusDataEnum::List(list) => {
            let mut elems = Vec::new();
            for elem in list.elems.iter() {
                elems.push(decode_plutus_datum_to_json_value(elem, schema)?);
            }
            (Some("list"), Value::from(elems))
        },
        PlutusDataEnum::Integer(bigint) => (
            Some("int"),
            bigint
                .as_int()
                .as_ref()
                .map(|int| if int.0 >= 0 { Value::from(int.0 as u64) } else { Value::from(int.0 as i64) })
                .ok_or_else(|| JsError::from_str(&format!("Integer {} too big for our JSON support", bigint.to_str())))?
        ),
        PlutusDataEnum::Bytes(bytes) => (Some("bytes"), Value::from(match schema {
            PlutusDatumSchema::BasicConversions => {
                // cardano-cli converts to a string only if bytes are utf8 and all characters are printable
                String::from_utf8(bytes.clone())
                    .ok()
                    .filter(|utf8| utf8.chars().all(|c| !c.is_control()))
                // otherwise we hex-encode the bytes with a 0x prefix
                    .unwrap_or_else(|| format!("0x{}", hex::encode(bytes)))
            },
            PlutusDatumSchema::DetailedSchema => hex::encode(bytes),
        })),
    };
    if type_tag.is_none() || schema != PlutusDatumSchema::DetailedSchema {
        Ok(json_value)
    } else {
        let mut wrapper = serde_json::map::Map::with_capacity(1);
        wrapper.insert(String::from(type_tag.unwrap()), json_value);
        Ok(Value::from(wrapper))
    }
}