use crate::*;
use std::collections::HashSet;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapWitnesses {
    witnesses: Vec<BootstrapWitness>,

    //for deduplication purpose
    dedup: HashSet<BootstrapWitness>,
}

impl_to_from!(BootstrapWitnesses);

#[wasm_bindgen]
impl BootstrapWitnesses {
    pub fn new() -> Self {
        Self {
            witnesses: Vec::new(),
            dedup: HashSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.witnesses.len()
    }

    pub fn get(&self, index: usize) -> BootstrapWitness {
        self.witnesses[index].clone()
    }

    /// Add a new `BootstrapWitness` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, elem: &BootstrapWitness) -> bool {
        if self.dedup.insert(elem.clone()) {
            self.witnesses.push(elem.clone());
            true
        } else {
            false
        }
    }

    pub(crate) fn get_vec_wits(&self) -> &Vec<BootstrapWitness> {
        &self.witnesses
    }

    pub(crate) fn from_vec_wits(wits: Vec<BootstrapWitness>) -> Self {
        let mut dedup = HashSet::new();
        let mut witnesses = Vec::new();
        for wit in wits {
            if dedup.insert(wit.clone()) {
                witnesses.push(wit);
            }
        }

        Self {
            witnesses,
            dedup,
        }
    }
}

impl NoneOrEmpty for BootstrapWitnesses {
    fn is_none_or_empty(&self) -> bool {
        self.witnesses.is_empty()
    }
}

impl serde::Serialize for BootstrapWitnesses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let wits = self.get_vec_wits();
        wits.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for BootstrapWitnesses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let wits = <Vec<_> as serde::de::Deserialize>::deserialize(deserializer)?;

        Ok(Self::from_vec_wits(wits))
    }
}

impl JsonSchema for BootstrapWitnesses {
    fn is_referenceable() -> bool {
        Vec::<BootstrapWitness>::is_referenceable()
    }
    fn schema_name() -> String {
        String::from("BootstrapWitnesses")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<BootstrapWitness>::json_schema(gen)
    }
}
