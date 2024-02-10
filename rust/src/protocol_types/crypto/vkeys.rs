use crate::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Vkeys(pub(crate) Vec<Vkey>);

#[wasm_bindgen]
impl Vkeys {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vkey {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Vkey) {
        self.0.push(elem.clone());
    }
}