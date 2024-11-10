use core::prelude::v1::Ok;
use crate::*;
use bech32::ToBase32;

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
pub(crate) enum DRepEnum {
    KeyHash(Ed25519KeyHash),
    ScriptHash(ScriptHash),
    AlwaysAbstain,
    AlwaysNoConfidence,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum DRepKind {
    KeyHash,
    ScriptHash,
    AlwaysAbstain,
    AlwaysNoConfidence,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum DRepExtendedId {
    Key(Ed25519KeyHash),
    Script(ScriptHash),
}

const DREP_CIP129_PREFIX_KEY: u8 = 34;
const DREP_CIP129_PREFIX_SCRIPT: u8 = 35;

#[wasm_bindgen]
impl DRepExtendedId {

    pub fn from_bytes(data: &Vec<u8>) -> Result<DRepExtendedId, JsError> {
        if data.len() != 29 {
            return Err(JsError::from_str("Malformed DRep Extended ID (incorrect len)"));
        }
        let prefix = data.get(0)
            .ok_or(JsError::from_str("Malformed DRep (unexpected failure to get bytes prefix)"))?;
        match prefix {
            &DREP_CIP129_PREFIX_KEY => return Ok(
                DRepExtendedId::Key(
                    Ed25519KeyHash::from_bytes(data[1..].to_vec())
                        .map_err(|_| JsError::from_str("Malformed DRep KeyHash"))?,
                )
            ),
            &DREP_CIP129_PREFIX_SCRIPT => return Ok(
                DRepExtendedId::Script(
                    ScriptHash::from_bytes(data[1..].to_vec())
                        .map_err(|_| JsError::from_str("Malformed DRep ScriptHash"))?,
                )
            ),
            _ => return Err(JsError::from_str("Malformed DRep Extended ID (incorrect prefix byte)"))
        }
    }

    pub fn from_hex(hex_str: &str) -> Result<DRepExtendedId, JsError> {
        DRepExtendedId::from_bytes(
            &hex::decode(hex_str)
                .map_err(|e| JsError::from_str(&e.to_string()))?
        )
    }

    pub fn from_bech32(bech32_str: &str) -> Result<DRepExtendedId, JsError> {
        let (hrp, u5data) =
            bech32::decode(bech32_str).map_err(|e| JsError::from_str(&e.to_string()))?;
        if hrp != "drep" {
            return Err(JsError::from_str("Malformed DRep Extended ID (incorrect prefix)"));
        }
        let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data)
            .map_err(|e: bech32::Error| JsError::from_str(&format!("Malformed DRep base32: {}", &e.to_string())))?;
        DRepExtendedId::from_bytes(&data)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let (prefix, data) = match self {
            DRepExtendedId::Key(keyhash) => (DREP_CIP129_PREFIX_KEY, keyhash.to_bytes()),
            DRepExtendedId::Script(scripthash) => (DREP_CIP129_PREFIX_SCRIPT, scripthash.to_bytes()),
        };
        let mut res = vec![prefix];
        res.extend(data);
        res
    }

    pub fn to_bech32(&self) -> Result<String, JsError> {
        bech32::encode("drep", self.to_bytes().to_base32()).map_err(|e| JsError::from_str(&format! {"{:?}", e}))
    }

    pub fn kind(&self) -> DRepKind {
        match self {
            DRepExtendedId::Key(_) => DRepKind::KeyHash,
            DRepExtendedId::Script(_) => DRepKind::ScriptHash,
        }
    }

    pub fn to_drep(&self) -> DRep {
        match self {
            DRepExtendedId::Key(key) => DRep(DRepEnum::KeyHash(key.clone())),
            DRepExtendedId::Script(script) => DRep(DRepEnum::ScriptHash(script.clone())),
        }
    }
}

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
#[wasm_bindgen]
pub struct DRep(pub(crate) DRepEnum);

impl_to_from!(DRep);

#[wasm_bindgen]
impl DRep {
    pub fn new_key_hash(key_hash: &Ed25519KeyHash) -> Self {
        Self(DRepEnum::KeyHash(key_hash.clone()))
    }

    pub fn new_script_hash(script_hash: &ScriptHash) -> Self {
        Self(DRepEnum::ScriptHash(script_hash.clone()))
    }

    pub fn new_always_abstain() -> Self {
        Self(DRepEnum::AlwaysAbstain)
    }

