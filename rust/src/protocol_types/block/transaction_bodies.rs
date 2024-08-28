use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct TransactionBodies(pub(crate) Vec<TransactionBody>);

impl_to_from!(TransactionBodies);

#[wasm_bindgen]
impl TransactionBodies {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionBody {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionBody) {
        self.0.push(elem.clone());
    }
}