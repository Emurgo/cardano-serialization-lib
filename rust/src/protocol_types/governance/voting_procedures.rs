use crate::*;
use std::collections::BTreeMap;

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
pub struct VotingProcedures(
    pub(crate) BTreeMap<Voter, BTreeMap<GovernanceActionId, VotingProcedure>>,
);

impl_to_from!(VotingProcedures);

#[wasm_bindgen]
impl VotingProcedures {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(
        &self,
        voter: &Voter,
        governance_action_id: &GovernanceActionId,
    ) -> Option<VotingProcedure> {
        self.0
            .get(voter)
            .and_then(|v| v.get(governance_action_id))
            .cloned()
    }

    pub fn get_voters(&self) -> Voters {
        Voters(self.0.keys().cloned().collect())
    }

    pub fn get_governance_action_ids_by_voter(&self, voter: &Voter) -> GovernanceActionIds {
        GovernanceActionIds(
            self.0
                .get(voter)
                .map(|v| v.keys().cloned().collect())
                .unwrap_or_default(),
        )
    }
}
