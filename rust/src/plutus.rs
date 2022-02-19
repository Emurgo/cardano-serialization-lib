use std::io::{BufRead, Seek, Write};
use super::*;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}};


#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScript(Vec<u8>);

to_from_bytes!(PlutusScript);

#[wasm_bindgen]
impl PlutusScript {
    pub fn hash(&self, namespace: ScriptHashNamespace) -> ScriptHash {
        hash_script(namespace, self.to_bytes())
    }

    /**
     * Creates a new Plutus script from the RAW bytes of the compiled script.
     * This does NOT include any CBOR encoding around these bytes (e.g. from "cborBytes" in cardano-cli)
     * If you creating this from those you should use PlutusScript::from_bytes() instead.
     */
    pub fn new(bytes: Vec<u8>) -> PlutusScript {
        Self(bytes)
    }

    /**
     * The raw bytes of this compiled Plutus script.
     * If you need "cborBytes" for cardano-cli use PlutusScript::to_bytes() instead.
     */
    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScripts(pub (crate) Vec<PlutusScript>);

to_from_bytes!(PlutusScripts);

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
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConstrPlutusData {
    alternative: BigNum,
    data: PlutusList,
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

const COST_MODEL_OP_COUNT: usize = 166;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CostModel(Vec<Int>);

to_from_bytes!(CostModel);

#[wasm_bindgen]
impl CostModel {
    pub fn new() -> Self {
        let mut costs = Vec::with_capacity(COST_MODEL_OP_COUNT);
        for _ in 0 .. COST_MODEL_OP_COUNT {
            costs.push(Int::new_i32(0));
        }
        Self(costs)
    }

    pub fn set(&mut self, operation: usize, cost: &Int) -> Result<Int, JsError> {
        if operation >= COST_MODEL_OP_COUNT {
            return Err(JsError::from_str(&format!("CostModel operation {} out of bounds. Max is {}", operation, COST_MODEL_OP_COUNT)));
        }
        let old = self.0[operation].clone();
        self.0[operation] = cost.clone();
        Ok(old)
    }

    pub fn get(&self, operation: usize) -> Result<Int, JsError> {
        if operation >= COST_MODEL_OP_COUNT {
            return Err(JsError::from_str(&format!("CostModel operation {} out of bounds. Max is {}", operation, COST_MODEL_OP_COUNT)));
        }
        Ok(self.0[operation].clone())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Costmdls(std::collections::BTreeMap<Language, CostModel>);

to_from_bytes!(Costmdls);

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
        let mut keys_bytes: Vec<(Language, Vec<u8>)> = self.0.iter().map(|(k, _v)| (k.clone(), k.to_bytes())).collect();
        // keys must be in canonical ordering first
        keys_bytes.sort_by(|lhs, rhs| match lhs.1.len().cmp(&rhs.1.len()) {
            std::cmp::Ordering::Equal => lhs.1.cmp(&rhs.1),
            len_order => len_order,
        });
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64)).unwrap();
        for (key, key_bytes) in keys_bytes.iter() {
            serializer.write_bytes(key_bytes).unwrap();
            let cost_model = self.0.get(&key).unwrap();
            // Due to a bug in the cardano-node input-output-hk/cardano-ledger-specs/issues/2512
            // we must use indefinite length serialization in this inner bytestring to match it
            let mut cost_model_serializer = Serializer::new_vec();
            cost_model_serializer.write_array(cbor_event::Len::Indefinite).unwrap();
            for cost in &cost_model.0 {
                cost.serialize(&mut cost_model_serializer).unwrap();
            }
            cost_model_serializer.write_special(cbor_event::Special::Break).unwrap();
            serializer.write_bytes(cost_model_serializer.finalize()).unwrap();
        }
        let out = serializer.finalize();
        println!("language_views = {}", hex::encode(out.clone()));
        out
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExUnitPrices {
    mem_price: SubCoin,
    step_price: SubCoin,
}

to_from_bytes!(ExUnitPrices);

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExUnits {
    // TODO: should these be u32 or BigNum?
    mem: BigNum,
    steps: BigNum,
}

to_from_bytes!(ExUnits);

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
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LanguageKind {
    PlutusV1,
    PlutusV2,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Language(LanguageKind);

to_from_bytes!(Language);

#[wasm_bindgen]
impl Language {
    pub fn new_plutus_v1() -> Self {
        Self(LanguageKind::PlutusV1)
    }

    pub fn new_plutus_v2() -> Self {
        Self(LanguageKind::PlutusV2)
    }

    pub fn kind(&self) -> LanguageKind {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Languages(Vec<Language>);

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
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusMap(std::collections::BTreeMap<PlutusData, PlutusData>);

to_from_bytes!(PlutusMap);

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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlutusDataEnum {
    ConstrPlutusData(ConstrPlutusData),
    Map(PlutusMap),
    List(PlutusList),
    Integer(BigInt),
    Bytes(Vec<u8>),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusData {
    datum: PlutusDataEnum,
    // We should always preserve the original datums when deserialized as this is NOT canonicized
    // before computing datum hashes. So this field stores the original bytes to re-use.
    original_bytes: Option<Vec<u8>>,
}

to_from_bytes!(PlutusData);

#[wasm_bindgen]
impl PlutusData {
    pub fn new_constr_plutus_data(constr_plutus_data: &ConstrPlutusData) -> Self {
        Self {
            datum: PlutusDataEnum::ConstrPlutusData(constr_plutus_data.clone()),
            original_bytes: None,
        }
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusList {
    pub(crate) elems: Vec<PlutusData>,
    // We should always preserve the original datums when deserialized as this is NOT canonicized
    // before computing datum hashes. This field will default to cardano-cli behavior if None
    // and will re-use the provided one if deserialized, unless the list is modified.
    pub(crate) definite_encoding: Option<bool>,
}

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

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Redeemer {
    tag: RedeemerTag,
    index: BigNum,
    data: PlutusData,
    ex_units: ExUnits,
}

to_from_bytes!(Redeemer);

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
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum RedeemerTagKind {
    Spend,
    Mint,
    Cert,
    Reward,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct RedeemerTag(RedeemerTagKind);

to_from_bytes!(RedeemerTag);

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Redeemers(pub (crate) Vec<Redeemer>);

to_from_bytes!(Redeemers);

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










// Serialization

use std::io::{SeekFrom};


impl cbor_event::se::Serialize for PlutusScript {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for PlutusScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for PlutusScripts {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(PlutusScript::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("PlutusScripts"))?;
        Ok(Self(arr))
    }
}


// TODO: write tests for this hand-coded implementation?
impl cbor_event::se::Serialize for ConstrPlutusData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        if let Some(compact_tag) = Self::alternative_to_compact_cbor_tag(from_bignum(&self.alternative)) {
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
                    let data = (|| -> Result<_, DeserializeError> {
                        Ok(PlutusList::deserialize(raw)?)
                    })().map_err(|e| e.annotate("datas"))?;
                    match len {
                        cbor_event::Len::Len(_) => (),
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => (),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    (alternative, data)
                },
                // concise form
                tag => {
                    if let Some(alternative) = Self::compact_cbor_tag_to_alternative(tag) {
                        (to_bignum(alternative), PlutusList::deserialize(raw)?)
                    } else {
                        return Err(DeserializeFailure::TagMismatch{
                            found: tag,
                            expected: Self::GENERAL_FORM_TAG,
                        }.into());
                    }
                },
            };
            Ok(ConstrPlutusData{
                alternative,
                data,
            })
        })().map_err(|e| e.annotate("ConstrPlutusData"))
    }
}

impl cbor_event::se::Serialize for CostModel {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(COST_MODEL_OP_COUNT as u64))?;
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
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Int::deserialize(raw)?);
            }
            if arr.len() != COST_MODEL_OP_COUNT {
                return Err(DeserializeFailure::OutOfRange{
                    min: COST_MODEL_OP_COUNT,
                    max: COST_MODEL_OP_COUNT,
                    found: arr.len()
                }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("CostModel"))?;
        Ok(Self(arr.try_into().unwrap()))
    }
}

impl cbor_event::se::Serialize for Costmdls {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = Language::deserialize(raw)?;
                let value = CostModel::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("Costmdls"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for ExUnitPrices {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            let mem_price = (|| -> Result<_, DeserializeError> {
                Ok(SubCoin::deserialize(raw)?)
            })().map_err(|e| e.annotate("mem_price"))?;
            let step_price = (|| -> Result<_, DeserializeError> {
                Ok(SubCoin::deserialize(raw)?)
            })().map_err(|e| e.annotate("step_price"))?;
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
        })().map_err(|e| e.annotate("ExUnitPrices"))
    }
}

impl cbor_event::se::Serialize for ExUnits {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            let mem = (|| -> Result<_, DeserializeError> {
                Ok(BigNum::deserialize(raw)?)
            })().map_err(|e| e.annotate("mem"))?;
            let steps = (|| -> Result<_, DeserializeError> {
                Ok(BigNum::deserialize(raw)?)
            })().map_err(|e| e.annotate("steps"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ExUnits {
                mem,
                steps,
            })
        })().map_err(|e| e.annotate("ExUnits"))
    }
}

