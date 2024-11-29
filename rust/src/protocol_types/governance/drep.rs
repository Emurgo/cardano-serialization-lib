use std::convert::TryFrom;
use bech32::ToBase32;
use crate::*;
use crate::protocol_types::governance::cip129_decoder::{GovIdType, GovernanceIdentifier};

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

    pub fn to_bech32(&self, cip_129_format: bool) -> Result<String, JsError> {
        if cip_129_format {
            let gov_identifier: GovernanceIdentifier = self.try_into()?;
            gov_identifier.to_bech32()
        } else {
            let (hrp, data) = match &self.0 {
                DRepEnum::KeyHash(keyhash) => Ok(("drep_vkh", keyhash.to_bytes())),
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
    }

    pub fn from_bech32(bech32_str: &str) -> Result<DRep, JsError> {
        let (hrp, u5data) =
            bech32::decode(bech32_str).map_err(|e| JsError::from_str(&e.to_string()))?;
        let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data)
            .map_err(|e: bech32::Error| JsError::from_str(&format!("Malformed DRep base32: {}", &e.to_string())))?;
        let prefix = hrp.as_str();
        match prefix {
            "drep" => match data.len() {
                28 => Self::from_bech32_internal(prefix, data),
                29 => GovernanceIdentifier::from_bech32(bech32_str)?.try_into(),
                _ => Err(JsError::from_str("Malformed DRep (drep1 byte len)"))
            },
            _ => Self::from_bech32_internal(prefix, data),
        }
    }

    fn from_bech32_internal(prefix: &str, data: Vec<u8>) -> Result<DRep, JsError> {
        let kind = match prefix {
            "drep" => DRepKind::KeyHash,
            "drep_vkh" => DRepKind::KeyHash,
            "drep_script" => DRepKind::ScriptHash,
            _ => return Err(JsError::from_str("Malformed DRep")),
        };
        let drep = match kind {
            DRepKind::KeyHash => DRepEnum::KeyHash(
                Ed25519KeyHash::from_bytes(data)
                    .map_err(|_| JsError::from_str("Malformed DRep"))?,
            ),
            DRepKind::ScriptHash => DRepEnum::ScriptHash(
                ScriptHash::from_bytes(data).map_err(|_| JsError::from_str("Malformed DRep"))?,
            ),
            DRepKind::AlwaysAbstain => DRepEnum::AlwaysAbstain,
            DRepKind::AlwaysNoConfidence => DRepEnum::AlwaysNoConfidence,
        };
        Ok(DRep(drep))
    }
}

impl TryFrom<&DRep> for Credential {
    type Error = JsError;

    fn try_from(drep: &DRep) -> Result<Self, Self::Error> {
        match &drep.0 {
            DRepEnum::KeyHash(keyhash) => Ok(Credential(CredType::Key(keyhash.clone()))),
            DRepEnum::ScriptHash(scripthash) => Ok(Credential(CredType::Script(scripthash.clone()))),
            DRepEnum::AlwaysAbstain => Err(JsError::from_str("Cannot convert AlwaysAbstain to Credential")),
            DRepEnum::AlwaysNoConfidence => Err(JsError::from_str("Cannot convert AlwaysNoConfidence to Credential")),
        }
    }
}

impl TryFrom<&DRep> for GovernanceIdentifier {
    type Error = JsError;

    fn try_from(drep: &DRep) -> Result<Self, Self::Error> {
        let credential = drep.try_into()?;
        Ok(GovernanceIdentifier::GovCredential {
            gov_id_type: GovIdType::DRep,
            credential,
        })
    }
}

impl TryFrom<GovernanceIdentifier> for DRep {
    type Error = JsError;

    fn try_from(gov_id: GovernanceIdentifier) -> Result<Self, Self::Error> {
        match gov_id {
            GovernanceIdentifier::GovCredential {
                gov_id_type: GovIdType::DRep,
                credential,
            } => Ok(DRep::new_from_credential(&credential)),
            _ => Err(JsError::from_str("Cannot convert GovernanceActionId to DRep")),
        }
    }
}
