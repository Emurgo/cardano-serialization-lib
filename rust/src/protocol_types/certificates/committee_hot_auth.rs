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
    pub(crate) committee_cold_credential: Credential,
    pub(crate) committee_hot_credential: Credential,
}

impl_to_from!(CommitteeHotAuth);

#[wasm_bindgen]
impl CommitteeHotAuth {
    pub fn committee_cold_credential(&self) -> Credential {
        self.committee_cold_credential.clone()
    }

    pub fn committee_hot_credential(&self) -> Credential {
        self.committee_hot_credential.clone()
    }

    pub fn new(committee_cold_credential: &Credential, committee_hot_credential: &Credential) -> Self {
        Self {
            committee_cold_credential: committee_cold_credential.clone(),
            committee_hot_credential: committee_hot_credential.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.committee_cold_credential.has_script_hash()
    }
}
