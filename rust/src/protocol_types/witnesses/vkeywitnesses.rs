use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::slice;
use std::iter::Map;
use std::collections::HashSet;
use std::sync::Arc;
use itertools::Itertools;
use schemars::JsonSchema;
use crate::*;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
)]
pub struct Vkeywitnesses {
    witnesses: Vec<Arc<Vkeywitness>>,
    dedup: HashSet<Arc<Vkeywitness>>,
    cbor_set_type: CborSetType,
    force_original_cbor_set_type: bool,
}

impl_to_from!(Vkeywitnesses);

impl NoneOrEmpty for Vkeywitnesses {
    fn is_none_or_empty(&self) -> bool {
        self.witnesses.is_empty()
    }
}

#[wasm_bindgen]
impl Vkeywitnesses {
    pub fn new() -> Self {
        Self {
            witnesses: Vec::new(),
            dedup: HashSet::new(),
            cbor_set_type: CborSetType::Tagged,
            force_original_cbor_set_type: false,
        }
    }

    pub(crate) fn new_from_prepared_fields(
        witnesses: Vec<Arc<Vkeywitness>>,
        dedup: HashSet<Arc<Vkeywitness>>,
    ) -> Self {
        Self {
            witnesses,
            dedup,
            cbor_set_type: CborSetType::Tagged,
            force_original_cbor_set_type: false,
        }
    }

    pub fn len(&self) -> usize {
        self.witnesses.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.witnesses[index].deref().clone()
    }

    /// Add a new `Vkeywitness` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, witness: &Vkeywitness) -> bool {
        let witness_rc = Arc::new(witness.clone());
        if self.dedup.insert(witness_rc.clone()) {
            self.witnesses.push(witness_rc.clone());
            true
        } else {
            false
        }
    }

    pub(crate) fn add_move(&mut self, witness: Vkeywitness) {
        let witness_rc = Arc::new(witness);
        if self.dedup.insert(witness_rc.clone()) {
            self.witnesses.push(witness_rc);
        }
    }

    pub(crate) fn from_vec(vec: Vec<Vkeywitness>) -> Self {
        let mut dedup = HashSet::new();
        let mut witnesses = Vec::new();
        for witness in vec {
            let witness_rc = Arc::new(witness.clone());
            if dedup.insert(witness_rc.clone()) {
                witnesses.push(witness_rc);
            }
        }
        Self::new_from_prepared_fields(witnesses, dedup)
    }

    pub(crate) fn get_set_type(&self) -> CborSetType {
        self.cbor_set_type.clone()
    }

    pub(crate) fn set_set_type(&mut self, cbor_set_type: CborSetType) {
        self.cbor_set_type = cbor_set_type;
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, witness: &Vkeywitness) -> bool {
        self.dedup.contains(witness)
    }

    pub(crate) fn set_force_original_cbor_set_type(&mut self, force_original_cbor_set_type: bool) {
        self.force_original_cbor_set_type = force_original_cbor_set_type;
    }

    pub(crate) fn force_original_cbor_set_type(&self) -> bool {
        self.force_original_cbor_set_type
    }
}

impl<'a> IntoIterator for &'a Vkeywitnesses {
    type Item = &'a Vkeywitness;
    type IntoIter = Map<
        slice::Iter<'a, Arc<Vkeywitness>>,
        fn(&'a Arc<Vkeywitness>) -> &'a Vkeywitness,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.witnesses.iter().map(|rc| rc.as_ref())
    }
}

impl PartialEq for Vkeywitnesses {
    fn eq(&self, other: &Self) -> bool {
        self.witnesses == other.witnesses
    }
}

impl Eq for Vkeywitnesses {}

impl Hash for Vkeywitnesses {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.witnesses.hash(state);
    }
}

impl serde::Serialize for Vkeywitnesses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.witnesses
            .iter()
            .map(|x| x.deref())
            .collect_vec()
            .serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Vkeywitnesses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = <Vec<Vkeywitness> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_vec(vec))
    }
}

impl JsonSchema for Vkeywitnesses {
    fn schema_name() -> String {
        String::from("Vkeywitnesses")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<Vkeywitness>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<Vkeywitness>::is_referenceable()
    }
}