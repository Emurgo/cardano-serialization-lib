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
pub struct NewConstitutionAction {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
    pub(crate) constitution: Constitution,
}

impl_to_from!(NewConstitutionAction);

#[wasm_bindgen]
impl NewConstitutionAction {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn constitution(&self) -> Constitution {
        self.constitution.clone()
    }

    pub fn new(constitution: &Constitution) -> Self {
        Self {
            gov_action_id: None,
            constitution: constitution.clone(),
        }
    }

    pub fn new_with_action_id(
        gov_action_id: &GovernanceActionId,
        constitution: &Constitution,
    ) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            constitution: constitution.clone(),
        }
    }

    pub fn has_script_hash(&self) -> bool {
        self.constitution.script_hash.is_some()
    }
}
