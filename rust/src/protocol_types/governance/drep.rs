use crate::*;

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
}
