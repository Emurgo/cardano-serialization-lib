use crate::*;

#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
#[wasm_bindgen]
pub struct InfoProposal();

#[wasm_bindgen]
impl InfoProposal {
    pub fn new() -> Self {
        Self()
    }
}
