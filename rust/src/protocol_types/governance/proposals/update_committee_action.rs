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
pub struct UpdateCommitteeAction {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
    pub(crate) committee: Committee,
    pub(crate) members_to_remove: CredentialsSet,
}

impl_to_from!(UpdateCommitteeAction);

#[wasm_bindgen]
impl UpdateCommitteeAction {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn committee(&self) -> Committee {
        self.committee.clone()
    }

    pub fn members_to_remove(&self) -> CredentialsSet {
        self.members_to_remove.clone()
    }

    pub fn new(committee: &Committee, members_to_remove: &CredentialsSet) -> Self {
        Self {
            gov_action_id: None,
            committee: committee.clone(),
            members_to_remove: members_to_remove.clone(),
        }
    }

    pub fn new_with_action_id(
        gov_action_id: &GovernanceActionId,
        committee: &Committee,
        members_to_remove: &CredentialsSet,
    ) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            committee: committee.clone(),
            members_to_remove: members_to_remove.clone(),
        }
    }
}
