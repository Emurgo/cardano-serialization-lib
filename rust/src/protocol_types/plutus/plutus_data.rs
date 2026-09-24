use crate::*;
use core::hash::Hasher;
use hashlink::LinkedHashMap;
use std::hash::Hash;

use cbor_event::{
    self,
    de::Deserializer,
    se::{Serialize, Serializer},
};

use schemars::JsonSchema;
use serde::ser::SerializeStruct;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ConstrPlutusData {
    pub(crate) alternative: BigNum,
    pub(crate) data: PlutusList,
}

to_from_bytes!(ConstrPlutusData);

#[macro_export]
macro_rules! plutus_constr {
    ($variant:expr $(, $field:expr)* $(,)?) => {
        $crate::ConstrPlutusData::new(
            &$crate::BigNum::from($variant),
            &$crate::plutus_list![$($field),*]
        )
    };
}

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
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize)]
pub struct PlutusMapValues {
    pub(crate) elems: Vec<PlutusData>,
}

impl_vec_wrapper!(PlutusMapValues, PlutusData, elems);

impl From<Vec<PlutusData>> for PlutusMapValues {
    fn from(elems: Vec<PlutusData>) -> Self {
        Self { elems }
    }
}

impl From<PlutusData> for PlutusMapValues {
    fn from(data: PlutusData) -> Self {
        Self { elems: vec![data] }
    }
}

#[wasm_bindgen]
impl PlutusMapValues {
    pub fn new() -> Self {
        Self { elems: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }

    pub fn get(&self, index: usize) -> Option<PlutusData> {
        self.elems.get(index).cloned()
    }

    pub fn add(&mut self, elem: &PlutusData) {
        self.elems.push(elem.clone());
    }

    pub(crate) fn add_move(&mut self, elem: PlutusData) {
        self.elems.push(elem);
    }
}


#[wasm_bindgen]
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct PlutusMap(pub(crate) LinkedHashMap<PlutusData, PlutusMapValues>);

to_from_bytes!(PlutusMap);

#[macro_export]
macro_rules! plutus_map_values {
    ($($item:expr),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut values = $crate::PlutusMapValues::new();
            $(values.add(&$crate::PlutusData::from($item));)*
            values
        }
    };
}

#[macro_export]
macro_rules! plutus_map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut plutus_map = $crate::PlutusMap::new();
            $(plutus_map.insert(&$key.into(), &$value.into());)*
            plutus_map
        }
    };
}

#[wasm_bindgen]
impl PlutusMap {
    pub fn new() -> Self {
        Self(LinkedHashMap::new())
    }


    /// Return count ok different keys in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the previous value associated with the key, if any.
    /// Replace the values associated with the key.
    pub fn insert(&mut self, key: &PlutusData, values: &PlutusMapValues) -> Option<PlutusMapValues> {
        self.0.insert(key.clone(), values.clone())
    }

