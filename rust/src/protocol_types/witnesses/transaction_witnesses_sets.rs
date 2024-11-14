use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct TransactionWitnessSets(pub(crate) Vec<TransactionWitnessSet>);

impl_to_from!(TransactionWitnessSets);

#[wasm_bindgen]
impl TransactionWitnessSets {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionWitnessSet {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionWitnessSet) {
        self.0.push(elem.clone());
    }
}