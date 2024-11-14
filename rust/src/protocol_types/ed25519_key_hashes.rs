use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;
use std::slice;
use std::iter::Map;
use itertools::Itertools;
pub use crate::*;

pub type RequiredSigners = Ed25519KeyHashes;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
)]
pub struct Ed25519KeyHashes {
    keyhashes: Vec<Rc<Ed25519KeyHash>>,
    dedup: HashSet<Rc<Ed25519KeyHash>>,
    cbor_set_type: CborSetType,
}

impl_to_from!(Ed25519KeyHashes);

#[wasm_bindgen]
impl Ed25519KeyHashes {
    pub fn new() -> Self {
        Self {
            keyhashes: Vec::new(),
            dedup: HashSet::new(),
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub(crate) fn new_from_prepared_fields(
        keyhashes: Vec<Rc<Ed25519KeyHash>>,
        dedup: HashSet<Rc<Ed25519KeyHash>>,
    ) -> Self {
        Self {
            keyhashes,
            dedup,
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub fn len(&self) -> usize {
        self.keyhashes.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.keyhashes[index].deref().clone()
    }

    /// Add a new `Ed25519KeyHash` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, keyhash: &Ed25519KeyHash) -> bool {
        let keyhash_rc = Rc::new(keyhash.clone());
        if self.dedup.insert(keyhash_rc.clone()) {
            self.keyhashes.push(keyhash_rc.clone());
            true
        } else {
            false
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

    pub(crate) fn add_move(&mut self, keyhash: Ed25519KeyHash) {
        let keyhash_rc = Rc::new(keyhash);
        if self.dedup.insert(keyhash_rc.clone()) {
            self.keyhashes.push(keyhash_rc);
        }
    }

    pub(crate) fn extend(&mut self, other: &Ed25519KeyHashes) {
        for keyhash in &other.keyhashes {
            self.add(keyhash);
        }
    }

    pub(crate) fn extend_move(&mut self, other: Ed25519KeyHashes) {
        for keyhash in other.keyhashes {
            if self.dedup.insert(keyhash.clone()) {
                self.keyhashes.push(keyhash);
            }
        }
    }

    pub(crate) fn from_vec(keyhash_vec: Vec<Ed25519KeyHash>) -> Self {
        let mut dedup = HashSet::new();
        let mut keyhashes = Vec::new();
        for keyhash in keyhash_vec {
            let keyhash_rc = Rc::new(keyhash.clone());
            if dedup.insert(keyhash_rc.clone()) {
                keyhashes.push(keyhash_rc);
            }
        }

        Self::new_from_prepared_fields(keyhashes, dedup)
    }

    pub(crate) fn get_set_type(&self) -> CborSetType {
        self.cbor_set_type.clone()
    }

    pub(crate) fn set_set_type(&mut self, cbor_set_type: CborSetType) {
        self.cbor_set_type = cbor_set_type;
    }
}

impl<'a> IntoIterator for &'a Ed25519KeyHashes {
    type Item = &'a Ed25519KeyHash;
    type IntoIter = Map<
        slice::Iter<'a, Rc<Ed25519KeyHash>>,
        fn(&'a Rc<Ed25519KeyHash>) -> &'a Ed25519KeyHash,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.keyhashes.iter().map(|rc| rc.as_ref())
    }
}

impl PartialEq for Ed25519KeyHashes {
    fn eq(&self, other: &Self) -> bool {
        self.keyhashes == other.keyhashes
    }
}

impl Eq for Ed25519KeyHashes {}

impl PartialOrd for Ed25519KeyHashes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.keyhashes.partial_cmp(&other.keyhashes)
    }
}

impl Ord for Ed25519KeyHashes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.keyhashes.cmp(&other.keyhashes)
    }
}

impl Hash for Ed25519KeyHashes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.keyhashes.hash(state);
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
        self.keyhashes
            .iter()
            .map(|x| x.deref())
            .collect_vec()
            .serialize(serializer)
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