impl cbor_event::se::Serialize for Language {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self.0 {
            LanguageKind::PlutusV1 => {
                serializer.write_unsigned_integer(0u64)
            },
            LanguageKind::PlutusV2 => {
                serializer.write_unsigned_integer(1u64)
            },
        }
    }
}

impl Deserialize for Language {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.unsigned_integer()? {
                0 => Ok(Language::new_plutus_v1()),
                _ => Err(DeserializeError::new("Language", DeserializeFailure::NoVariantMatched.into())),
            }
        })().map_err(|e| e.annotate("Language"))
    }
}

impl cbor_event::se::Serialize for Languages {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Language::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Languages"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for PlutusMap {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = PlutusData::deserialize(raw)?;
                let value = PlutusData::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("PlutusMap"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for PlutusDataEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            PlutusDataEnum::ConstrPlutusData(x) => {
                x.serialize(serializer)
            },
            PlutusDataEnum::Map(x) => {
                x.serialize(serializer)
            },
            PlutusDataEnum::List(x) => {
                x.serialize(serializer)
            },
            PlutusDataEnum::Integer(x) => {
                x.serialize(serializer)
            },
            PlutusDataEnum::Bytes(x) => {
                write_bounded_bytes(serializer, &x)
            },
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
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PlutusMap::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Map(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PlutusList::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::List(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(BigInt::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Integer(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(read_bounded_bytes(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::Bytes(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("PlutusDataEnum", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("PlutusDataEnum"))
    }
}

impl cbor_event::se::Serialize for PlutusData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(PlutusData::deserialize(raw)?);
            }
            Ok(len)
        })().map_err(|e| e.annotate("PlutusList"))?;
        Ok(Self {
            elems: arr,
            definite_encoding: Some(len != cbor_event::Len::Indefinite),
        })
    }
}

impl cbor_event::se::Serialize for Redeemer {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            let tag = (|| -> Result<_, DeserializeError> {
                Ok(RedeemerTag::deserialize(raw)?)
            })().map_err(|e| e.annotate("tag"))?;
            let index = (|| -> Result<_, DeserializeError> {
                Ok(BigNum::deserialize(raw)?)
            })().map_err(|e| e.annotate("index"))?;
            let data = (|| -> Result<_, DeserializeError> {
                Ok(PlutusData::deserialize(raw)?)
            })().map_err(|e| e.annotate("data"))?;
            let ex_units = (|| -> Result<_, DeserializeError> {
                Ok(ExUnits::deserialize(raw)?)
            })().map_err(|e| e.annotate("ex_units"))?;
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
        })().map_err(|e| e.annotate("Redeemer"))
    }
}

