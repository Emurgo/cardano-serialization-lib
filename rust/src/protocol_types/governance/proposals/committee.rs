use crate::*;
use schemars::gen::SchemaGenerator;
use schemars::schema::Schema;
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
struct CommitteeMember {
    stake_credential: Credential,
    term_limit: Epoch,
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
struct CommitteeJsonStruct {
    members: Vec<CommitteeMember>,
    quorum_threshold: UnitInterval,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[wasm_bindgen]
pub struct Committee {
    pub(crate) members: BTreeMap<Credential, Epoch>,
    pub(crate) quorum_threshold: UnitInterval,
}

impl_to_from!(Committee);

#[wasm_bindgen]
impl Committee {
    pub fn new(quorum_threshold: &UnitInterval) -> Self {
        Self {
            members: BTreeMap::new(),
            quorum_threshold: quorum_threshold.clone(),
        }
    }

    pub fn members_keys(&self) -> CredentialsSet {
        CredentialsSet::from_iter(self.members.keys().cloned())
    }

    pub fn quorum_threshold(&self) -> UnitInterval {
        self.quorum_threshold.clone()
    }

    pub fn add_member(&mut self, committee_cold_credential: &Credential, epoch: Epoch) {
        self.members
            .insert(committee_cold_credential.clone(), epoch);
    }

    pub fn get_member_epoch(&self, committee_cold_credential: &Credential) -> Option<Epoch> {
        self.members.get(committee_cold_credential).cloned()
    }
}

impl JsonSchema for Committee {
    fn is_referenceable() -> bool {
        CommitteeJsonStruct::is_referenceable()
    }

    fn schema_name() -> String {
        "Committee".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        CommitteeJsonStruct::json_schema(gen)
    }
}

impl serde::ser::Serialize for Committee {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let committee = CommitteeJsonStruct {
            members: self
                .members
                .iter()
                .map(|(k, v)| CommitteeMember {
                    stake_credential: k.clone(),
                    term_limit: v.clone(),
                })
                .collect(),
            quorum_threshold: self.quorum_threshold.clone(),
        };

        committee.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Committee {
    fn deserialize<D>(deserializer: D) -> Result<Committee, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let committee_json: CommitteeJsonStruct =
            serde::de::Deserialize::deserialize(deserializer)?;
        let mut committee = Committee::new(&committee_json.quorum_threshold);
        let mut members = BTreeMap::new();
        for member in committee_json.members {
            members.insert(member.stake_credential, member.term_limit);
        }
        committee.members = members;
        Ok(committee)
    }
}
