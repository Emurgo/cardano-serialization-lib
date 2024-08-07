use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq)]
/// Warning: This is experimental and may be removed or changed in the future.
pub struct FixedTransactionBodies(pub(crate) Vec<FixedTransactionBody>);

from_bytes!(FixedTransactionBodies);

#[wasm_bindgen]
impl FixedTransactionBodies {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> FixedTransactionBody {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &FixedTransactionBody) {
        self.0.push(elem.clone());
    }
}