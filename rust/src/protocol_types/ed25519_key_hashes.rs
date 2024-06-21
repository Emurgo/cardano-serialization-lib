pub use crate::*;

pub type RequiredSigners = Ed25519KeyHashes;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    Hash,
    PartialEq,
    PartialOrd,
)]
pub struct Ed25519KeyHashes {
    keyhashes: Vec<Ed25519KeyHash>,
    dedup: BTreeSet<Ed25519KeyHash>,
}

impl_to_from!(Ed25519KeyHashes);

#[wasm_bindgen]
impl Ed25519KeyHashes {
    pub fn new() -> Self {
        Self {
            keyhashes: Vec::new(),
            dedup: BTreeSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.keyhashes.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.keyhashes[index].clone()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) {
        if self.dedup.insert(elem.clone()) {
            self.keyhashes.push(elem.clone());
        }
    }

    pub fn contains(&self, elem: &Ed25519KeyHash) -> bool {
        self.dedup.contains(elem)
    }

    pub fn to_option(&self) -> Option<Ed25519KeyHashes> {
        if self.keyhashes.len() > 0 {
            Some(self.clone())
        } else {
            None
        }
    }

    pub(crate) fn to_vec(&self) -> &Vec<Ed25519KeyHash> {
        &self.keyhashes
    }

    pub(crate) fn add_move(&mut self, elem: Ed25519KeyHash) {
        if self.dedup.insert(elem.clone()) {
            self.keyhashes.push(elem);
        }
    }

    pub(crate) fn extend(&mut self, other: &Ed25519KeyHashes) {
        for keyhash in &other.keyhashes {
            self.add(keyhash);
        }
    }

    pub(crate) fn extend_move(&mut self, other: Ed25519KeyHashes) {
        for keyhash in other.keyhashes {
            self.add_move(keyhash);
        }
    }

    pub(crate) fn from_vec(keyhash_vec: Vec<Ed25519KeyHash>) -> Self {
        let mut dedup = BTreeSet::new();
        let mut keyhashes = Vec::new();
        for keyhash in keyhash_vec {
            if dedup.insert(keyhash.clone()) {
                keyhashes.push(keyhash);
            }
        }

        Self { keyhashes, dedup }
    }
}

impl NoneOrEmpty for Ed25519KeyHashes {
    fn is_none_or_empty(&self) -> bool {
        self.keyhashes.is_empty()
    }
}

impl serde::Serialize for Ed25519KeyHashes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.keyhashes.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Ed25519KeyHashes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let vec = <Vec<_> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::from_vec(vec))
    }
}

impl JsonSchema for Ed25519KeyHashes {
    fn schema_name() -> String {
        String::from("Ed25519KeyHashes")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<Ed25519KeyHash>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<Ed25519KeyHash>::is_referenceable()
    }
}