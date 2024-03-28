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
pub struct CommitteeHotAuth {
    pub(crate) committee_cold_key: Credential,
    pub(crate) committee_hot_key: Credential,
}

impl_to_from!(CommitteeHotAuth);

#[wasm_bindgen]
impl CommitteeHotAuth {
    pub fn committee_cold_key(&self) -> Credential {
        self.committee_cold_key.clone()
    }

    pub fn committee_hot_key(&self) -> Credential {
        self.committee_hot_key.clone()
    }

    pub fn new(committee_cold_key: &Credential, committee_hot_key: &Credential) -> Self {
        Self {
            committee_cold_key: committee_cold_key.clone(),
            committee_hot_key: committee_hot_key.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.committee_cold_key.has_script_hash()
    }
}
