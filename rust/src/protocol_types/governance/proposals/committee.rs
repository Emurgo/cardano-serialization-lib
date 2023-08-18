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
pub struct Committee {
    pub(crate) members: BTreeMap<StakeCredential, Epoch>,
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

    pub fn members_keys(&self) -> StakeCredentials {
        StakeCredentials(self.members.keys().cloned().collect())
    }

    pub fn quorum_threshold(&self) -> UnitInterval {
        self.quorum_threshold.clone()
    }

    pub fn add_member(&mut self, committee_cold_credential: &StakeCredential, epoch: Epoch) {
        self.members
            .insert(committee_cold_credential.clone(), epoch);
    }

    pub fn get_member_epoch(&self, committee_cold_credential: &StakeCredential) -> Option<Epoch> {
        self.members.get(committee_cold_credential).cloned()
    }
}
