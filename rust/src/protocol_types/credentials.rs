use crate::*;
use linked_hash_map::LinkedHashMap;

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
pub struct CredentialsSet(pub(crate) BTreeSet<Credential>);

impl_to_from!(CredentialsSet);

#[wasm_bindgen]
impl CredentialsSet {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<Credential> {
        self.0.iter().nth(index).cloned()
    }

    pub fn add(&mut self, elem: &Credential) {
        self.0.insert(elem.clone());
    }

    pub fn contains(&self, elem: &Credential) -> bool {
        self.0.contains(elem)
    }

    pub fn to_vec(&self) -> Credentials {
        Credentials(self.0.iter().cloned().collect())
    }

    pub(crate) fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Credential>,
    {
        Self(iter.into_iter().collect())
    }

    pub(crate) fn add_move(&mut self, elem: Credential) {
        self.0.insert(elem);
    }
}

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct Credentials(pub(crate) Vec<Credential>);

impl_to_from!(Credentials);

#[wasm_bindgen]
impl Credentials {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Credential {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Credential) {
        self.0.push(elem.clone());
    }
}

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
    Key(Ed25519KeyHash),
    Script(ScriptHash),
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
