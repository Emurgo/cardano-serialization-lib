use crate::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::Map;
use itertools::Itertools;
use std::ops::Deref;
use std::slice;
use std::sync::Arc;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Certificates {
    pub(crate) certs: Vec<Arc<Certificate>>,
    pub(crate) dedup: HashSet<Arc<Certificate>>,
    pub(crate) cbor_set_type: CborSetType,
}

impl_to_from!(Certificates);

impl NoneOrEmpty for Certificates {
    fn is_none_or_empty(&self) -> bool {
        self.certs.is_empty()
    }
}

#[wasm_bindgen]
impl Certificates {
    pub fn new() -> Self {
        Self {
            certs: Vec::new(),
            dedup: HashSet::new(),
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub fn len(&self) -> usize {
        self.certs.len()
    }

    pub fn get(&self, index: usize) -> Certificate {
        self.certs[index].deref().clone()
    }

    /// Add a new `Certificate` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, elem: &Certificate) -> bool {
        let rc_elem = Arc::new(elem.clone());
        if self.dedup.insert(rc_elem.clone()) {
            self.certs.push(rc_elem.clone());
            true
        } else {
            false
        }
    }

    pub(crate) fn add_move(&mut self, elem: Certificate) {
        let rc_elem = Arc::new(elem);
        if self.dedup.insert(rc_elem.clone()) {
            self.certs.push(rc_elem.clone());
        }
    }

    pub(crate) fn from_vec(certs_vec: Vec<Certificate>) -> Self {
        let mut certs = Self::new();
        for cert in certs_vec {
            certs.add_move(cert);
        }
        certs
    }

    pub(crate) fn get_set_type(&self) -> CborSetType {
        self.cbor_set_type.clone()
    }

    pub(crate) fn set_set_type(&mut self, set_type: CborSetType) {
        self.cbor_set_type = set_type;
    }
}

impl PartialEq for Certificates {
    fn eq(&self, other: &Self) -> bool {
        self.certs == other.certs
    }
}

impl Eq for Certificates {}

impl PartialOrd for Certificates {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.certs.partial_cmp(&other.certs)
    }
}

impl Ord for Certificates {
    fn cmp(&self, other: &Self) -> Ordering {
        self.certs.cmp(&other.certs)
    }
}

impl Hash for Certificates {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.certs.hash(state);
    }
}

impl<'a> IntoIterator for &'a Certificates {
    type Item = &'a Certificate;
    type IntoIter = Map<
        slice::Iter<'a, Arc<Certificate>>,
        fn(&'a Arc<Certificate>) -> &'a Certificate,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.certs.iter().map(|rc| rc.as_ref())
    }
}

impl serde::Serialize for Certificates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.certs
            .iter()
            .map(|x| x.deref())
            .collect_vec()
            .serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Certificates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let vec = <Vec<_> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_vec(vec))
    }
}

impl JsonSchema for Certificates {
    fn schema_name() -> String {
        String::from("Certificates")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<Certificate>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<Certificate>::is_referenceable()
    }
}
