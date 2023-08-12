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
pub struct CommitteeHotKeyRegistration {
    pub(crate) committee_cold_key: StakeCredential,
    pub(crate) committee_hot_key: StakeCredential,
}

impl_to_from!(CommitteeHotKeyRegistration);

#[wasm_bindgen]
impl CommitteeHotKeyRegistration {
    pub fn committee_cold_key(&self) -> StakeCredential {
        self.committee_cold_key.clone()
    }

    pub fn committee_hot_key(&self) -> StakeCredential {
        self.committee_hot_key.clone()
    }

    pub fn new(
        committee_cold_key: &StakeCredential,
        committee_hot_key: &StakeCredential,
    ) -> Self {
        Self {
            committee_cold_key: committee_cold_key.clone(),
            committee_hot_key: committee_hot_key.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.committee_cold_key.has_script_hash()
    }
}
