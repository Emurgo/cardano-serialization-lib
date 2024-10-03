use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;
use itertools::Itertools;
use crate::*;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
)]
pub struct Credentials {
    pub(crate) credentials: Vec<Rc<Credential>>,
    pub(crate) dedup: BTreeSet<Rc<Credential>>,
    pub(crate) cbor_set_type: CborSetType,
}

impl_to_from!(Credentials);

#[wasm_bindgen]
impl Credentials {
    pub fn new() -> Self {
        Self {
            credentials: Vec::new(),
            dedup: BTreeSet::new(),
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub(crate) fn new_from_prepared_fields(
        credentials: Vec<Rc<Credential>>,
        dedup: BTreeSet<Rc<Credential>>,
    ) -> Self {
        Self {
            credentials,
            dedup,
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub fn len(&self) -> usize {
        self.credentials.len()
    }

    pub fn get(&self, index: usize) -> Credential {
        self.credentials[index].deref().clone()
    }

    /// Add a new `Credential` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self,credential: &Credential) -> bool {
        let credential_rc = Rc::new(credential.clone());
        if self.dedup.insert(credential_rc.clone()) {
            self.credentials.push(credential_rc);
            true
        } else {
            false
        }
    }

    pub(crate) fn add_move(&mut self, credential: Credential) {
        let credential_rc = Rc::new(credential);
        if self.dedup.insert(credential_rc.clone()) {
            self.credentials.push(credential_rc);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, elem: &Credential) -> bool {
        self.dedup.contains(elem)
    }

    pub(crate) fn from_vec(vec: Vec<Credential>) -> Self {
        let mut dedup = BTreeSet::new();
        let mut credentials = Vec::new();
        for elem in vec {
            let elem_rc = Rc::new(elem);
            if dedup.insert(elem_rc.clone()) {
                credentials.push(elem_rc);
            }
        }
        Self::new_from_prepared_fields(credentials, dedup)
    }

    pub(crate) fn from_iter(iter: impl IntoIterator<Item = Credential>) -> Self {
        let mut dedup = BTreeSet::new();
        let mut credentials = Vec::new();
        for elem in iter {
            let elem_rc = Rc::new(elem);
            if dedup.insert(elem_rc.clone()) {
                credentials.push(elem_rc);
            }
        }
        Self::new_from_prepared_fields(credentials, dedup)
    }

    pub(crate) fn get_set_type(&self) -> CborSetType {
        self.cbor_set_type.clone()
    }

    pub(crate) fn set_set_type(&mut self, cbor_set_type: CborSetType) {
        self.cbor_set_type = cbor_set_type;
    }

}

impl PartialEq for Credentials {
    fn eq(&self, other: &Self) -> bool {
        self.credentials == other.credentials
    }
}

impl Eq for Credentials {}

impl PartialOrd for Credentials {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.credentials.partial_cmp(&other.credentials)
    }
}

impl Ord for Credentials {
    fn cmp(&self, other: &Self) -> Ordering {
        self.credentials.cmp(&other.credentials)
    }
}

impl serde::Serialize for Credentials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.credentials
            .iter()
            .map(|x| x.deref())
            .collect_vec()
            .serialize(serializer)
    }
}

impl Hash for Credentials {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.credentials.hash(state);
    }
}

impl<'de> serde::de::Deserialize<'de> for Credentials {
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

impl JsonSchema for Credentials {
    fn schema_name() -> String {
        String::from("Credentials")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<Credential>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<Credential>::is_referenceable()
    }
}