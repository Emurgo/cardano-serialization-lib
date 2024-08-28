use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct Certificates {
    pub(crate) certs: Vec<Certificate>,
    pub(crate) dedup: BTreeSet<Certificate>
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
            dedup: BTreeSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.certs.len()
    }

    pub fn get(&self, index: usize) -> Certificate {
        self.certs[index].clone()
    }

    /// Add a new `Certificate` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, elem: &Certificate) -> bool {
        if self.dedup.insert(elem.clone()) {
            self.certs.push(elem.clone());
            true
        } else {
            false
        }
    }

    pub(crate) fn add_move(&mut self, elem: Certificate) {
        if self.dedup.insert(elem.clone()) {
            self.certs.push(elem);
        }
    }


    pub(crate) fn from_vec(certs_vec: Vec<Certificate>) -> Self {
        let mut certs = Self::new();
        for cert in certs_vec {
            certs.add_move(cert);
        }
        certs
    }
}

impl serde::Serialize for Certificates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.certs.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Certificates {
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
