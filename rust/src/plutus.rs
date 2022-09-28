use super::*;
use std::io::{BufRead, Seek, Write};

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
    bytes: Vec<u8>,
    language: LanguageKind,
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
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ConstrPlutusData {
    alternative: BigNum,
    data: PlutusList,
}

impl_to_from!(ConstrPlutusData);

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
    const GENERAL_FORM_TAG: u64 = 102;

    // None -> needs general tag serialization, not compact
    fn alternative_to_compact_cbor_tag(alt: u64) -> Option<u64> {
        if alt <= 6 {
            Some(121 + alt)
        } else if alt >= 7 && alt <= 127 {
            Some(1280 - 7 + alt)
        } else {
            None
        }
    }

    // None -> General tag(=102) OR Invalid CBOR tag for this scheme
    fn compact_cbor_tag_to_alternative(cbor_tag: u64) -> Option<u64> {
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
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct CostModel(Vec<Int>);

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
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Costmdls(std::collections::BTreeMap<Language, CostModel>);

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
                Some(costmodel) => { result.insert(&lang, &costmodel); },
                _ => {}
            }
        }
        result
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ExUnitPrices {
    mem_price: SubCoin,
    step_price: SubCoin,
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
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct ExUnits {
    mem: BigNum,
    steps: BigNum,
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
}

impl LanguageKind {
    fn from_u64(x: u64) -> Option<LanguageKind> {
        match x {
            0 => Some(LanguageKind::PlutusV1),
            1 => Some(LanguageKind::PlutusV2),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Language(LanguageKind);

impl_to_from!(Language);

#[wasm_bindgen]
impl Language {
    pub fn new_plutus_v1() -> Self {
        Self(LanguageKind::PlutusV1)
    }

    pub fn new_plutus_v2() -> Self {
        Self(LanguageKind::PlutusV2)
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

    pub(crate) fn list() -> Languages {
        Languages(vec![Language::new_plutus_v1(), Language::new_plutus_v2()])
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct PlutusMap(std::collections::BTreeMap<PlutusData, PlutusData>);

impl_to_from!(PlutusMap);

#[wasm_bindgen]
impl PlutusMap {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
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

#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub enum PlutusDataEnum {
    ConstrPlutusData(ConstrPlutusData),
    Map(PlutusMap),
    List(PlutusList),
    Integer(BigInt),
    Bytes(Vec<u8>),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Ord, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct PlutusData {
    datum: PlutusDataEnum,
    // We should always preserve the original datums when deserialized as this is NOT canonicized
    // before computing datum hashes. So this field stores the original bytes to re-use.
    original_bytes: Option<Vec<u8>>,
}

impl std::cmp::PartialEq<Self> for PlutusData {
    fn eq(&self, other: &Self) -> bool {
        self.datum.eq(&other.datum)
    }
}

impl std::cmp::Eq for PlutusData {}

impl_to_from!(PlutusData);

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
}

#[wasm_bindgen]
#[derive(Clone, Debug, Ord, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct PlutusList {
    elems: Vec<PlutusData>,
    // We should always preserve the original datums when deserialized as this is NOT canonicized
    // before computing datum hashes. This field will default to cardano-cli behavior if None
    // and will re-use the provided one if deserialized, unless the list is modified.
    pub(crate) definite_encoding: Option<bool>,
}

impl std::cmp::PartialEq<Self> for PlutusList {
    fn eq(&self, other: &Self) -> bool {
        self.elems.eq(&other.elems)
    }
}

impl std::cmp::Eq for PlutusList {}

impl_to_from!(PlutusList);

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
    tag: RedeemerTag,
    index: BigNum,
    data: PlutusData,
    ex_units: ExUnits,
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

    pub(crate) fn clone_with_index(&self, index: &BigNum) -> Self {
        Self {
            tag: self.tag.clone(),
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
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct RedeemerTag(RedeemerTagKind);

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
pub struct Strings(Vec<String>);

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

#[wasm_bindgen]
pub fn decode_plutus_datum_to_json_str(
    datum: &PlutusData,
    schema: PlutusDatumSchema,
) -> Result<String, JsError> {
    let value = decode_plutus_datum_to_json_value(datum, schema)?;
    serde_json::to_string(&value).map_err(|e| JsError::from_str(&e.to_string()))
}

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

// Serialization

use std::io::SeekFrom;

impl cbor_event::se::Serialize for PlutusScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.bytes)
    }
}

impl Deserialize for PlutusScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self::new(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for PlutusScripts {
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

impl Deserialize for PlutusScripts {
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
                arr.push(PlutusScript::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("PlutusScripts"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for ConstrPlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if let Some(compact_tag) =
            Self::alternative_to_compact_cbor_tag(from_bignum(&self.alternative))
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
                        (to_bignum(alternative), PlutusList::deserialize(raw)?)
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

impl cbor_event::se::Serialize for CostModel {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for cost in &self.0 {
            cost.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for CostModel {
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
                arr.push(Int::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("CostModel"))?;
        Ok(Self(arr.try_into().unwrap()))
    }
}

impl cbor_event::se::Serialize for Costmdls {
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

impl Deserialize for Costmdls {
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
                let key = Language::deserialize(raw)?;
                let value = CostModel::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Costmdls"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for ExUnitPrices {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.mem_price.serialize(serializer)?;
        self.step_price.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ExUnitPrices {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let mem_price =
                (|| -> Result<_, DeserializeError> { Ok(SubCoin::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("mem_price"))?;
            let step_price =
                (|| -> Result<_, DeserializeError> { Ok(SubCoin::deserialize(raw)?) })()
                    .map_err(|e| e.annotate("step_price"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ExUnitPrices {
                mem_price,
                step_price,
            })
        })()
        .map_err(|e| e.annotate("ExUnitPrices"))
    }
}

impl cbor_event::se::Serialize for ExUnits {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.mem.serialize(serializer)?;
        self.steps.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ExUnits {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let mem = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
                .map_err(|e| e.annotate("mem"))?;
            let steps = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
                .map_err(|e| e.annotate("steps"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ExUnits { mem, steps })
        })()
        .map_err(|e| e.annotate("ExUnits"))
    }
}

impl cbor_event::se::Serialize for Language {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        // https://github.com/input-output-hk/cardano-ledger/blob/master/eras/babbage/test-suite/cddl-files/babbage.cddl#L324-L327
        serializer.write_unsigned_integer(self.kind() as u64)
    }
}

impl Deserialize for Language {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match LanguageKind::from_u64(raw.unsigned_integer()?) {
                Some(kind) => Ok(Language(kind)),
                _ => Err(DeserializeError::new(
                    "Language",
                    DeserializeFailure::NoVariantMatched.into(),
                )),
            }
        })()
        .map_err(|e| e.annotate("Language"))
    }
}

impl cbor_event::se::Serialize for Languages {
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

impl Deserialize for Languages {
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
                arr.push(Language::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Languages"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for PlutusMap {
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

impl Deserialize for PlutusMap {
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
                let key = PlutusData::deserialize(raw)?;
                let value = PlutusData::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("PlutusMap"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for PlutusDataEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            PlutusDataEnum::ConstrPlutusData(x) => x.serialize(serializer),
            PlutusDataEnum::Map(x) => x.serialize(serializer),
            PlutusDataEnum::List(x) => x.serialize(serializer),
            PlutusDataEnum::Integer(x) => x.serialize(serializer),
            PlutusDataEnum::Bytes(x) => write_bounded_bytes(serializer, &x),
        }
    }
}

impl Deserialize for PlutusDataEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ConstrPlutusData::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::ConstrPlutusData(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PlutusMap::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Map(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PlutusList::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::List(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(BigInt::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Integer(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(read_bounded_bytes(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Bytes(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            Err(DeserializeError::new(
                "PlutusDataEnum",
                DeserializeFailure::NoVariantMatched.into(),
            ))
        })()
        .map_err(|e| e.annotate("PlutusDataEnum"))
    }
}

impl cbor_event::se::Serialize for PlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match &self.original_bytes {
            Some(bytes) => serializer.write_raw_bytes(bytes),
            None => self.datum.serialize(serializer),
        }
    }
}

impl Deserialize for PlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        // these unwraps are fine since we're seeking the current position
        let before = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        let datum = PlutusDataEnum::deserialize(raw)?;
        let after = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        let bytes_read = (after - before) as usize;
        raw.as_mut_ref().seek(SeekFrom::Start(before)).unwrap();
        // these unwraps are fine since we read the above already
        let original_bytes = raw.as_mut_ref().fill_buf().unwrap()[..bytes_read].to_vec();
        raw.as_mut_ref().consume(bytes_read);
        Ok(Self {
            datum,
            original_bytes: Some(original_bytes),
        })
    }
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

impl Deserialize for PlutusList {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        let len = (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(PlutusData::deserialize(raw)?);
            }
            Ok(len)
        })()
        .map_err(|e| e.annotate("PlutusList"))?;
        Ok(Self {
            elems: arr,
            definite_encoding: Some(len != cbor_event::Len::Indefinite),
        })
    }
}

impl cbor_event::se::Serialize for Redeemer {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.tag.serialize(serializer)?;
        self.index.serialize(serializer)?;
        self.data.serialize(serializer)?;
        self.ex_units.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Redeemer {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let tag = (|| -> Result<_, DeserializeError> { Ok(RedeemerTag::deserialize(raw)?) })()
                .map_err(|e| e.annotate("tag"))?;
            let index = (|| -> Result<_, DeserializeError> { Ok(BigNum::deserialize(raw)?) })()
                .map_err(|e| e.annotate("index"))?;
            let data = (|| -> Result<_, DeserializeError> { Ok(PlutusData::deserialize(raw)?) })()
                .map_err(|e| e.annotate("data"))?;
            let ex_units = (|| -> Result<_, DeserializeError> { Ok(ExUnits::deserialize(raw)?) })()
                .map_err(|e| e.annotate("ex_units"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Redeemer {
                tag,
                index,
                data,
                ex_units,
            })
        })()
        .map_err(|e| e.annotate("Redeemer"))
    }
}

impl cbor_event::se::Serialize for RedeemerTagKind {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            RedeemerTagKind::Spend => serializer.write_unsigned_integer(0u64),
            RedeemerTagKind::Mint => serializer.write_unsigned_integer(1u64),
            RedeemerTagKind::Cert => serializer.write_unsigned_integer(2u64),
            RedeemerTagKind::Reward => serializer.write_unsigned_integer(3u64),
        }
    }
}

impl Deserialize for RedeemerTagKind {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.unsigned_integer() {
                Ok(0) => Ok(RedeemerTagKind::Spend),
                Ok(1) => Ok(RedeemerTagKind::Mint),
                Ok(2) => Ok(RedeemerTagKind::Cert),
                Ok(3) => Ok(RedeemerTagKind::Reward),
                Ok(_) | Err(_) => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
        .map_err(|e| e.annotate("RedeemerTagEnum"))
    }
}

impl cbor_event::se::Serialize for RedeemerTag {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for RedeemerTag {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(RedeemerTagKind::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for Redeemers {
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

impl Deserialize for Redeemers {
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
                arr.push(Redeemer::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Redeemers"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Strings {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            serializer.write_text(&element)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Strings {
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
                arr.push(String::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Strings"))?;
        Ok(Self(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::*;
    use crate::tx_builder_constants::TxBuilderConstants;

    #[test]
    pub fn plutus_constr_data() {
        let constr_0 = PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(
            &to_bignum(0),
            &PlutusList::new(),
        ));
        let constr_0_hash = hex::encode(hash_plutus_data(&constr_0).to_bytes());
        assert_eq!(
            constr_0_hash,
            "923918e403bf43c34b4ef6b48eb2ee04babed17320d8d1b9ff9ad086e86f44ec"
        );
        // let constr_0_roundtrip = PlutusData::from_bytes(constr_0.to_bytes()).unwrap();
        // TODO: do we want semantic equality or bytewise equality?
        // assert_eq!(constr_0, constr_0_roundtrip);
        // let constr_1854 = PlutusData::new_constr_plutus_data(
        //     &ConstrPlutusData::new(&to_bignum(1854), &PlutusList::new())
        // );
        // let constr_1854_roundtrip = PlutusData::from_bytes(constr_1854.to_bytes()).unwrap();
        // assert_eq!(constr_1854, constr_1854_roundtrip);
    }

    #[test]
    pub fn plutus_list_serialization_cli_compatibility() {
        // mimic cardano-cli array encoding, see https://github.com/Emurgo/cardano-serialization-lib/issues/227
        let datum_cli = "d8799f4100d8799fd8799fd8799f581cffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd8799fd8799fd8799f581cffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd87a80ff1a002625a0d8799fd879801a000f4240d87a80ffff";
        let datum = PlutusData::from_bytes(Vec::from_hex(datum_cli).unwrap()).unwrap();
        assert_eq!(datum_cli, hex::encode(datum.to_bytes()));

        // encode empty arrays as fixed
        assert_eq!("80", hex::encode(PlutusList::new().to_bytes()));

        // encode arrays as indefinite length array
        let mut list = PlutusList::new();
        list.add(&PlutusData::new_integer(&BigInt::from_str("1").unwrap()));
        assert_eq!("9f01ff", hex::encode(list.to_bytes()));

        // witness_set should have fixed length array
        let mut witness_set = TransactionWitnessSet::new();
        witness_set.set_plutus_data(&list);
        assert_eq!("a1049f01ff", hex::encode(witness_set.to_bytes()));

        list = PlutusList::new();
        list.add(&datum);
        witness_set.set_plutus_data(&list);
        assert_eq!(
            format!("a1049f{}ff", datum_cli),
            hex::encode(witness_set.to_bytes())
        );
    }

    #[test]
    pub fn plutus_datums_respect_deserialized_encoding() {
        let orig_bytes = Vec::from_hex(
            "81d8799f581ce1cbb80db89e292269aeb93ec15eb963dda5176b66949fe1c2a6a38da140a1401864ff",
        )
        .unwrap();
        let datums = PlutusList::from_bytes(orig_bytes.clone()).unwrap();
        let new_bytes = datums.to_bytes();
        assert_eq!(orig_bytes, new_bytes);
    }

    #[test]
    pub fn plutus_datum_from_json_basic() {
        let json = "{
            \"5\": \"some utf8 string\",
            \"0xDEADBEEF\": [
                {\"reg string\": {}},
                -9
            ]
        }";

        let datum =
            encode_json_str_to_plutus_datum(json, PlutusDatumSchema::BasicConversions).unwrap();

        let map = datum.as_map().unwrap();
        let map_5 = map
            .get(&PlutusData::new_integer(&BigInt::from_str("5").unwrap()))
            .unwrap();
        let utf8_bytes = "some utf8 string".as_bytes();
        assert_eq!(map_5.as_bytes().unwrap(), utf8_bytes);
        let map_deadbeef: PlutusList = map
            .get(&PlutusData::new_bytes(vec![222, 173, 190, 239]))
            .expect("DEADBEEF key not found")
            .as_list()
            .expect("must be a map");
        assert_eq!(map_deadbeef.len(), 2);
        let inner_map = map_deadbeef.get(0).as_map().unwrap();
        assert_eq!(inner_map.len(), 1);
        let reg_string = inner_map
            .get(&PlutusData::new_bytes("reg string".as_bytes().to_vec()))
            .unwrap();
        assert_eq!(reg_string.as_map().expect("reg string: {}").len(), 0);
        assert_eq!(
            map_deadbeef.get(1).as_integer(),
            BigInt::from_str("-9").ok()
        );

        // test round-trip via generated JSON
        let json2 =
            decode_plutus_datum_to_json_str(&datum, PlutusDatumSchema::BasicConversions).unwrap();
        let datum2 =
            encode_json_str_to_plutus_datum(&json2, PlutusDatumSchema::BasicConversions).unwrap();
        assert_eq!(datum, datum2);
    }

    #[test]
    pub fn plutus_datum_from_json_detailed() {
        let json = "{\"list\": [
            {\"map\": [
                {\"k\": {\"bytes\": \"DEADBEEF\"}, \"v\": {\"int\": 42}},
                {\"k\": {\"map\" : [
                    {\"k\": {\"int\": 9}, \"v\": {\"int\": -5}}
                ]}, \"v\": {\"list\": []}}
            ]},
            {\"bytes\": \"CAFED00D\"},
            {\"constructor\": 0, \"fields\": [
                {\"map\": []},
                {\"int\": 23}
            ]}
        ]}";
        let datum =
            encode_json_str_to_plutus_datum(json, PlutusDatumSchema::DetailedSchema).unwrap();

        let list = datum.as_list().unwrap();
        assert_eq!(3, list.len());
        // map
        let map = list.get(0).as_map().unwrap();
        assert_eq!(map.len(), 2);
        let map_deadbeef = map
            .get(&PlutusData::new_bytes(vec![222, 173, 190, 239]))
            .unwrap();
        assert_eq!(map_deadbeef.as_integer(), BigInt::from_str("42").ok());
        let mut long_key = PlutusMap::new();
        long_key.insert(
            &PlutusData::new_integer(&BigInt::from_str("9").unwrap()),
            &PlutusData::new_integer(&BigInt::from_str("-5").unwrap()),
        );
        let map_9_to_5 = map
            .get(&PlutusData::new_map(&long_key))
            .unwrap()
            .as_list()
            .unwrap();
        assert_eq!(map_9_to_5.len(), 0);
        // bytes
        let bytes = list.get(1).as_bytes().unwrap();
        assert_eq!(bytes, [202, 254, 208, 13]);
        // constr data
        let constr = list.get(2).as_constr_plutus_data().unwrap();
        assert_eq!(to_bignum(0), constr.alternative());
        let fields = constr.data();
        assert_eq!(fields.len(), 2);
        let field0 = fields.get(0).as_map().unwrap();
        assert_eq!(field0.len(), 0);
        let field1 = fields.get(1);
        assert_eq!(field1.as_integer(), BigInt::from_str("23").ok());

        // test round-trip via generated JSON
        let json2 =
            decode_plutus_datum_to_json_str(&datum, PlutusDatumSchema::DetailedSchema).unwrap();
        let datum2 =
            encode_json_str_to_plutus_datum(&json2, PlutusDatumSchema::DetailedSchema).unwrap();
        assert_eq!(datum, datum2);
    }

    #[test]
    pub fn test_cost_model() {
        let arr = vec![
            197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32, 2477736, 29175, 4,
            29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 100, 100,
            29773, 100, 150000, 32, 150000, 32, 150000, 32, 150000, 1000, 0, 1, 150000, 32, 150000,
            1000, 0, 8, 148000, 425507, 118, 0, 1, 1, 150000, 1000, 0, 8, 150000, 112536, 247, 1,
            150000, 10000, 1, 136542, 1326, 1, 1000, 150000, 1000, 1, 150000, 32, 150000, 32,
            150000, 32, 1, 1, 150000, 1, 150000, 4, 103599, 248, 1, 103599, 248, 1, 145276, 1366,
            1, 179690, 497, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000,
            32, 148000, 425507, 118, 0, 1, 1, 61516, 11218, 0, 1, 150000, 32, 148000, 425507, 118,
            0, 1, 1, 148000, 425507, 118, 0, 1, 1, 2477736, 29175, 4, 0, 82363, 4, 150000, 5000, 0,
            1, 150000, 32, 197209, 0, 1, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000,
            32, 150000, 32, 150000, 32, 3345831, 1, 1,
        ];
        let cm = arr
            .iter()
            .fold((CostModel::new(), 0), |(mut cm, i), x| {
                cm.set(i, &Int::new_i32(x.clone())).unwrap();
                (cm, i + 1)
            })
            .0;
        let mut cms = Costmdls::new();
        cms.insert(&Language::new_plutus_v1(), &cm);
        assert_eq!(
            hex::encode(cms.language_views_encoding()),
            "a141005901d59f1a000302590001011a00060bc719026d00011a000249f01903e800011a000249f018201a0025cea81971f70419744d186419744d186419744d186419744d186419744d186419744d18641864186419744d18641a000249f018201a000249f018201a000249f018201a000249f01903e800011a000249f018201a000249f01903e800081a000242201a00067e2318760001011a000249f01903e800081a000249f01a0001b79818f7011a000249f0192710011a0002155e19052e011903e81a000249f01903e8011a000249f018201a000249f018201a000249f0182001011a000249f0011a000249f0041a000194af18f8011a000194af18f8011a0002377c190556011a0002bdea1901f1011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000242201a00067e23187600010119f04c192bd200011a000249f018201a000242201a00067e2318760001011a000242201a00067e2318760001011a0025cea81971f704001a000141bb041a000249f019138800011a000249f018201a000302590001011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a00330da70101ff"
        );
    }

    #[test]
    fn test_plutus_script_hash() {
        let hash = EnterpriseAddress::from_address(
            &Address::from_bech32("addr1w896t6qnpsjs32xhw8jl3kw34pqz69kgd72l8hqw83w0k3qahx2sv")
                .unwrap(),
        )
        .unwrap()
        .payment_cred()
        .to_scripthash()
        .unwrap();
        let script = PlutusScript::from_bytes(
            hex::decode("590e6f590e6c0100003323332223322333222332232332233223232333222323332223233333333222222223233322232333322223232332232323332223232332233223232333332222233223322332233223322332222323232232232325335303233300a3333573466e1cd55cea8042400046664446660a40060040026eb4d5d0a8041bae35742a00e66a05046666ae68cdc39aab9d37540029000102b11931a982599ab9c04f04c04a049357426ae89401c8c98d4c124cd5ce0268250240239999ab9a3370ea0089001102b11999ab9a3370ea00a9000102c11931a982519ab9c04e04b0490480473333573466e1cd55cea8012400046601a64646464646464646464646666ae68cdc39aab9d500a480008cccccccccc06ccd40a48c8c8cccd5cd19b8735573aa0049000119810981c9aba15002302e357426ae8940088c98d4c164cd5ce02e82d02c02b89aab9e5001137540026ae854028cd40a40a8d5d0a804999aa8183ae502f35742a010666aa060eb940bcd5d0a80399a8148211aba15006335029335505304b75a6ae854014c8c8c8cccd5cd19b8735573aa0049000119a8119919191999ab9a3370e6aae7540092000233502b33504175a6ae854008c118d5d09aba25002232635305d3357380c20bc0b80b626aae7940044dd50009aba150023232323333573466e1cd55cea80124000466a05266a082eb4d5d0a80118231aba135744a004464c6a60ba66ae7018417817016c4d55cf280089baa001357426ae8940088c98d4c164cd5ce02e82d02c02b89aab9e5001137540026ae854010cd40a5d71aba15003335029335505375c40026ae854008c0e0d5d09aba2500223263530553357380b20ac0a80a626ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135744a00226aae7940044dd50009aba150023232323333573466e1d4005200623020303a357426aae79400c8cccd5cd19b875002480108c07cc110d5d09aab9e500423333573466e1d400d20022301f302f357426aae7940148cccd5cd19b875004480008c088dd71aba135573ca00c464c6a60a066ae7015014413c13813413012c4d55cea80089baa001357426ae8940088c98d4c124cd5ce026825024023882489931a982419ab9c4910350543500049047135573ca00226ea80044d55ce9baa001135744a00226aae7940044dd50009109198008018011000911111111109199999999980080580500480400380300280200180110009109198008018011000891091980080180109000891091980080180109000891091980080180109000909111180200290911118018029091111801002909111180080290008919118011bac0013200135503c2233335573e0024a01c466a01a60086ae84008c00cd5d100101811919191999ab9a3370e6aae75400d200023330073232323333573466e1cd55cea8012400046601a605c6ae854008cd404c0a8d5d09aba25002232635303433573807006a06606426aae7940044dd50009aba150033335500b75ca0146ae854008cd403dd71aba135744a004464c6a606066ae700d00c40bc0b84d5d1280089aab9e5001137540024442466600200800600440024424660020060044002266aa002eb9d6889119118011bab00132001355036223233335573e0044a012466a01066aa05c600c6aae754008c014d55cf280118021aba200302b1357420022244004244244660020080062400224464646666ae68cdc3a800a400046a05e600a6ae84d55cf280191999ab9a3370ea00490011281791931a981399ab9c02b028026025024135573aa00226ea80048c8c8cccd5cd19b8735573aa004900011980318039aba15002375a6ae84d5d1280111931a981219ab9c028025023022135573ca00226ea80048848cc00400c00880048c8cccd5cd19b8735573aa002900011bae357426aae7940088c98d4c080cd5ce01201080f80f09baa00112232323333573466e1d400520042500723333573466e1d4009200223500a3006357426aae7940108cccd5cd19b87500348000940288c98d4c08ccd5ce01381201101081000f89aab9d50011375400224244460060082244400422444002240024646666ae68cdc3a800a4004400c46666ae68cdc3a80124000400c464c6a603666ae7007c0700680640604d55ce9baa0011220021220012001232323232323333573466e1d4005200c200b23333573466e1d4009200a200d23333573466e1d400d200823300b375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c46601a6eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc048c050d5d0a8049bae357426ae8940248cccd5cd19b875006480088c050c054d5d09aab9e500b23333573466e1d401d2000230133016357426aae7940308c98d4c080cd5ce01201080f80f00e80e00d80d00c80c09aab9d5004135573ca00626aae7940084d55cf280089baa00121222222230070082212222222330060090082122222223005008122222220041222222200322122222223300200900822122222223300100900820012323232323333573466e1d400520022333008375a6ae854010dd69aba15003375a6ae84d5d1280191999ab9a3370ea00490001180518059aba135573ca00c464c6a602266ae7005404804003c0384d55cea80189aba25001135573ca00226ea80048488c00800c888488ccc00401401000c80048c8c8cccd5cd19b875001480088c018dd71aba135573ca00646666ae68cdc3a80124000460106eb8d5d09aab9e5004232635300b33573801e01801401201026aae7540044dd5000909118010019091180080190008891119191999ab9a3370e6aae75400920002335500b300635742a004600a6ae84d5d1280111931a980419ab9c00c009007006135573ca00226ea800526120012001112212330010030021120014910350543100222123330010040030022001121223002003112200112001120012001122002122001200111232300100122330033002002001332323233322233322233223332223322332233322233223322332233223233322232323322323232323333222232332232323222323222325335301a5335301a333573466e1cc8cccd54c05048004c8cd406488ccd406400c004008d4058004cd4060888c00cc008004800488cdc0000a40040029000199aa98068900091299a980e299a9a81a1a98169a98131a9812001110009110019119a98188011281c11a81c8009080f880e899a8148010008800a8141a981028009111111111005240040380362038266ae712413c53686f756c642062652065786163746c79206f6e652073637269707420696e70757420746f2061766f696420646f75626c65207361742069737375650001b15335303500315335301a5335301a333573466e20ccc064ccd54c03448005402540a0cc020d4c0c00188880094004074074cdc09a9818003111001a80200d80e080e099ab9c49010f73656c6c6572206e6f7420706169640001b15335301a333573466e20ccc064cc88ccd54c03c48005402d40a8cc028004009400401c074075401006c07040704cd5ce24810d66656573206e6f7420706169640001b101b15335301a3322353022002222222222253353503e33355301f1200133502322533535040002210031001503f253353027333573466e3c0300040a40a04d41040045410000c840a4409d4004d4c0c001888800840704cd5ce2491c4f6e6c792073656c6c65722063616e2063616e63656c206f666665720001b101b135301d00122002153353016333573466e2540040d406005c40d4540044cdc199b8235302b001222003480c920d00f2235301a0012222222222333553011120012235302a002222353034003223353038002253353026333573466e3c0500040a009c4cd40cc01401c401c801d40b0024488cd54c02c480048d4d5408c00488cd54098008cd54c038480048d4d5409800488cd540a4008ccd4d540340048cc0e12000001223303900200123303800148000004cd54c02c480048d4d5408c00488cd54098008ccd4d540280048cd54c03c480048d4d5409c00488cd540a8008d5404400400488ccd5540200580080048cd54c03c480048d4d5409c00488cd540a8008d5403c004004ccd55400c044008004444888ccd54c018480054080cd54c02c480048d4d5408c00488cd54098008d54034004ccd54c0184800488d4d54090008894cd4c05cccd54c04048004c8cd405488ccd4d402c00c88008008004d4d402400488004cd4024894cd4c064008406c40040608d4d5409c00488cc028008014018400c4cd409001000d4084004cd54c02c480048d4d5408c00488c8cd5409c00cc004014c8004d540d8894cd4d40900044d5403400c884d4d540a4008894cd4c070cc0300080204cd5404801c0044c01800c00848848cc00400c00848004c8004d540b488448894cd4d40780044008884cc014008ccd54c01c480040140100044484888c00c01044884888cc0080140104484888c004010448004c8004d540a08844894cd4d406000454068884cd406cc010008cd54c01848004010004c8004d5409c88448894cd4d40600044d401800c884ccd4024014c010008ccd54c01c4800401401000448d4d400c0048800448d4d40080048800848848cc00400c0084800488ccd5cd19b8f002001006005222323230010053200135502522335350130014800088d4d54060008894cd4c02cccd5cd19b8f00200900d00c13007001130060033200135502422335350120014800088d4d5405c008894cd4c028ccd5cd19b8f00200700c00b10011300600312200212200120014881002212330010030022001222222222212333333333300100b00a009008007006005004003002200122123300100300220012221233300100400300220011122002122122330010040031200111221233001003002112001221233001003002200121223002003212230010032001222123330010040030022001121223002003112200112001122002122001200122337000040029040497a0088919180080091198019801001000a4411c28f07a93d7715db0bdc1766c8bd5b116602b105c02c54fc3bcd0d4680001").unwrap().clone(),
        ).unwrap();
        assert_eq!(script.hash(), hash);
    }

    fn redeemer_with_ex_units(mem: &BigNum, steps: &BigNum) -> Redeemer {
        Redeemer::new(
            &RedeemerTag::new_spend(),
            &BigNum::zero(),
            &PlutusData::new_integer(&BigInt::from_str("0").unwrap()),
            &ExUnits::new(mem, steps),
        )
    }

    #[test]
    fn test_total_ex_units() {
        let mut r = Redeemers::new();

        fn assert_ex_units(eu: &ExUnits, exp_mem: u64, exp_steps: u64) {
            assert_eq!(eu.mem, to_bignum(exp_mem));
            assert_eq!(eu.steps, to_bignum(exp_steps));
        }

        r.add(&redeemer_with_ex_units(&to_bignum(10), &to_bignum(100)));
        assert_ex_units(&r.total_ex_units().unwrap(), 10, 100);
        r.add(&redeemer_with_ex_units(&to_bignum(20), &to_bignum(200)));
        assert_ex_units(&r.total_ex_units().unwrap(), 30, 300);
        r.add(&redeemer_with_ex_units(&to_bignum(30), &to_bignum(300)));
        assert_ex_units(&r.total_ex_units().unwrap(), 60, 600);
    }

    #[test]
    fn test_empty_constr_data() {
        assert_eq!(
            PlutusData::new_empty_constr_plutus_data(&BigNum::one()),
            PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(
                &BigNum::from_str("1").unwrap(),
                &PlutusList::new(),
            ),),
        )
    }

    #[test]
    fn test_plutus_script_version() {
        let bytes = hex::decode("4e4d01000033222220051200120011").unwrap();
        let s1: PlutusScript = PlutusScript::from_bytes(bytes.clone()).unwrap();
        let s2: PlutusScript = PlutusScript::from_bytes_v2(bytes.clone()).unwrap();

        assert_eq!(s1.bytes(), bytes[1..]);
        assert_eq!(s2.bytes(), bytes[1..]);
        assert_eq!(s1.language_version(), Language::new_plutus_v1());
        assert_eq!(s2.language_version(), Language::new_plutus_v2());

        assert_eq!(
            s1,
            PlutusScript::from_bytes_with_version(bytes.clone(), &Language::new_plutus_v1(),)
                .unwrap()
        );
        assert_eq!(
            s2,
            PlutusScript::from_bytes_with_version(bytes.clone(), &Language::new_plutus_v2(),)
                .unwrap()
        );
    }

    #[test]
    fn test_language_roundtrip() {
        fn deserialize_language_from_uint(x: u64) -> Result<Language, DeserializeError> {
            let mut buf = Serializer::new_vec();
            x.serialize(&mut buf).unwrap();
            Language::from_bytes(buf.finalize())
        }

        assert_eq!(
            deserialize_language_from_uint(0).unwrap(),
            Language::new_plutus_v1()
        );
        assert_eq!(
            deserialize_language_from_uint(1).unwrap(),
            Language::new_plutus_v2()
        );
        assert!(deserialize_language_from_uint(2).is_err());

        assert_eq!(
            Language::from_bytes(Language::new_plutus_v1().to_bytes()).unwrap(),
            Language::new_plutus_v1(),
        );
        assert_eq!(
            Language::from_bytes(Language::new_plutus_v2().to_bytes()).unwrap(),
            Language::new_plutus_v2(),
        );
    }

    #[test]
    fn test_cost_model_roundtrip() {
        use crate::tx_builder_constants::TxBuilderConstants;
        let costmodels = TxBuilderConstants::plutus_vasil_cost_models();
        assert_eq!(
            costmodels,
            Costmdls::from_bytes(costmodels.to_bytes()).unwrap()
        );
    }

    #[test]
    fn test_known_plutus_data_hash() {
        use crate::tx_builder_constants::TxBuilderConstants;
        let pdata = PlutusList::from(vec![PlutusData::new_constr_plutus_data(
            &ConstrPlutusData::new(
                &BigNum::zero(),
                &PlutusList::from(vec![
                    PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(
                        &BigNum::zero(),
                        &PlutusList::from(vec![
                            PlutusData::new_bytes(
                                hex::decode(
                                    "A183BF86925F66C579A3745C9517744399679B090927B8F6E2F2E1BB",
                                )
                                .unwrap(),
                            ),
                            PlutusData::new_bytes(
                                hex::decode("6164617065416D616E734576616E73").unwrap(),
                            ),
                        ]),
                    )),
                    PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(
                        &BigNum::zero(),
                        &PlutusList::from(vec![
                            PlutusData::new_bytes(
                                hex::decode(
                                    "9A4E855293A0B9AF5E50935A331D83E7982AB5B738EA0E6FC0F9E656",
                                )
                                .unwrap(),
                            ),
                            PlutusData::new_bytes(
                                hex::decode("4652414D455F38333030325F4C30").unwrap(),
                            ),
                        ]),
                    )),
                    PlutusData::new_bytes(
                        hex::decode("BEA1C521DF58F4EEEF60C647E5EBD88C6039915409F9FD6454A476B9")
                            .unwrap(),
                    ),
                ]),
            ),
        )]);
        let redeemers = Redeemers(vec![Redeemer::new(
            &RedeemerTag::new_spend(),
            &BigNum::one(),
            &PlutusData::new_empty_constr_plutus_data(&BigNum::zero()),
            &ExUnits::new(&to_bignum(7000000), &to_bignum(3000000000)),
        )]);
        let lang = Language::new_plutus_v1();
        let lang_costmodel = TxBuilderConstants::plutus_vasil_cost_models()
            .get(&lang)
            .unwrap();
        let mut retained_cost_models = Costmdls::new();
        retained_cost_models.insert(&lang, &lang_costmodel);
        let hash = hash_script_data(&redeemers, &retained_cost_models, Some(pdata));
        assert_eq!(
            hex::encode(hash.to_bytes()),
            "357041b88b914670a3b5e3b0861d47f2ac05ed4935ea73886434d8944aa6dfe0"
        );
    }

    #[test]
    fn test_same_datum_in_different_formats_with_expected_hashes() {
        // This is a known datum with indefinite arrays and a known expected hash
        let pdata1 = PlutusData::from_bytes(hex::decode("d8799fd8799f581ca183bf86925f66c579a3745c9517744399679b090927b8f6e2f2e1bb4f616461706541696c656e416d61746fffd8799f581c9a4e855293a0b9af5e50935a331d83e7982ab5b738ea0e6fc0f9e6564e4652414d455f36353030335f4c30ff581cbea1c521df58f4eeef60c647e5ebd88c6039915409f9fd6454a476b9ff").unwrap()).unwrap();
        assert_eq!(
            hex::encode(hash_plutus_data(&pdata1).to_bytes()),
            "ec3028f46325b983a470893a8bdc1b4a100695b635fb1237d301c3490b23e89b"
        );
        // This is the same exact datum manually converted to definite arrays
        // and it produces a different known expected hash because the format is preserved after deserialization
        let pdata2 = PlutusData::from_bytes(hex::decode("d87983d87982581ca183bf86925f66c579a3745c9517744399679b090927b8f6e2f2e1bb4f616461706541696c656e416d61746fd87982581c9a4e855293a0b9af5e50935a331d83e7982ab5b738ea0e6fc0f9e6564e4652414d455f36353030335f4c30581cbea1c521df58f4eeef60c647e5ebd88c6039915409f9fd6454a476b9").unwrap()).unwrap();
        assert_eq!(
            hex::encode(hash_plutus_data(&pdata2).to_bytes()),
            "816cdf6d4d8cba3ad0188ca643db95ddf0e03cdfc0e75a9550a72a82cb146222"
        );
    }

    #[test]
    fn test_known_plutus_data_hash_with_no_datums() {
        let mut costmodels = Costmdls::new();
        costmodels.insert(
            &Language::new_plutus_v2(),
            &TxBuilderConstants::plutus_vasil_cost_models().get(&Language::new_plutus_v2()).unwrap(),
        );
        let hash = hash_script_data(
            &Redeemers(vec![
                Redeemer::new(
                    &RedeemerTag::new_spend(),
                    &BigNum::zero(),
                    &PlutusData::new_empty_constr_plutus_data(&BigNum::zero()),
                    &ExUnits::new(&to_bignum(842996), &to_bignum(246100241)),
                ),
            ]),
            &costmodels,
            None,
        );
        assert_eq!(hex::encode(hash.to_bytes()), "ac71f2adcaecd7576fa658098b12001dec03ce5c27dbb890e16966e3e135b3e2");
    }

    #[test]
    fn test_known_plutus_data_hash_2() {
        let datums = PlutusList::from(vec![
            PlutusData::new_constr_plutus_data(
                &ConstrPlutusData::new(
                    &BigNum::zero(),
                    &PlutusList::from(vec![
                        PlutusData::new_bytes(
                            hex::decode("45F6A506A49A38263C4A8BBB2E1E369DD8732FB1F9A281F3E8838387").unwrap(),
                        ),
                        PlutusData::new_integer(&BigInt::from_str("60000000").unwrap()),
                        PlutusData::new_bytes(
                            hex::decode("EE8E37676F6EBB8E031DFF493F88FF711D24AA68666A09D61F1D3FB3").unwrap(),
                        ),
                        PlutusData::new_bytes(
                            hex::decode("43727970746F44696E6F3036333039").unwrap(),
                        ),
                    ]),
                )
            )
        ]);
        let redeemers = Redeemers(vec![
            Redeemer::new(
                &RedeemerTag::new_spend(),
                &BigNum::one(),
                &PlutusData::new_empty_constr_plutus_data(&BigNum::one()),
                &ExUnits::new(&to_bignum(61300), &to_bignum(18221176)),
            ),
        ]);
        let hash = hash_script_data(
            &redeemers,
            &TxBuilderConstants::plutus_vasil_cost_models()
                .retain_language_versions(&Languages(vec![Language::new_plutus_v1()])),
            Some(datums),
        );
        assert_eq!(hex::encode(hash.to_bytes()), "e6129f50a866d19d95bc9c95ee87b57a9e695c05d92ba2746141b03c15cf5f70");
    }
}
