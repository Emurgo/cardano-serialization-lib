use crate::*;
use itertools::Itertools;
use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::ops::Deref;
use std::rc::Rc;
use std::slice;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionInputs {
    pub(crate) inputs: Vec<Rc<TransactionInput>>,
    pub(crate) dedup: BTreeSet<Rc<TransactionInput>>,
    pub(crate) cbor_set_type: CborSetType,
}

impl_to_from!(TransactionInputs);

impl NoneOrEmpty for TransactionInputs {
    fn is_none_or_empty(&self) -> bool {
        self.inputs.is_empty()
    }
}

#[wasm_bindgen]
impl TransactionInputs {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            dedup: BTreeSet::new(),
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub(crate) fn new_from_prepared_fields(
        inputs: Vec<Rc<TransactionInput>>,
        dedup: BTreeSet<Rc<TransactionInput>>,
    ) -> Self {
        Self {
            inputs,
            dedup,
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.inputs[index].deref().clone()
    }

    /// Add a new `TransactionInput` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, input: &TransactionInput) -> bool {
        let input_rc = Rc::new(input.clone());
        if self.dedup.insert(input_rc.clone()) {
            self.inputs.push(input_rc.clone());
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, elem: &TransactionInput) -> bool {
        self.dedup.contains(elem)
    }

    pub fn to_option(&self) -> Option<TransactionInputs> {
        if !self.inputs.is_empty() {
            Some(self.clone())
        } else {
            None
        }
    }

    pub(crate) fn add_move(&mut self, input: TransactionInput) {
        let input_rc = Rc::new(input);
        if self.dedup.insert(input_rc.clone()) {
            self.inputs.push(input_rc);
        }
    }

    pub(crate) fn extend(&mut self, other: &TransactionInputs) {
        for input in &other.inputs {
            self.add(input);
        }
    }

    pub(crate) fn extend_move(&mut self, other: TransactionInputs) {
        for input in other.inputs {
            if self.dedup.insert(input.clone()) {
                self.inputs.push(input);
            }
        }
    }

    pub(crate) fn from_vec(inputs_vec: Vec<TransactionInput>) -> Self {
        let mut dedup = BTreeSet::new();
        let mut inputs = Vec::new();
        for input in inputs_vec {
            let input_rc = Rc::new(input.clone());
            if dedup.insert(input_rc.clone()) {
                inputs.push(input_rc);
            }
        }

        Self::new_from_prepared_fields(inputs, dedup)
    }

    pub(crate) fn get_set_type(&self) -> CborSetType {
        self.cbor_set_type.clone()
    }

    pub(crate) fn set_set_type(&mut self, cbor_set_type: CborSetType) {
        self.cbor_set_type = cbor_set_type;
    }
}

impl<'a> IntoIterator for &'a TransactionInputs {
    type Item = &'a TransactionInput;
    type IntoIter = Map<
        slice::Iter<'a, Rc<TransactionInput>>,
        fn(&'a Rc<TransactionInput>) -> &'a TransactionInput,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inputs.iter().map(|rc| rc.as_ref())
    }
}

impl PartialEq for TransactionInputs {
    fn eq(&self, other: &Self) -> bool {
        self.inputs == other.inputs
    }
}

impl Eq for TransactionInputs {}

impl PartialOrd for TransactionInputs {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inputs.partial_cmp(&other.inputs)
    }
}

impl Ord for TransactionInputs {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inputs.cmp(&other.inputs)
    }
}

impl Hash for TransactionInputs {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inputs.hash(state);
    }
}

impl serde::Serialize for TransactionInputs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inputs
            .iter()
            .map(|x| x.deref())
            .collect_vec()
            .serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for TransactionInputs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let vec = <Vec<TransactionInput> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_vec(vec))
    }
}

impl JsonSchema for TransactionInputs {
    fn schema_name() -> String {
        String::from("TransactionInputs")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<TransactionInput>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<TransactionInput>::is_referenceable()
    }
}
