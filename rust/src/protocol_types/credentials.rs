use crate::*;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct Credentials {
    pub(crate) credentials: Vec<Credential>,
    pub(crate) dedup: BTreeSet<Credential>
}

impl_to_from!(Credentials);

#[wasm_bindgen]
impl Credentials {
    pub fn new() -> Self {
        Self {
            credentials: Vec::new(),
            dedup: BTreeSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.credentials.len()
    }

    pub fn get(&self, index: usize) -> Credential {
        self.credentials[index].clone()
    }

    pub fn add(&mut self, elem: &Credential) {
        if self.dedup.insert(elem.clone()) {
            self.credentials.push(elem.clone());
        }
    }

    pub(crate) fn add_move(&mut self, elem: Credential) {
        if self.dedup.insert(elem.clone()) {
            self.credentials.push(elem);
        }
    }

    pub(crate) fn contains(&self, elem: &Credential) -> bool {
        self.dedup.contains(elem)
    }

    pub(crate) fn from_vec(vec: Vec<Credential>) -> Self {
        let mut dedup = BTreeSet::new();
        let mut credentials = Vec::new();
        for elem in vec {
            if dedup.insert(elem.clone()) {
                credentials.push(elem);
            }
        }
        Self {
            credentials,
            dedup
        }
    }

    pub(crate) fn from_iter(iter: impl IntoIterator<Item = Credential>) -> Self {
        let mut dedup = BTreeSet::new();
        let mut credentials = Vec::new();
        for elem in iter {
            if dedup.insert(elem.clone()) {
                credentials.push(elem);
            }
        }
        Self {
            credentials,
            dedup
        }
    }

    pub(crate) fn to_vec(&self) -> &Vec<Credential> {
        &self.credentials
    }
}


impl serde::Serialize for Credentials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.credentials.serialize(serializer)
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