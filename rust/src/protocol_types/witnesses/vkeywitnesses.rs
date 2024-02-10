use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Vkeywitnesses(pub(crate) Vec<Vkeywitness>);

impl_to_from!(Vkeywitnesses);

#[wasm_bindgen]
impl Vkeywitnesses {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &Vkeywitness) {
        self.0.push(elem.clone());
    }
}