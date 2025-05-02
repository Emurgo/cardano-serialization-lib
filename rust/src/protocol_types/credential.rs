use crate::*;

#[derive(
Debug,
Clone,
Hash,
Eq,
Ord,
PartialEq,
PartialOrd,
serde::Serialize,
serde::Deserialize,
JsonSchema,
)]
pub enum CredType {
    Script(ScriptHash),
    Key(Ed25519KeyHash),
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CredKind {
    Key,
    Script,
}

#[wasm_bindgen]
#[derive(
Debug,
Clone,
Eq,
Hash,
Ord,
PartialEq,
PartialOrd,
serde::Serialize,
serde::Deserialize,
JsonSchema,
)]
pub struct Credential(pub(crate) CredType);

#[wasm_bindgen]
impl Credential {
    pub fn from_keyhash(hash: &Ed25519KeyHash) -> Self {
        Credential(CredType::Key(hash.clone()))
    }

    pub fn from_scripthash(hash: &ScriptHash) -> Self {
        Credential(CredType::Script(hash.clone()))
    }

    pub fn to_keyhash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            CredType::Key(hash) => Some(hash.clone()),
            CredType::Script(_) => None,
        }
    }

    pub fn to_scripthash(&self) -> Option<ScriptHash> {
        match &self.0 {
            CredType::Key(_) => None,
            CredType::Script(hash) => Some(hash.clone()),
        }
    }

    pub fn kind(&self) -> CredKind {
        match &self.0 {
            CredType::Key(_) => CredKind::Key,
            CredType::Script(_) => CredKind::Script,
        }
    }

    pub fn has_script_hash(&self) -> bool {
        match &self.0 {
            CredType::Key(_) => false,
            CredType::Script(_) => true,
        }
    }

    pub(crate) fn to_raw_bytes(&self) -> Vec<u8> {
        match &self.0 {
            CredType::Key(hash) => hash.to_bytes(),
            CredType::Script(hash) => hash.to_bytes(),
        }
    }
}

impl_to_from!(Credential);
