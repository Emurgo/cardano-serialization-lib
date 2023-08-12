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
pub struct CommitteeHotKeyDeregistration {
    pub(crate) committee_cold_key: StakeCredential,
}

impl_to_from!(CommitteeHotKeyDeregistration);

#[wasm_bindgen]
impl CommitteeHotKeyDeregistration {
    pub fn committee_cold_key(&self) -> StakeCredential {
        self.committee_cold_key.clone()
    }

    pub fn new(
        committee_cold_key: &StakeCredential,
    ) -> Self {
        Self {
            committee_cold_key: committee_cold_key.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.committee_cold_key.has_script_hash()
    }
}
