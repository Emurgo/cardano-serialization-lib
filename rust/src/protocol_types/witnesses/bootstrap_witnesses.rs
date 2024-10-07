use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::ops::Deref;
use std::slice;
use std::iter::Map;
use std::collections::HashSet;
use itertools::Itertools;
use schemars::JsonSchema;
use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BootstrapWitnesses {
    witnesses: Vec<Rc<BootstrapWitness>>,
    dedup: HashSet<Rc<BootstrapWitness>>,
    cbor_set_type: CborSetType,
}

impl_to_from!(BootstrapWitnesses);

impl NoneOrEmpty for BootstrapWitnesses {
    fn is_none_or_empty(&self) -> bool {
        self.witnesses.is_empty()
    }
}

#[wasm_bindgen]
impl BootstrapWitnesses {
    pub fn new() -> Self {
        Self {
            witnesses: Vec::new(),
            dedup: HashSet::new(),
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub(crate) fn new_from_prepared_fields(
        witnesses: Vec<Rc<BootstrapWitness>>,
        dedup: HashSet<Rc<BootstrapWitness>>,
    ) -> Self {
        Self {
            witnesses,
            dedup,
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub fn len(&self) -> usize {
        self.witnesses.len()
    }

    pub fn get(&self, index: usize) -> BootstrapWitness {
        self.witnesses[index].deref().clone()
    }

    /// Add a new `BootstrapWitness` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, witness: &BootstrapWitness) -> bool {
        let witness_rc = Rc::new(witness.clone());
        if self.dedup.insert(witness_rc.clone()) {
            self.witnesses.push(witness_rc);
            true
        } else {
            false
        }
    }

    pub(crate) fn from_vec(witnesses_vec: Vec<BootstrapWitness>) -> Self {
        let mut dedup = HashSet::new();
        let mut witnesses = Vec::new();
        for witness in witnesses_vec {
            let witness_rc = Rc::new(witness.clone());
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

    pub(crate) fn get_vec_wits(&self) -> &Vec<Rc<BootstrapWitness>> {
        &self.witnesses
    }

    #[allow(dead_code)]
    pub (crate) fn contains(&self, elem: &BootstrapWitness) -> bool {
        self.dedup.contains(elem)
    }
}

impl<'a> IntoIterator for &'a BootstrapWitnesses {
    type Item = &'a BootstrapWitness;
    type IntoIter = Map<
        slice::Iter<'a, Rc<BootstrapWitness>>,
        fn(&'a Rc<BootstrapWitness>) -> &'a BootstrapWitness,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.witnesses.iter().map(|rc| rc.as_ref())
    }
}

impl PartialEq for BootstrapWitnesses {
    fn eq(&self, other: &Self) -> bool {
        self.witnesses == other.witnesses
    }
}

impl Eq for BootstrapWitnesses {}

impl Hash for BootstrapWitnesses {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.witnesses.hash(state);
    }
}

impl serde::Serialize for BootstrapWitnesses {
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

impl<'de> serde::de::Deserialize<'de> for BootstrapWitnesses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let witnesses_vec = <Vec<BootstrapWitness> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_vec(witnesses_vec))
    }
}

impl JsonSchema for BootstrapWitnesses {
    fn schema_name() -> String {
        String::from("BootstrapWitnesses")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<BootstrapWitness>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<BootstrapWitness>::is_referenceable()
    }
}