pub use crate::*;

pub type RequiredSigners = Ed25519KeyHashes;

#[wasm_bindgen]
#[derive(
    Clone,
    Hash,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct Ed25519KeyHashesSet(pub(crate) BTreeSet<Ed25519KeyHash>);

impl_to_from!(Ed25519KeyHashesSet);

#[wasm_bindgen]
impl Ed25519KeyHashesSet {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn add(&mut self, key: &Ed25519KeyHash) {
        self.0.insert(key.clone());
    }

    pub fn remove(&mut self, key: &Ed25519KeyHash) {
        self.0.remove(key);
    }

    pub fn contains(&self, key: &Ed25519KeyHash) -> bool {
        self.0.contains(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<Ed25519KeyHash> {
        self.0.iter().nth(index).cloned()
    }

    pub fn to_vec(&self) -> RequiredSigners {
        Ed25519KeyHashes(self.0.iter().cloned().collect())
    }

    pub(crate) fn add_move(&mut self, key: Ed25519KeyHash) {
        self.0.insert(key);
    }

    pub(crate) fn extend(&mut self, keys: Ed25519KeyHashesSet) {
        self.0.extend(keys.0);
    }

    pub(crate) fn to_option(&self) -> Option<Ed25519KeyHashesSet> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.clone())
        }
    }
}

impl NoneOrEmpty for Ed25519KeyHashesSet {
    fn is_none_or_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl NoneOrEmpty for RequiredSigners {
    fn is_none_or_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&Ed25519KeyHashes> for Ed25519KeyHashesSet {
    fn from(keys: &Ed25519KeyHashes) -> Self {
        keys.0
            .iter()
            .fold(Ed25519KeyHashesSet::new(), |mut set, k| {
                set.add_move(k.clone());
                set
            })
    }
}