impl cbor_event::se::Serialize for RedeemerTagKind {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            RedeemerTagKind::Spend => {
                serializer.write_unsigned_integer(0u64)
            },
            RedeemerTagKind::Mint => {
                serializer.write_unsigned_integer(1u64)
            },
            RedeemerTagKind::Cert => {
                serializer.write_unsigned_integer(2u64)
            },
            RedeemerTagKind::Reward => {
                serializer.write_unsigned_integer(3u64)
            },
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
        })().map_err(|e| e.annotate("RedeemerTagEnum"))
    }
}

impl cbor_event::se::Serialize for RedeemerTag {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for RedeemerTag {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(RedeemerTagKind::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for Redeemers {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(Redeemer::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Redeemers"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Strings {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
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
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(String::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Strings"))?;
        Ok(Self(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::*;

    #[test]
    pub fn plutus_constr_data() {
        let constr_0 = PlutusData::new_constr_plutus_data(
            &ConstrPlutusData::new(&to_bignum(0), &PlutusList::new())
        );
        let constr_0_hash = hex::encode(hash_plutus_data(&constr_0).to_bytes());
        assert_eq!(constr_0_hash, "923918e403bf43c34b4ef6b48eb2ee04babed17320d8d1b9ff9ad086e86f44ec");
        let constr_0_roundtrip = PlutusData::from_bytes(constr_0.to_bytes()).unwrap();
        // TODO: do we want semantic equality or bytewise equality?
        //assert_eq!(constr_0, constr_0_roundtrip);
        let constr_1854 = PlutusData::new_constr_plutus_data(
            &ConstrPlutusData::new(&to_bignum(1854), &PlutusList::new())
        );
        let constr_1854_roundtrip = PlutusData::from_bytes(constr_1854.to_bytes()).unwrap();
        //assert_eq!(constr_1854, constr_1854_roundtrip);
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
        assert_eq!(format!("a1049f{}ff", datum_cli), hex::encode(witness_set.to_bytes()));
    }

    #[test]
    pub fn plutus_datums_respect_deserialized_encoding() {
        let orig_bytes = Vec::from_hex("81d8799f581ce1cbb80db89e292269aeb93ec15eb963dda5176b66949fe1c2a6a38da140a1401864ff").unwrap();
        let datums = PlutusList::from_bytes(orig_bytes.clone()).unwrap();
        let new_bytes = datums.to_bytes();
        assert_eq!(orig_bytes, new_bytes);
    }

    #[test]
    pub fn test_cost_model() {
        let arr = vec![
            197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32,
            2477736, 29175, 4, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773,
            100, 29773, 100, 100, 100, 29773, 100, 150000, 32, 150000, 32, 150000, 32,
            150000, 1000, 0, 1, 150000, 32, 150000, 1000, 0, 8, 148000, 425507, 118,
            0, 1, 1, 150000, 1000, 0, 8, 150000, 112536, 247, 1, 150000, 10000, 1,
            136542, 1326, 1, 1000, 150000, 1000, 1, 150000, 32, 150000, 32, 150000,
            32, 1, 1, 150000, 1, 150000, 4, 103599, 248, 1, 103599, 248, 1, 145276,
            1366, 1, 179690, 497, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32,
            150000, 32, 150000, 32, 148000, 425507, 118, 0, 1, 1, 61516, 11218, 0, 1,
            150000, 32, 148000, 425507, 118, 0, 1, 1, 148000, 425507, 118, 0, 1, 1,
            2477736, 29175, 4, 0, 82363, 4, 150000, 5000, 0, 1, 150000, 32, 197209, 0,
            1, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000,
            32, 150000, 32, 3345831, 1, 1,
        ];
        let cm = arr.iter().fold((CostModel::new(), 0), |(mut cm, i), x| {
            cm.set(i, &Int::new_i32(x.clone()));
            (cm, i + 1)
        }).0;
        let mut cms = Costmdls::new();
        cms.insert(&Language::new_plutus_v1(), &cm);
        assert_eq!(
            hex::encode(cms.language_views_encoding()),
            "a141005901d59f1a000302590001011a00060bc719026d00011a000249f01903e800011a000249f018201a0025cea81971f70419744d186419744d186419744d186419744d186419744d186419744d18641864186419744d18641a000249f018201a000249f018201a000249f018201a000249f01903e800011a000249f018201a000249f01903e800081a000242201a00067e2318760001011a000249f01903e800081a000249f01a0001b79818f7011a000249f0192710011a0002155e19052e011903e81a000249f01903e8011a000249f018201a000249f018201a000249f0182001011a000249f0011a000249f0041a000194af18f8011a000194af18f8011a0002377c190556011a0002bdea1901f1011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000242201a00067e23187600010119f04c192bd200011a000249f018201a000242201a00067e2318760001011a000242201a00067e2318760001011a0025cea81971f704001a000141bb041a000249f019138800011a000249f018201a000302590001011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a00330da70101ff"
        );
    }
}
