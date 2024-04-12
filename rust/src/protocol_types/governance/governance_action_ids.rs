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
pub struct GovernanceActionIds(pub(crate) Vec<GovernanceActionId>);

to_from_json!(GovernanceActionIds);

#[wasm_bindgen]
impl GovernanceActionIds {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, governance_action_id: &GovernanceActionId) {
        self.0.push(governance_action_id.clone());
    }

    pub fn get(&self, index: usize) -> Option<GovernanceActionId> {
        self.0.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
