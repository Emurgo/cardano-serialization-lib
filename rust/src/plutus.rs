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
    pub fn new(bytes: Vec<u8>) -> PlutusScript {
        Self(bytes)
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusScripts(Vec<PlutusScript>);

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
    tag: Int,
    data: PlutusList,
}

to_from_bytes!(ConstrPlutusData);

#[wasm_bindgen]
impl ConstrPlutusData {
    pub fn tag(&self) -> Int {
        self.tag.clone()
    }

    pub fn data(&self) -> PlutusList {
        self.data.clone()
    }

    pub fn new(tag: Int, data: &PlutusList) -> Self {
        Self {
            tag,
            data: data.clone(),
        }
    }
}

impl ConstrPlutusData {
    fn is_tag_compact(tag: i128) -> bool {
        (tag >= 121 && tag <= 127) || (tag >= 1280 && tag <= 1400)
    }

    const GENERAL_FORM_TAG: u64 = 102;
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CostModel(std::collections::BTreeMap<String, PreludeInteger>);

to_from_bytes!(CostModel);

#[wasm_bindgen]
impl CostModel {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: String, value: &PreludeInteger) -> Option<PreludeInteger> {
        self.0.insert(key, value.clone())
    }

    pub fn get(&self, key: String) -> Option<PreludeInteger> {
        self.0.get(&key).map(|v| v.clone())
    }

