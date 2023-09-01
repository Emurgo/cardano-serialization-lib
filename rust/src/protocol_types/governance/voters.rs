use crate::*;

#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
#[wasm_bindgen]
pub struct Voters(pub(crate) Vec<Voter>);

to_from_json!(Voters);

#[wasm_bindgen]
impl Voters {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, voter: &Voter) {
        self.0.push(voter.clone());
    }

    pub fn get(&self, index: usize) -> Option<Voter> {
        self.0.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}