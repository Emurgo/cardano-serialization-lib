use crate::*;

#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
#[wasm_bindgen]
pub struct NoConfidenceProposal {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
}

impl_to_from!(NoConfidenceProposal);

#[wasm_bindgen]
impl NoConfidenceProposal {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn new() -> Self {
        Self {
            gov_action_id: None,
        }
    }

    pub fn new_with_action_id(gov_action_id: &GovernanceActionId) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
        }
    }
}
