use crate::*;

#[wasm_bindgen]
pub struct PublicKeys(Vec<PublicKey>);

#[wasm_bindgen]
impl PublicKeys {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PublicKeys {
        PublicKeys(vec![])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PublicKey {
        self.0[index].clone()
    }

    pub fn add(&mut self, key: &PublicKey) {
        self.0.push(key.clone());
    }
}