    pub fn get(&self, key: &PlutusData) -> Option<PlutusMapValues> {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn keys(&self) -> PlutusList {
        PlutusList {
            elems: self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>(),
            definite_encoding: None,
            cbor_set_type: None,
        }
    }

    /// Adds a value to the list of values associated with the key.
    pub(crate) fn add_value(&mut self, key: &PlutusData, value: &PlutusData) {
        let values = self.0
            .entry(key.clone())
            .or_insert_with(PlutusMapValues::new);
        values.add(value);
    }

    pub(crate) fn add_value_move(&mut self, key: PlutusData, value: PlutusData) {
        let values = self.0
            .entry(key)
            .or_insert_with(PlutusMapValues::new);
        values.add_move(value);
    }

    pub(crate) fn total_len(&self) -> usize {
        self.0.iter().map(|(_k, v)| v.len()).sum()
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

impl From<ConstrPlutusData> for PlutusDataEnum {
    fn from(value: ConstrPlutusData) -> Self {
        Self::ConstrPlutusData(value)
    }
}

impl From<PlutusMap> for PlutusDataEnum {
    fn from(value: PlutusMap) -> Self {
        Self::Map(value)
    }
}

impl From<PlutusList> for PlutusDataEnum {
    fn from(value: PlutusList) -> Self {
        Self::List(value)
    }
}

impl From<BigInt> for PlutusDataEnum {
    fn from(value: BigInt) -> Self {
        Self::Integer(value)
    }
}

impl From<BigNum> for PlutusDataEnum {
    fn from(value: BigNum) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u64> for PlutusDataEnum {
    fn from(value: u64) -> Self {
        Self::Integer(BigNum::from(value).into())
    }
}

impl From<i32> for PlutusDataEnum {
    fn from(value: i32) -> Self {
        Self::Integer(BigInt::from(value))
    }
}

impl From<i64> for PlutusDataEnum {
    fn from(value: i64) -> Self {
        Self::Integer(BigInt::from(value))
    }
}

impl From<Vec<u8>> for PlutusDataEnum {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Bytes(bytes)
    }
}

impl From<&[u8]> for PlutusDataEnum {
    fn from(bytes: &[u8]) -> Self {
        Self::Bytes(bytes.to_vec())
    }
}

#[macro_export]
macro_rules! plutus_bytes {
    ($($byte:expr),* $(,)?) => {
        $crate::PlutusData::from(vec![$($byte),*])
    };
    ($elem:expr; $n:expr) => {
        $crate::PlutusData::from(vec![$elem ; $n])
    };
}

#[wasm_bindgen]
pub struct PlutusData {
    pub(crate) datum: PlutusDataEnum,
    // We should always preserve the original datums when deserialized as this is NOT canonicized
    // before computing datum hashes. So this field stores the original bytes to re-use.
    pub(crate) original_bytes: Option<Vec<u8>>,
}

impl<T> From<T> for PlutusData where PlutusDataEnum: From<T> {
    fn from(value: T) -> Self {
        Self {
            datum: value.into(),
            original_bytes: None
        }
    }
}

impl std::cmp::PartialEq<Self> for PlutusData {
    fn eq(&self, other: &Self) -> bool {
        plutus_data_eq(self, other)
    }
}

impl Hash for PlutusData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_plutus_datum(&self.datum, state)
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

    pub(crate) fn into_datum(self) -> PlutusDataEnum {
        let this = std::mem::ManuallyDrop::new(self);
        unsafe {
            let original_bytes = std::ptr::read(&this.original_bytes);
            drop(original_bytes);
            std::ptr::read(&this.datum)
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
            AddrType::Reward(_) => Err(JsError::from_str(
                "Cannot convert Reward address to PlutusData",
            )),
            AddrType::Byron(_) => Err(JsError::from_str(
                "Cannot convert Byron address to PlutusData",
            )),
            AddrType::Malformed(_) => Err(JsError::from_str(
                "Cannot convert Malformed address to PlutusData",
            ))
        }?;

        let staking_data = match &address.0 {
            AddrType::Base(addr) => {
                let staking_bytes_data = PlutusData::from_credential(&addr.stake_cred())?;
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

        let payment_data = PlutusData::from_credential(&payment_cred)?;
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

    pub fn as_address(&self, network: &NetworkInfo) -> Result<Address, JsError> {
        let constr_data = self.as_constr_plutus_data().ok_or_else(|| {
            JsError::from_str("Expected PlutusData to be a ConstrPlutusData")
        })?;

        if constr_data.alternative().0 != 0 {
            return Err(JsError::from_str(
                "Expected alternative 0 in ConstrPlutusData",
            ));
        }

        let data_list = constr_data.data();
        if data_list.len() != 2 {
            return Err(JsError::from_str(
                "Expected exactly 2 elements in the data list",
            ));
        }

        let payment_data = data_list.get(0);
        let payment_cred = payment_data.as_credential()?;

        let staking_data = data_list.get(1);
        let staking_cred_option =
            if let Some(constr_staking_data) = staking_data.as_constr_plutus_data()
        {
            match constr_staking_data.alternative().0 {
                0 => {
                    let staking_inner_data = constr_staking_data.data();
                    if staking_inner_data.len() != 1 {
                        return Err(JsError::from_str(
                            "Expected exactly 1 element in staking inner data",
                        ));
                    }
                    let constr_staking_inner_data = staking_inner_data.get(0);
                    Some(constr_staking_inner_data)
                }
                1 => {
                    None
                }
                _ => {
                    return Err(JsError::from_str(
                        "Invalid alternative in staking data constructor",
                    ));
                }
            }
        } else {
            return Err(JsError::from_str(
                "Expected staking data to be a ConstrPlutusData",
            ));
        };

        let network = network.network_id();
        let address = if let Some(staking_cred_data) = staking_cred_option {
            let staking_cred_constr = staking_cred_data.as_constr_plutus_data().ok_or_else(|| {
                JsError::from_str("Expected staking data to be a ConstrPlutusData")
            })?;
            match staking_cred_constr.alternative.0 {
                0 => {
                    let staking_cred_constr_data = staking_cred_constr.data();
                    if staking_cred_constr_data.len() != 1 {
                        return Err(JsError::from_str(
                            "Expected exactly 1 element in staking data constructor",
                        ));
                    }
                    let staking_cred = staking_cred_constr_data.get(0).as_credential()?;
                    BaseAddress::new(network, &payment_cred, &staking_cred).to_address()
                }
                1 => {
                    let pointer = staking_cred_data.as_pointer()?;
                    PointerAddress::new(network, &payment_cred, &pointer).to_address()
                }
                _ => {
                    return Err(JsError::from_str(
                        "Invalid alternative in staking data constructor",
                    ));
                }
            }
        } else {
            EnterpriseAddress::new(network, &payment_cred).to_address()
        };

        Ok(address)
    }

    fn from_credential(stake_credential: &Credential) -> Result<PlutusData, JsError> {
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

    fn as_credential(&self) -> Result<Credential, JsError> {
        let constr_data = self.as_constr_plutus_data().ok_or_else(|| {
            JsError::from_str("Expected PlutusData to be a ConstrPlutusData")
        })?;

        let index = constr_data.alternative().0;

        let data_list = constr_data.data();
        if data_list.len() != 1 {
            return Err(JsError::from_str(
                "Expected exactly one element in ConstrPlutusData data",
            ));
        }

        let bytes_data = data_list.get(0);
        let bytes = bytes_data.as_bytes().ok_or_else(|| {
            JsError::from_str("Expected the inner PlutusData to be bytes")
        })?;

        match index {
            0 => {
                let key_hash = Ed25519KeyHash::from_bytes(bytes).map_err(|_| {
                    JsError::from_str("Invalid KeyHash bytes")
                })?;
                Ok(Credential(CredType::Key(key_hash)))
            }
            1 => {
                let script_hash = ScriptHash::from_bytes(bytes).map_err(|_| {
                    JsError::from_str("Invalid ScriptHash bytes")
                })?;
                Ok(Credential(CredType::Script(script_hash)))
            }
            _ => Err(JsError::from_str(
                "Invalid alternative index in ConstrPlutusData for Credential",
            )),
        }
    }

    fn from_pointer(pointer: &Pointer) -> Result<PlutusData, JsError> {
        Ok(ConstrPlutusData::new(&1u64.into(), &plutus_list![
            pointer.slot_bignum(),
            pointer.tx_index_bignum(),
            pointer.cert_index_bignum()
        ]).into())
    }

    fn as_pointer(&self) -> Result<Pointer, JsError> {
        let constr_data = self.as_constr_plutus_data().ok_or_else(|| {
            JsError::from_str("Expected PlutusData to be a ConstrPlutusData")
        })?;

        if constr_data.alternative() != BigNum(1) {
            return Err(JsError::from_str("Expected alternative 1 in ConstrPlutusData"));
        }

        let data_list = constr_data.data();
        if data_list.len() != 3 {
            return Err(JsError::from_str("Expected exactly 3 elements in data list"));
        }

        let slot_data = data_list.get(0);
        let tx_index_data = data_list.get(1);
        let cert_index_data = data_list.get(2);

        let slot_integer = slot_data.as_integer().ok_or_else(|| {
            JsError::from_str("Expected slot data to be an integer")
        })?;
        let tx_index_integer = tx_index_data.as_integer().ok_or_else(|| {
            JsError::from_str("Expected tx_index data to be an integer")
        })?;
        let cert_index_integer = cert_index_data.as_integer().ok_or_else(|| {
            JsError::from_str("Expected cert_index data to be an integer")
        })?;

        let slot_bignum = slot_integer
            .as_u64()
            .ok_or_else(|| JsError::from_str("Slot integer must be positive"))?;
        let tx_index_bignum = tx_index_integer
            .as_u64()
            .ok_or_else(|| JsError::from_str("Tx index integer must be positive"))?;
        let cert_index_bignum = cert_index_integer
            .as_u64()
            .ok_or_else(|| JsError::from_str("Cert index integer must be positive"))?;

        let pointer = Pointer::new_pointer(&slot_bignum, &tx_index_bignum, &cert_index_bignum);

        Ok(pointer)
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
#[derive(Clone, Debug, Default)]
pub struct PlutusList {
    pub(crate) elems: Vec<PlutusData>,
    // We should always preserve the original datums when deserialized as this is NOT canonicized
    // before computing datum hashes. This field will default to cardano-cli behavior if None
    // and will re-use the provided one if deserialized, unless the list is modified.
    pub(crate) definite_encoding: Option<bool>,
    pub(crate) cbor_set_type: Option<CborSetType>,
}

to_from_bytes!(PlutusList);
impl_vec_wrapper!(PlutusList, PlutusData, elems);

#[macro_export]
macro_rules! plutus_list {
    ($($item:expr),* $(,)?) => {
        $crate::PlutusList::from(vec![$($crate::PlutusData::from($item)),*])
    };
    ($elem:expr; $n:expr) => {
        {
            let elem = $crate::PlutusData::from($elem);
            $crate::PlutusList::from(vec![ elem ; $n ])
        }
    };
}

#[wasm_bindgen]
impl PlutusList {
    pub fn new() -> Self {
        Self {
            elems: Vec::new(),
            definite_encoding: None,
            cbor_set_type: None,
        }
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }

    pub fn get(&self, index: usize) -> PlutusData {
        self.elems[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusData) {
        self.push(elem.clone());
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, elem: &PlutusData) -> bool {
        self.elems.contains(elem)
    }

    pub(crate) fn deduplicated_view(&self) -> Vec<&PlutusData> {
        let mut dedup = BTreeSet::new();
        let mut datas = Vec::new();
        for elem in &self.elems {
            if dedup.insert(elem) {
                datas.push(elem);
            }
        }
        datas
    }

    pub(crate) fn to_set_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize_as_set(true, &mut buf).unwrap();
        buf.finalize()
    }

    pub(crate) fn deduplicated_clone(&self) -> Self {
        let mut dedup = BTreeSet::new();
        let mut elems = Vec::new();
        for elem in &self.elems {
            if dedup.insert(elem) {
                elems.push(elem.clone());
            }
        }
        Self {
            elems,
            definite_encoding: self.definite_encoding,
            cbor_set_type: self.cbor_set_type.clone(),
        }
    }

    pub(crate) fn extend(&mut self, other: &PlutusList) {
        self.elems.extend(other.elems.iter().cloned());
    }

    #[allow(dead_code)]
    pub(crate) fn set_set_type(&mut self, set_type: CborSetType) {
        self.cbor_set_type = Some(set_type);
    }

    pub(crate) fn get_set_type(&self) -> Option<CborSetType> {
        self.cbor_set_type.clone()
    }
}

impl PlutusList {
    pub fn push(&mut self, data: PlutusData) {
        self.elems.push(data);
        self.definite_encoding = None;
    }
}

#[derive(serde::Deserialize, JsonSchema)]
struct PlutusListFields {
    elems: Vec<PlutusData>,
    definite_encoding: Option<bool>,
}

impl serde::Serialize for PlutusList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PlutusList", 2)?;
        state.serialize_field("elems", &self.elems)?;
        state.serialize_field("definite_encoding", &self.definite_encoding)?;
        state.end()
    }
}


impl<'de> serde::de::Deserialize<'de> for PlutusList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = PlutusListFields::deserialize(deserializer)?;
        Ok(Self {
            elems: fields.elems,
            definite_encoding: fields.definite_encoding,
            cbor_set_type: None,
        })
    }
}

impl JsonSchema for PlutusList {
    fn schema_name() -> String {
        String::from("PlutusList")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        PlutusListFields::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        PlutusListFields::is_referenceable()
    }
}

impl PartialOrd for PlutusList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.elems.partial_cmp(&other.elems) {
            Some(core::cmp::Ordering::Equal) => self.definite_encoding.partial_cmp(&other.definite_encoding),
            non_eq => non_eq,
        }
    }
}

impl Hash for PlutusList {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elems.hash(state);
        self.definite_encoding.hash(state);
    }
}

impl Ord for PlutusList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.elems.cmp(&other.elems) {
            core::cmp::Ordering::Equal => self.definite_encoding.cmp(&other.definite_encoding),
            non_eq => non_eq,
        }
    }
}