    pub fn new_always_no_confidence() -> Self {
        Self(DRepEnum::AlwaysNoConfidence)
    }

    pub fn new_from_credential(cred: &Credential) -> Self {
        let drep = match &cred.0 {
            CredType::Key(key_hash) => DRepEnum::KeyHash(key_hash.clone()),
            CredType::Script(script_hash) => DRepEnum::ScriptHash(script_hash.clone()),
        };
        Self(drep)
    }

    pub fn kind(&self) -> DRepKind {
        match &self.0 {
            DRepEnum::KeyHash(_) => DRepKind::KeyHash,
            DRepEnum::ScriptHash(_) => DRepKind::ScriptHash,
            DRepEnum::AlwaysAbstain => DRepKind::AlwaysAbstain,
            DRepEnum::AlwaysNoConfidence => DRepKind::AlwaysNoConfidence,
        }
    }

    pub fn to_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            DRepEnum::KeyHash(keyhash) => Some(keyhash.clone()),
            _ => None,
        }
    }

    pub fn to_script_hash(&self) -> Option<ScriptHash> {
        match &self.0 {
            DRepEnum::ScriptHash(scripthash) => Some(scripthash.clone()),
            _ => None,
        }
    }

    pub fn to_extended_id(&self) -> Option<DRepExtendedId> {
        match &self.0 {
            DRepEnum::KeyHash(x) => Some(DRepExtendedId::Key(x.clone())),
            DRepEnum::ScriptHash(x) => Some(DRepExtendedId::Script(x.clone())),
            _ => None
        }
    }

    fn internal_to_bech32(&self, vkh_prefix: String) -> Result<String, JsError> {
        let (hrp, data) = match &self.0 {
            DRepEnum::KeyHash(keyhash) => Ok((vkh_prefix.as_str(), keyhash.to_bytes())),
            DRepEnum::ScriptHash(scripthash) => Ok(("drep_script", scripthash.to_bytes())),
            DRepEnum::AlwaysAbstain => {
                Err(JsError::from_str("Cannot convert AlwaysAbstain to bech32"))
            }
            DRepEnum::AlwaysNoConfidence => Err(JsError::from_str(
                "Cannot convert AlwaysNoConfidence to bech32",
            )),
        }?;
        bech32::encode(&hrp, data.to_base32()).map_err(|e| JsError::from_str(&format! {"{:?}", e}))
    }

    pub fn to_bech32(&self) -> Result<String, JsError> {
        self.internal_to_bech32("drep".to_string())
    }

    pub fn to_bech32_cip129(&self) -> Result<String, JsError> {
        self.internal_to_bech32("drep_vkh".to_string())
    }

    pub fn from_bech32(bech32_str: &str) -> Result<DRep, JsError> {
        let (hrp, u5data) =
            bech32::decode(bech32_str).map_err(|e| JsError::from_str(&e.to_string()))?;
        let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data)
            .map_err(|e: bech32::Error| JsError::from_str(&format!("Malformed DRep base32: {}", &e.to_string())))?;
        let kind = match hrp.as_str() {
            "drep" => match data.len() {
                28 => DRepKind::KeyHash, // pre cip129 compatibility
                29 => return DRepExtendedId::from_bech32(bech32_str).and_then(|id| Ok(id.to_drep())),
                _ => return Err(JsError::from_str("Malformed DRep (drep1 byte len)"))
            },
            "drep_vkh" => DRepKind::KeyHash,
            "drep_script" => DRepKind::ScriptHash,
            _ => return Err(JsError::from_str("Malformed DRep (bech prefix)")),
        };
        let drep = match kind {
            DRepKind::KeyHash => DRepEnum::KeyHash(
                Ed25519KeyHash::from_bytes(data)
                    .map_err(|_| JsError::from_str("Malformed DRep KeyHash"))?,
            ),
            DRepKind::ScriptHash => DRepEnum::ScriptHash(
                ScriptHash::from_bytes(data).map_err(|_| JsError::from_str("Malformed DRep ScriptHash"))?,
            ),
            DRepKind::AlwaysAbstain => DRepEnum::AlwaysAbstain,
            DRepKind::AlwaysNoConfidence => DRepEnum::AlwaysNoConfidence,
        };
        Ok(DRep(drep))
    }
}