    pub fn keys(&self) -> Strings {
        Strings(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
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
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExUnitPrices {
    mem_price: Coin,
    step_price: Coin,
}

to_from_bytes!(ExUnitPrices);

#[wasm_bindgen]
impl ExUnitPrices {
    pub fn mem_price(&self) -> Coin {
        self.mem_price.clone()
    }

    pub fn step_price(&self) -> Coin {
        self.step_price.clone()
    }

    pub fn new(mem_price: &Coin, step_price: &Coin) -> Self {
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
        PlutusList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlutusDataKind {
    ConstrPlutusData,
    PlutusMap,
    PlutusList,
    PreludeInteger,
    Bytes,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlutusDataEnum {
    ConstrPlutusData(ConstrPlutusData),
    PlutusMap(PlutusMap),
    PlutusList(PlutusList),
    PreludeInteger(PreludeInteger),
    Bytes(Vec<u8>),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusData(PlutusDataEnum);

const PLUTUS_BYTES_MAX_LEN: usize = 64;

to_from_bytes!(PlutusDataEnum);

#[wasm_bindgen]
impl PlutusData {
    pub fn new_constr_plutus_data(constr_plutus_data: &ConstrPlutusData) -> Self {
        Self(PlutusDataEnum::ConstrPlutusData(constr_plutus_data.clone()))
    }

    pub fn new_map(map: &PlutusMap) -> Self {
        Self(PlutusDataEnum::PlutusMap(map.clone()))
    }

    pub fn new_list(list: &PlutusList) -> Self {
        Self(PlutusDataEnum::PlutusList(list.clone()))
    }

    pub fn new_integer(integer: &PreludeInteger) -> Self {
        Self(PlutusDataEnum::PreludeInteger(integer.clone()))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Result<PlutusData, JsError> {
        if bytes.len() > PLUTUS_BYTES_MAX_LEN {
            Err(JsError::from_str(&format!("Max Plutus bytes too long: {}, max = {}", bytes.len(), PLUTUS_BYTES_MAX_LEN)))
        } else {
            Ok(Self(PlutusDataEnum::Bytes(bytes)))
        }
    }

    pub fn kind(&self) -> PlutusDataKind {
        match &self.0 {
            PlutusDataEnum::ConstrPlutusData(_) => PlutusDataKind::ConstrPlutusData,
            PlutusDataEnum::PlutusMap(_) => PlutusDataKind::PlutusMap,
            PlutusDataEnum::PlutusList(_) => PlutusDataKind::PlutusList,
            PlutusDataEnum::PreludeInteger(_) => PlutusDataKind::PreludeInteger,
            PlutusDataEnum::Bytes(_) => PlutusDataKind::Bytes,
        }
    }

    pub fn as_constr_plutus_data(&self) -> Option<ConstrPlutusData> {
        match &self.0 {
            PlutusDataEnum::ConstrPlutusData(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<PlutusMap> {
        match &self.0 {
            PlutusDataEnum::PlutusMap(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<PlutusList> {
        match &self.0 {
            PlutusDataEnum::PlutusList(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<PreludeInteger> {
        match &self.0 {
            PlutusDataEnum::PreludeInteger(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.0 {
            PlutusDataEnum::Bytes(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlutusList(Vec<PlutusData>);

#[wasm_bindgen]
impl PlutusList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusData {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusData) {
        self.0.push(elem.clone());
    }
}

// TODO: replace these prelude ints with a generalized Integer class
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PreludeBigintKind {
    PreludeBiguint,
    PreludeBignint,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PreludeBigintEnum {
    PreludeBiguint(PreludeBiguint),
    PreludeBignint(PreludeBignint),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PreludeBigint(PreludeBigintEnum);

to_from_bytes!(PreludeBigintEnum);

#[wasm_bindgen]
impl PreludeBigint {
    pub fn new_prelude_biguint(prelude_biguint: &PreludeBiguint) -> Self {
        Self(PreludeBigintEnum::PreludeBiguint(prelude_biguint.clone()))
    }

    pub fn new_prelude_bignint(prelude_bignint: &PreludeBignint) -> Self {
        Self(PreludeBigintEnum::PreludeBignint(prelude_bignint.clone()))
    }

    pub fn kind(&self) -> PreludeBigintKind {
        match &self.0 {
            PreludeBigintEnum::PreludeBiguint(_) => PreludeBigintKind::PreludeBiguint,
            PreludeBigintEnum::PreludeBignint(_) => PreludeBigintKind::PreludeBignint,
        }
    }

    pub fn as_prelude_biguint(&self) -> Option<PreludeBiguint> {
        match &self.0 {
            PreludeBigintEnum::PreludeBiguint(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_prelude_bignint(&self) -> Option<PreludeBignint> {
        match &self.0 {
            PreludeBigintEnum::PreludeBignint(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PreludeBignint(Vec<u8>);

#[wasm_bindgen]
impl PreludeBignint {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PreludeBiguint(Vec<u8>);

#[wasm_bindgen]
impl PreludeBiguint {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PreludeIntegerKind {
    Int,
    PreludeBigint,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PreludeIntegerEnum {
    Int(Int),
    PreludeBigint(PreludeBigint),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PreludeInteger(PreludeIntegerEnum);

to_from_bytes!(PreludeIntegerEnum);

#[wasm_bindgen]
impl PreludeInteger {
    pub fn new_int(int: &Int) -> Self {
        Self(PreludeIntegerEnum::Int(int.clone()))
    }

    pub fn new_prelude_bigint(prelude_bigint: &PreludeBigint) -> Self {
        Self(PreludeIntegerEnum::PreludeBigint(prelude_bigint.clone()))
    }

    pub fn kind(&self) -> PreludeIntegerKind {
        match &self.0 {
            PreludeIntegerEnum::Int(_) => PreludeIntegerKind::Int,
            PreludeIntegerEnum::PreludeBigint(_) => PreludeIntegerKind::PreludeBigint,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        match &self.0 {
            PreludeIntegerEnum::Int(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_prelude_bigint(&self) -> Option<PreludeBigint> {
        match &self.0 {
            PreludeIntegerEnum::PreludeBigint(x) => Some(x.clone()),
            _ => None,
        }
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RedeemerTagKind {
    Spend,
    Mint,
    Cert,
    Reward,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RedeemerTagEnum {
    Spend,
    Mint,
    Cert,
    Reward,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RedeemerTag(RedeemerTagEnum);

to_from_bytes!(RedeemerTagEnum);

#[wasm_bindgen]
impl RedeemerTag {
    pub fn new_i0() -> Self {
        Self(RedeemerTagEnum::Spend)
    }

    pub fn new_i1() -> Self {
        Self(RedeemerTagEnum::Mint)
    }

    pub fn new_i2() -> Self {
        Self(RedeemerTagEnum::Cert)
    }

    pub fn new_i3() -> Self {
        Self(RedeemerTagEnum::Reward)
    }

    pub fn kind(&self) -> RedeemerTagKind {
        match &self.0 {
            RedeemerTagEnum::Spend => RedeemerTagKind::Spend,
            RedeemerTagEnum::Mint => RedeemerTagKind::Mint,
            RedeemerTagEnum::Cert => RedeemerTagKind::Cert,
            RedeemerTagEnum::Reward => RedeemerTagKind::Reward,
        }
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
        if Self::is_tag_compact(self.tag.0) {
            // compact form
            serializer.write_tag(self.tag.0 as u64)?;
            self.data.serialize(serializer)
        } else {
            // general form
            serializer.write_tag(Self::GENERAL_FORM_TAG)?;
            serializer.write_array(cbor_event::Len::Len(2))?;
            self.tag.serialize(serializer)?;
            self.data.serialize(serializer)
        }
    }
}

impl Deserialize for ConstrPlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (tag, data) = match raw.tag()? {
                // general form
                Self::GENERAL_FORM_TAG => {
                    let len = raw.array()?;
                    let mut read_len = CBORReadLen::new(len);
                    read_len.read_elems(2)?;
                    let tag = Int::deserialize(raw)?;
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
                    (tag, data)
                },
                // concise form
                tag if Self::is_tag_compact(tag.into()) => (Int::new(&to_bignum(tag)), PlutusList::deserialize(raw)?),
                invalid_tag => return Err(DeserializeFailure::TagMismatch{
                    found: invalid_tag,
                    expected: Self::GENERAL_FORM_TAG,
                }.into()),
            };
            Ok(ConstrPlutusData{
                tag,
                data,
            })
        })().map_err(|e| e.annotate("ConstrPlutusData"))
    }
}

impl cbor_event::se::Serialize for CostModel {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            serializer.write_text(&key)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for CostModel {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = String::deserialize(raw)?;
                let value = PreludeInteger::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(key)).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("CostModel"))?;
        Ok(Self(table))
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
                Ok(Coin::deserialize(raw)?)
            })().map_err(|e| e.annotate("mem_price"))?;
            let step_price = (|| -> Result<_, DeserializeError> {
                Ok(Coin::deserialize(raw)?)
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
            PlutusDataEnum::PlutusMap(x) => {
                x.serialize(serializer)
            },
            PlutusDataEnum::PlutusList(x) => {
                x.serialize(serializer)
            },
            PlutusDataEnum::PreludeInteger(x) => {
                x.serialize(serializer)
            },
            PlutusDataEnum::Bytes(x) => {
                serializer.write_bytes(&x)
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
                Ok(variant) => return Ok(PlutusDataEnum::PlutusMap(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PlutusList::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::PlutusList(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PreludeInteger::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PlutusDataEnum::PreludeInteger(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(raw.bytes()?)
            })(raw)
            {
                Ok(variant) => if variant.len() <= PLUTUS_BYTES_MAX_LEN {
                    return Ok(PlutusDataEnum::Bytes(variant));
                } else {
                    return Err(DeserializeFailure::OutOfRange{
                        min: 0,
                        max: PLUTUS_BYTES_MAX_LEN,
                        found: variant.len(),
                    }.into());
                }
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("PlutusDataEnum", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("PlutusDataEnum"))
    }
}

impl cbor_event::se::Serialize for PlutusData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for PlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(PlutusDataEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for PlutusList {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for PlutusList {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(PlutusData::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("PlutusList"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for PreludeBigintEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            PreludeBigintEnum::PreludeBiguint(x) => {
                x.serialize(serializer)
            },
            PreludeBigintEnum::PreludeBignint(x) => {
                x.serialize(serializer)
            },
        }
    }
}

impl Deserialize for PreludeBigintEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PreludeBiguint::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PreludeBigintEnum::PreludeBiguint(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PreludeBignint::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PreludeBigintEnum::PreludeBignint(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("PreludeBigintEnum", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("PreludeBigintEnum"))
    }
}

impl cbor_event::se::Serialize for PreludeBigint {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for PreludeBigint {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(PreludeBigintEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for PreludeBignint {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(3u64)?;
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for PreludeBignint {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let tag = raw.tag().map_err(|e| DeserializeError::from(e).annotate("PreludeBignint"))?;
        if tag != 3 {
            return Err(DeserializeError::new("PreludeBignint", DeserializeFailure::TagMismatch{ found: tag, expected: 3 }));
        }
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for PreludeBiguint {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(2u64)?;
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for PreludeBiguint {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let tag = raw.tag().map_err(|e| DeserializeError::from(e).annotate("PreludeBiguint"))?;
        if tag != 2 {
            return Err(DeserializeError::new("PreludeBiguint", DeserializeFailure::TagMismatch{ found: tag, expected: 2 }));
        }
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for PreludeIntegerEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            PreludeIntegerEnum::Int(x) => {
                x.serialize(serializer)
            },
            PreludeIntegerEnum::PreludeBigint(x) => {
                x.serialize(serializer)
            },
        }
    }
}

impl Deserialize for PreludeIntegerEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(Int::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PreludeIntegerEnum::Int(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PreludeBigint::deserialize(raw)?)
            })(raw)
            {
                Ok(variant) => return Ok(PreludeIntegerEnum::PreludeBigint(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("PreludeIntegerEnum", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("PreludeIntegerEnum"))
    }
}

impl cbor_event::se::Serialize for PreludeInteger {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for PreludeInteger {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(PreludeIntegerEnum::deserialize(raw)?))
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

impl cbor_event::se::Serialize for RedeemerTagEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            RedeemerTagEnum::Spend => {
                serializer.write_unsigned_integer(0u64)
            },
            RedeemerTagEnum::Mint => {
                serializer.write_unsigned_integer(1u64)
            },
            RedeemerTagEnum::Cert => {
                serializer.write_unsigned_integer(2u64)
            },
            RedeemerTagEnum::Reward => {
                serializer.write_unsigned_integer(3u64)
            },
        }
    }
}

impl Deserialize for RedeemerTagEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let i0_value = raw.unsigned_integer()?;
                if i0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i0_value), expected: Key::Uint(0) }.into());
                }
                Ok(())
            })(raw)
            {
                Ok(()) => return Ok(RedeemerTagEnum::Spend),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let i1_value = raw.unsigned_integer()?;
                if i1_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i1_value), expected: Key::Uint(1) }.into());
                }
                Ok(())
            })(raw)
            {
                Ok(()) => return Ok(RedeemerTagEnum::Mint),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let i2_value = raw.unsigned_integer()?;
                if i2_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i2_value), expected: Key::Uint(2) }.into());
                }
                Ok(())
            })(raw)
            {
                Ok(()) => return Ok(RedeemerTagEnum::Cert),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let i3_value = raw.unsigned_integer()?;
                if i3_value != 3 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i3_value), expected: Key::Uint(3) }.into());
                }
                Ok(())
            })(raw)
            {
                Ok(()) => return Ok(RedeemerTagEnum::Reward),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("RedeemerTagEnum", DeserializeFailure::NoVariantMatched.into()))
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
        Ok(Self(RedeemerTagEnum::deserialize(raw)?))
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