use crate::*;
use schemars::gen::SchemaGenerator;
use schemars::schema::Schema;
use serde::ser::SerializeSeq;
use std::collections::BTreeMap;
use std::vec::Vec;

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
struct VoterVotes {
    voter: Voter,
    votes: BTreeSet<Vote>,
}

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
struct Vote {
    action_id: GovernanceActionId,
    voting_procedure: VotingProcedure,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

    pub fn insert(
        &mut self,
        voter: &Voter,
        governance_action_id: &GovernanceActionId,
        voting_procedure: &VotingProcedure,
    ) {
        self.0
            .entry(voter.clone())
            .or_insert_with(BTreeMap::new)
            .insert(governance_action_id.clone(), voting_procedure.clone());
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

impl JsonSchema for VotingProcedures {
    fn is_referenceable() -> bool {
        Vec::<VoterVotes>::is_referenceable()
    }

    fn schema_name() -> String {
        "VotingProcedures".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Vec::<VoterVotes>::json_schema(gen)
    }
}

impl serde::ser::Serialize for VotingProcedures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (voter, votes) in &self.0 {
            let voter_votes = VoterVotes {
                voter: voter.clone(),
                votes: votes
                    .iter()
                    .map(|(action_id, voting_procedure)| Vote {
                        action_id: action_id.clone(),
                        voting_procedure: voting_procedure.clone(),
                    })
                    .collect(),
            };
            seq.serialize_element(&voter_votes)?;
        }
        seq.end()
    }
}

impl<'de> serde::de::Deserialize<'de> for VotingProcedures {
    fn deserialize<D>(deserializer: D) -> Result<VotingProcedures, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let all_votes: Vec<VoterVotes> = serde::de::Deserialize::deserialize(deserializer)?;
        let mut voting_procedures = VotingProcedures::new();
        for votes in all_votes {
            let mut voter_votes = BTreeMap::new();
            for vote in votes.votes {
                voter_votes.insert(vote.action_id, vote.voting_procedure);
            }
            voting_procedures.0.insert(votes.voter, voter_votes);
        }
        Ok(voting_procedures)
    }
}