impl std::cmp::PartialEq<Self> for PlutusList {
    fn eq(&self, other: &Self) -> bool {
        self.elems.eq(&other.elems)
    }
}

impl std::cmp::Eq for PlutusList {}

impl From<Vec<PlutusData>> for PlutusList {
    fn from(elems: Vec<PlutusData>) -> Self {
        Self {
            elems,
            definite_encoding: None,
            cbor_set_type: None,
        }
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

    #[cfg(feature = "arbitrary-precision-json")]
    fn encode_number(x: serde_json::Number) -> Result<PlutusData, JsError> {
        if let Ok(big_int) = BigInt::from_str(x.as_str()) {
            Ok(PlutusData::new_integer(&big_int))
        } else {
            Err(JsError::from_str(&format!("Expected an integer value but got \"{}\"", x)))
        }
    }

    #[cfg(not(feature = "arbitrary-precision-json"))]
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
                    map.add_value(&key, &value);
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
                                map.add_value(
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
                        .and_then(|v| Some(v.as_u64()?.into()))
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
    let value = plutus_datum_to_json_val(datum, schema)?;
    Ok(crate::stack_safe::stringify_json(&value, false))
}

pub fn decode_plutus_datum_to_json_value(
    datum: &PlutusData,
    schema: PlutusDatumSchema,
) -> Result<serde_json::Value, JsError> {
    let value = plutus_datum_to_json_val(datum, schema)?;
    Ok(crate::stack_safe::json_val_to_serde(value))
}

fn wrap_json_tag(tag: &str, value: crate::stack_safe::JsonVal) -> crate::stack_safe::JsonVal {
    crate::stack_safe::JsonVal::Object(vec![(tag.to_string(), value)])
}

fn plutus_basic_map_key(key: &PlutusData) -> Result<String, JsError> {
    match &key.datum {
        PlutusDataEnum::ConstrPlutusData(_) => Err(JsError::from_str(
            "plutus data constructors are not allowed as keys in this schema. Use DetailedSchema.",
        )),
        PlutusDataEnum::Map(_) => Err(JsError::from_str(
            "plutus maps are not allowed as keys in this schema. Use DetailedSchema.",
        )),
        PlutusDataEnum::List(_) => Err(JsError::from_str(
            "plutus lists are not allowed as keys in this schema. Use DetailedSchema.",
        )),
        PlutusDataEnum::Integer(x) => Ok(x.to_str()),
        PlutusDataEnum::Bytes(bytes) => String::from_utf8(bytes.clone())
            .or_else(|_err| Ok(format!("0x{}", hex::encode(bytes)))),
    }
}

fn plutus_bytes_json(bytes: &[u8], schema: PlutusDatumSchema) -> String {
    match schema {
        PlutusDatumSchema::BasicConversions => String::from_utf8(bytes.to_vec())
            .ok()
            .filter(|utf8| utf8.chars().all(|c| !c.is_control()))
            .unwrap_or_else(|| format!("0x{}", hex::encode(bytes))),
        PlutusDatumSchema::DetailedSchema => hex::encode(bytes),
    }
}

fn plutus_int_json_number(bigint: &BigInt) -> Result<crate::stack_safe::JsonVal, JsError> {
    #[cfg(not(feature = "arbitrary-precision-json"))]
    {
        if bigint.as_int().is_none() {
            return Err(JsError::from_str(&format!(
                "Integer {} too big for our JSON support",
                bigint.to_str()
            )));
        }
    }
    Ok(crate::stack_safe::JsonVal::Number(bigint.to_str()))
}

enum PlutusToJsonKind {
    List { wrap: bool },
    Constr { alternative: String },
    MapBasic { keys: std::vec::IntoIter<String> },
    MapDetailed,
}

struct PlutusToJsonFrame<'a> {
    kind: PlutusToJsonKind,
    rest: std::vec::IntoIter<&'a PlutusData>,
    acc: Vec<crate::stack_safe::JsonVal>,
}

fn finish_plutus_to_json(
    kind: PlutusToJsonKind,
    acc: Vec<crate::stack_safe::JsonVal>,
) -> crate::stack_safe::JsonVal {
    use crate::stack_safe::JsonVal;
    match kind {
        PlutusToJsonKind::List { wrap } => {
            let arr = JsonVal::Array(acc);
            if wrap {
                wrap_json_tag("list", arr)
            } else {
                arr
            }
        }
        PlutusToJsonKind::Constr { alternative } => JsonVal::Object(vec![
            ("constructor".to_string(), JsonVal::Number(alternative)),
            ("fields".to_string(), JsonVal::Array(acc)),
        ]),
        PlutusToJsonKind::MapBasic { keys } => {
            let mut obj = Vec::new();
            let mut keys = keys;
            for v in acc {
                obj.push((keys.next().expect("plutus basic map json keys"), v));
            }
            // `serde_json::Map` semantics as in 16.0.0: keys sorted, a repeated key keeps its last value.
            JsonVal::object_sorted(obj)
        }
        PlutusToJsonKind::MapDetailed => {
            let mut entries = Vec::new();
            let mut iter = acc.into_iter();
            while let Some(k) = iter.next() {
                let v = iter.next().expect("plutus detailed map k/v pairs");
                entries.push(JsonVal::Object(vec![
                    ("k".to_string(), k),
                    ("v".to_string(), v),
                ]));
            }
            wrap_json_tag("map", JsonVal::Array(entries))
        }
    }
}

fn plutus_datum_to_json_val(
    root: &PlutusData,
    schema: PlutusDatumSchema,
) -> Result<crate::stack_safe::JsonVal, JsError> {
    use crate::stack_safe::JsonVal;
    let mut stack: Vec<PlutusToJsonFrame<'_>> = Vec::new();
    let mut node = root;
    loop {
        let mut completed = match &node.datum {
            PlutusDataEnum::Integer(bigint) => {
                let n = plutus_int_json_number(bigint)?;
                if schema == PlutusDatumSchema::DetailedSchema {
                    wrap_json_tag("int", n)
                } else {
                    n
                }
            }
            PlutusDataEnum::Bytes(bytes) => {
                let s = JsonVal::String(plutus_bytes_json(bytes, schema));
                if schema == PlutusDatumSchema::DetailedSchema {
                    wrap_json_tag("bytes", s)
                } else {
                    s
                }
            }
            PlutusDataEnum::List(list) => {
                let kind = PlutusToJsonKind::List {
                    wrap: schema == PlutusDatumSchema::DetailedSchema,
                };
                let mut rest = list.elems.iter();
                if let Some(first) = rest.next() {
                    stack.push(PlutusToJsonFrame {
                        kind,
                        rest: rest.collect::<Vec<_>>().into_iter(),
                        acc: Vec::new(),
                    });
                    node = first;
                    continue;
                }
                finish_plutus_to_json(kind, Vec::new())
            }
            PlutusDataEnum::ConstrPlutusData(constr) => {
                let kind = PlutusToJsonKind::Constr {
                    alternative: constr.alternative.0.to_string(),
                };
                let mut rest = constr.data.elems.iter();
                if let Some(first) = rest.next() {
                    stack.push(PlutusToJsonFrame {
                        kind,
                        rest: rest.collect::<Vec<_>>().into_iter(),
                        acc: Vec::new(),
                    });
                    node = first;
                    continue;
                }
                finish_plutus_to_json(kind, Vec::new())
            }
            PlutusDataEnum::Map(map) => match schema {
                PlutusDatumSchema::BasicConversions => {
                    let mut keys = Vec::new();
                    let mut children = Vec::new();
                    for (key, values) in map.0.iter() {
                        if values.len() > 1 {
                            return Err(JsError::from_str("plutus maps are not allowed to have more than one value per key in this schema. Use DetailedSchema."));
                        }
                        let value = values.elems.get(0).ok_or_else(|| {
                            JsError::from_str("plutus maps are not allowed to have empty values in this schema. Use DetailedSchema.")
                        })?;
                        keys.push(plutus_basic_map_key(key)?);
                        children.push(value);
                    }
                    let kind = PlutusToJsonKind::MapBasic {
                        keys: keys.into_iter(),
                    };
                    let mut rest = children.into_iter();
                    if let Some(first) = rest.next() {
                        stack.push(PlutusToJsonFrame {
                            kind,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_plutus_to_json(kind, Vec::new())
                }
                PlutusDatumSchema::DetailedSchema => {
                    let mut children = Vec::new();
                    for (key, values) in map.0.iter() {
                        for value in &values.elems {
                            children.push(key);
                            children.push(value);
                        }
                    }
                    let kind = PlutusToJsonKind::MapDetailed;
                    let mut rest = children.into_iter();
                    if let Some(first) = rest.next() {
                        stack.push(PlutusToJsonFrame {
                            kind,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    finish_plutus_to_json(kind, Vec::new())
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
                        completed = finish_plutus_to_json(frame.kind, frame.acc);
                    }
                }
            }
        }
    }
}



impl Drop for PlutusData {
    fn drop(&mut self) {
        crate::stack_safe::iterative_drop(self, take_plutus_children);
    }
}

fn take_plutus_children(data: &mut PlutusData, stack: &mut Vec<PlutusData>) {
    match &mut data.datum {
        PlutusDataEnum::ConstrPlutusData(constr) => {
            stack.extend(std::mem::take(&mut constr.data.elems));
        }
        PlutusDataEnum::List(list) => {
            stack.extend(std::mem::take(&mut list.elems));
        }
        PlutusDataEnum::Map(map) => {
            for (key, values) in std::mem::take(&mut map.0) {
                stack.push(key);
                stack.extend(values.elems);
            }
        }
        PlutusDataEnum::Integer(_) | PlutusDataEnum::Bytes(_) => {}
    }
}

enum PlutusCloneKind {
    Constr {
        alternative: BigNum,
        definite_encoding: Option<bool>,
        set_type: Option<CborSetType>,
        original_bytes: Option<Vec<u8>>,
    },
    List {
        definite_encoding: Option<bool>,
        set_type: Option<CborSetType>,
        original_bytes: Option<Vec<u8>>,
    },
    Map {
        original_bytes: Option<Vec<u8>>,
        /// Flattened key/value children: for each map value list of len L we emit
        /// key then L values. `value_lens` remembers L per key so we can rebuild.
        value_lens: Vec<usize>,
    },
}

struct PlutusCloneFrame<'a> {
    kind: PlutusCloneKind,
    rest: std::vec::IntoIter<&'a PlutusData>,
    acc: Vec<PlutusData>,
}

fn plutus_from_cloned(
    kind: PlutusCloneKind,
    acc: Vec<PlutusData>,
) -> PlutusData {
    match kind {
        PlutusCloneKind::Constr {
            alternative,
            definite_encoding,
            set_type,
            original_bytes,
        } => PlutusData {
            datum: PlutusDataEnum::ConstrPlutusData(ConstrPlutusData {
                alternative,
                data: PlutusList {
                    elems: acc,
                    definite_encoding,
                    cbor_set_type: set_type,
                },
            }),
            original_bytes,
        },
        PlutusCloneKind::List {
            definite_encoding,
            set_type,
            original_bytes,
        } => PlutusData {
            datum: PlutusDataEnum::List(PlutusList {
                elems: acc,
                definite_encoding,
                cbor_set_type: set_type,
            }),
            original_bytes,
        },
        PlutusCloneKind::Map {
            original_bytes,
            value_lens,
        } => {
            let mut map = LinkedHashMap::new();
            let mut iter = acc.into_iter();
            for len in value_lens {
                let key = iter.next().unwrap();
                let mut values = PlutusMapValues::new();
                for _ in 0..len {
                    values.add_move(iter.next().unwrap());
                }
                map.insert(key, values);
            }
            PlutusData {
                datum: PlutusDataEnum::Map(PlutusMap(map)),
                original_bytes,
            }
        }
    }
}

impl Clone for PlutusData {
    fn clone(&self) -> Self {
        let mut stack: Vec<PlutusCloneFrame<'_>> = Vec::new();
        let mut node = self;
        loop {
            let mut completed = match &node.datum {
                PlutusDataEnum::Integer(x) => PlutusData {
                    datum: PlutusDataEnum::Integer(x.clone()),
                    original_bytes: node.original_bytes.clone(),
                },
                PlutusDataEnum::Bytes(x) => PlutusData {
                    datum: PlutusDataEnum::Bytes(x.clone()),
                    original_bytes: node.original_bytes.clone(),
                },
                PlutusDataEnum::ConstrPlutusData(c) => {
                    let kind = PlutusCloneKind::Constr {
                        alternative: c.alternative,
                        definite_encoding: c.data.definite_encoding,
                        set_type: c.data.cbor_set_type.clone(),
                        original_bytes: node.original_bytes.clone(),
                    };
                    let mut rest = c.data.elems.iter();
                    if let Some(first) = rest.next() {
                        stack.push(PlutusCloneFrame {
                            kind,
                            rest: rest.collect::<Vec<_>>().into_iter(),
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    plutus_from_cloned(kind, Vec::new())
                }
                PlutusDataEnum::List(l) => {
                    let kind = PlutusCloneKind::List {
                        definite_encoding: l.definite_encoding,
                        set_type: l.cbor_set_type.clone(),
                        original_bytes: node.original_bytes.clone(),
                    };
                    let mut rest = l.elems.iter();
                    if let Some(first) = rest.next() {
                        stack.push(PlutusCloneFrame {
                            kind,
                            rest: rest.collect::<Vec<_>>().into_iter(),
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    plutus_from_cloned(kind, Vec::new())
                }
                PlutusDataEnum::Map(m) => {
                    let mut children = Vec::new();
                    let mut value_lens = Vec::new();
                    for (k, vs) in m.0.iter() {
                        children.push(k);
                        value_lens.push(vs.elems.len());
                        children.extend(vs.elems.iter());
                    }
                    let kind = PlutusCloneKind::Map {
                        original_bytes: node.original_bytes.clone(),
                        value_lens,
                    };
                    let mut rest = children.into_iter();
                    if let Some(first) = rest.next() {
                        stack.push(PlutusCloneFrame {
                            kind,
                            rest,
                            acc: Vec::new(),
                        });
                        node = first;
                        continue;
                    }
                    plutus_from_cloned(kind, Vec::new())
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
                            completed = plutus_from_cloned(frame.kind, frame.acc);
                        }
                    }
                }
            }
        }
    }
}

fn plutus_data_eq(a: &PlutusData, b: &PlutusData) -> bool {
    let mut stack = vec![(a, b)];
    while let Some((left, right)) = stack.pop() {
        match (&left.datum, &right.datum) {
            (PlutusDataEnum::Integer(x), PlutusDataEnum::Integer(y)) => {
                if x != y {
                    return false;
                }
            }
            (PlutusDataEnum::Bytes(x), PlutusDataEnum::Bytes(y)) => {
                if x != y {
                    return false;
                }
            }
            (PlutusDataEnum::List(x), PlutusDataEnum::List(y)) => {
                if x.elems.len() != y.elems.len() {
                    return false;
                }
                stack.extend(x.elems.iter().zip(y.elems.iter()));
            }
            (PlutusDataEnum::ConstrPlutusData(x), PlutusDataEnum::ConstrPlutusData(y)) => {
                if x.alternative != y.alternative || x.data.elems.len() != y.data.elems.len() {
                    return false;
                }
                stack.extend(x.data.elems.iter().zip(y.data.elems.iter()));
            }
            (PlutusDataEnum::Map(x), PlutusDataEnum::Map(y)) => {
                if x.0.len() != y.0.len() {
                    return false;
                }
                for ((lk, lv), (rk, rv)) in x.0.iter().zip(y.0.iter()) {
                    if lv.elems.len() != rv.elems.len() {
                        return false;
                    }
                    stack.push((lk, rk));
                    stack.extend(lv.elems.iter().zip(rv.elems.iter()));
                }
            }
            _ => return false,
        }
    }
    true
}

fn hash_plutus_datum<H: Hasher>(datum: &PlutusDataEnum, state: &mut H) {
    hash_plutus_datum_iter(datum, state);
}

fn hash_plutus_datum_iter<H: Hasher>(root: &PlutusDataEnum, state: &mut H) {
    enum Job<'a> {
        Datum(&'a PlutusDataEnum),
        Data(&'a PlutusData),
        Disc(u8),
        Alt(BigNum),
        Bytes(&'a [u8]),
        OptBool(Option<bool>),
        Integer(&'a BigInt),
    }
    let mut stack = vec![Job::Datum(root)];
    while let Some(job) = stack.pop() {
        match job {
            Job::Disc(d) => d.hash(state),
            Job::Alt(n) => n.hash(state),
            Job::Bytes(b) => b.hash(state),
            Job::OptBool(v) => v.hash(state),
            Job::Integer(i) => i.hash(state),
            Job::Data(data) => stack.push(Job::Datum(&data.datum)),
            Job::Datum(datum) => match datum {
                PlutusDataEnum::ConstrPlutusData(c) => {
                    stack.push(Job::OptBool(c.data.definite_encoding));
                    for elem in c.data.elems.iter().rev() {
                        stack.push(Job::Data(elem));
                    }
                    stack.push(Job::Alt(c.alternative));
                    stack.push(Job::Disc(0));
                }
                PlutusDataEnum::Map(m) => {
                    for (k, vs) in m.0.iter().rev() {
                        for v in vs.elems.iter().rev() {
                            stack.push(Job::Data(v));
                        }
                        stack.push(Job::Data(k));
                    }
                    stack.push(Job::Disc(1));
                }
                PlutusDataEnum::List(l) => {
                    stack.push(Job::OptBool(l.definite_encoding));
                    for elem in l.elems.iter().rev() {
                        stack.push(Job::Data(elem));
                    }
                    stack.push(Job::Disc(2));
                }
                PlutusDataEnum::Integer(i) => {
                    stack.push(Job::Integer(i));
                    stack.push(Job::Disc(3));
                }
                PlutusDataEnum::Bytes(b) => {
                    stack.push(Job::Bytes(b));
                    stack.push(Job::Disc(4));
                }
            },
        }
    }
}

enum PlutusCmpJob<'a> {
    Data(&'a PlutusData, &'a PlutusData),
    Datum(&'a PlutusDataEnum, &'a PlutusDataEnum),
    Bytes(&'a [u8], &'a [u8]),
    Ints(&'a BigInt, &'a BigInt),
    Nums(BigNum, BigNum),
    OptBool(Option<bool>, Option<bool>),
    Orig(Option<&'a [u8]>, Option<&'a [u8]>),
    Kind(u8, u8),
    Lens(usize, usize),
}

impl PartialOrd for PlutusData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlutusData {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut stack = vec![
            PlutusCmpJob::Orig(
                self.original_bytes.as_deref(),
                other.original_bytes.as_deref(),
            ),
            PlutusCmpJob::Datum(&self.datum, &other.datum),
        ];
        while let Some(job) = stack.pop() {
            let ord = match job {
                PlutusCmpJob::Kind(a, b) => a.cmp(&b),
                PlutusCmpJob::Lens(a, b) => a.cmp(&b),
                PlutusCmpJob::Nums(a, b) => a.cmp(&b),
                PlutusCmpJob::OptBool(a, b) => a.cmp(&b),
                PlutusCmpJob::Ints(a, b) => a.cmp(&b),
                PlutusCmpJob::Bytes(a, b) => a.cmp(&b),
                PlutusCmpJob::Orig(a, b) => a.cmp(&b),
                PlutusCmpJob::Data(a, b) => {
                    stack.push(PlutusCmpJob::Orig(
                        a.original_bytes.as_deref(),
                        b.original_bytes.as_deref(),
                    ));
                    stack.push(PlutusCmpJob::Datum(&a.datum, &b.datum));
                    Ordering::Equal
                }
                PlutusCmpJob::Datum(a, b) => {
                    match (a, b) {
                        (PlutusDataEnum::Integer(x), PlutusDataEnum::Integer(y)) => {
                            stack.push(PlutusCmpJob::Ints(x, y));
                        }
                        (PlutusDataEnum::Bytes(x), PlutusDataEnum::Bytes(y)) => {
                            stack.push(PlutusCmpJob::Bytes(x, y));
                        }
                        (PlutusDataEnum::List(x), PlutusDataEnum::List(y)) => {
                            stack.push(PlutusCmpJob::OptBool(
                                x.definite_encoding,
                                y.definite_encoding,
                            ));
                            push_plutus_vec_cmp(&mut stack, &x.elems, &y.elems);
                        }
                        (PlutusDataEnum::ConstrPlutusData(x), PlutusDataEnum::ConstrPlutusData(y)) => {
                            stack.push(PlutusCmpJob::OptBool(
                                x.data.definite_encoding,
                                y.data.definite_encoding,
                            ));
                            push_plutus_vec_cmp(&mut stack, &x.data.elems, &y.data.elems);
                            stack.push(PlutusCmpJob::Nums(x.alternative, y.alternative));
                        }
                        (PlutusDataEnum::Map(x), PlutusDataEnum::Map(y)) => {
                            let xs: Vec<_> = x.0.iter().collect();
                            let ys: Vec<_> = y.0.iter().collect();
                            stack.push(PlutusCmpJob::Lens(xs.len(), ys.len()));
                            let min_len = xs.len().min(ys.len());
                            for i in (0..min_len).rev() {
                                let (lk, lv) = xs[i];
                                let (rk, rv) = ys[i];
                                stack.push(PlutusCmpJob::Lens(lv.elems.len(), rv.elems.len()));
                                let vmin = lv.elems.len().min(rv.elems.len());
                                for j in (0..vmin).rev() {
                                    stack.push(PlutusCmpJob::Data(&lv.elems[j], &rv.elems[j]));
                                }
                                stack.push(PlutusCmpJob::Data(lk, rk));
                            }
                        }
                        _ => {}
                    }
                    stack.push(PlutusCmpJob::Kind(plutus_kind_index(a), plutus_kind_index(b)));
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

fn plutus_kind_index(datum: &PlutusDataEnum) -> u8 {
    match datum {
        PlutusDataEnum::ConstrPlutusData(_) => 0,
        PlutusDataEnum::Map(_) => 1,
        PlutusDataEnum::List(_) => 2,
        PlutusDataEnum::Integer(_) => 3,
        PlutusDataEnum::Bytes(_) => 4,
    }
}

fn push_plutus_vec_cmp<'a>(
    stack: &mut Vec<PlutusCmpJob<'a>>,
    xs: &'a [PlutusData],
    ys: &'a [PlutusData],
) {
    stack.push(PlutusCmpJob::Lens(xs.len(), ys.len()));
    let min_len = xs.len().min(ys.len());
    for i in (0..min_len).rev() {
        stack.push(PlutusCmpJob::Data(&xs[i], &ys[i]));
    }
}

impl std::fmt::Debug for PlutusData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.datum {
            PlutusDataEnum::Integer(x) => f.debug_tuple("PlutusData").field(x).finish(),
            PlutusDataEnum::Bytes(x) => f.debug_tuple("PlutusData").field(x).finish(),
            PlutusDataEnum::List(x) => f
                .debug_tuple("PlutusData")
                .field(&format_args!("List(len={})", x.elems.len()))
                .finish(),
            PlutusDataEnum::Map(x) => f
                .debug_tuple("PlutusData")
                .field(&format_args!("Map(len={})", x.0.len()))
                .finish(),
            PlutusDataEnum::ConstrPlutusData(x) => f
                .debug_tuple("PlutusData")
                .field(&format_args!(
                    "Constr(alt={}, len={})",
                    x.alternative,
                    x.data.elems.len()
                ))
                .finish(),
        }
    }
}

#[cfg(test)]
impl ConstrPlutusData {
    pub(crate) fn new_moved(alternative: BigNum, data: PlutusList) -> Self {
        Self { alternative, data }
    }
}

#[cfg(test)]
impl PlutusData {
    pub(crate) fn from_list_moved(list: PlutusList) -> Self {
        Self {
            datum: PlutusDataEnum::List(list),
            original_bytes: None,
        }
    }

    pub(crate) fn from_constr_moved(constr: ConstrPlutusData) -> Self {
        Self {
            datum: PlutusDataEnum::ConstrPlutusData(constr),
            original_bytes: None,
        }
    }
}
