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
pub struct NewCommitteeProposal {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
    pub(crate) committee: Committee,
    pub(crate) members_to_remove: BTreeSet<Credential>,
}

impl_to_from!(NewCommitteeProposal);

#[wasm_bindgen]
impl NewCommitteeProposal {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn committee(&self) -> Committee {
        self.committee.clone()
    }

    pub fn members_to_remove(&self) -> StakeCredentials {
        StakeCredentials(self.members_to_remove.iter().cloned().collect())
    }

    pub fn new(committee: &Committee, members_to_remove: &StakeCredentials) -> Self {
        let members_to_remove = members_to_remove.0.iter().cloned().collect();
        Self {
            gov_action_id: None,
            committee: committee.clone(),
            members_to_remove,
        }
    }

    pub fn new_with_action_id(
        gov_action_id: &GovernanceActionId,
        committee: &Committee,
        members_to_remove: &StakeCredentials,
    ) -> Self {
        let members_to_remove = members_to_remove.0.iter().cloned().collect();
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            committee: committee.clone(),
            members_to_remove,
        }
    }
}
