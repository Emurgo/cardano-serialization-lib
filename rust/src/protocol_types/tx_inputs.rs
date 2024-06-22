use crate::*;
use std::slice::Iter;
use std::vec::IntoIter;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct TransactionInputs {
    pub(crate) inputs: Vec<TransactionInput>,
    pub(crate) dedup: BTreeSet<TransactionInput>,
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
        }
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.inputs[index].clone()
    }

    /// Add a new `TransactionInput` to the set.
    /// Returns `true` if the element was not already present in the set.
    /// Note that the `TransactionInput` is added to the set only if it is not already present.
    pub fn add(&mut self, elem: &TransactionInput) -> bool {
        if self.dedup.insert(elem.clone()) {
            self.inputs.push(elem.clone());
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, elem: &TransactionInput) -> bool {
        self.dedup.contains(elem)
    }

    pub(crate) fn add_move(&mut self, elem: TransactionInput) {
        if self.dedup.insert(elem.clone()) {
            self.inputs.push(elem);
        }
    }

    pub(crate) fn from_vec(inputs_vec: Vec<TransactionInput>) -> Self {
        let mut inputs = Self::new();
        for input in inputs_vec {
            inputs.add_move(input);
        }
        inputs
    }

    pub fn to_option(&self) -> Option<TransactionInputs> {
        if self.len() > 0 {
            Some(self.clone())
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a TransactionInputs {
    type Item = &'a TransactionInput;
    type IntoIter = Iter<'a, TransactionInput>;

    fn into_iter(self) -> Self::IntoIter {
        self.inputs.iter()
    }
}

impl IntoIterator for TransactionInputs {
    type Item = TransactionInput;
    type IntoIter = IntoIter<TransactionInput>;

    fn into_iter(self) -> Self::IntoIter {
        self.inputs.into_iter()
    }
}

impl serde::Serialize for TransactionInputs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.inputs.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for TransactionInputs {
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
