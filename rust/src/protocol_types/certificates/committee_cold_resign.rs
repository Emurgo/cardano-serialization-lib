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
pub struct CommitteeColdResign {
    pub(crate) committee_cold_key: Credential,
    pub(crate) anchor: Option<Anchor>,
}

impl_to_from!(CommitteeColdResign);

#[wasm_bindgen]
impl CommitteeColdResign {
    pub fn committee_cold_key(&self) -> Credential {
        self.committee_cold_key.clone()
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.anchor.clone()
    }

    pub fn new(committee_cold_key: &Credential) -> Self {
        Self {
            committee_cold_key: committee_cold_key.clone(),
            anchor: None,
        }
    }

    pub fn new_with_anchor(committee_cold_key: &Credential, anchor: &Anchor) -> Self {
        Self {
            committee_cold_key: committee_cold_key.clone(),
            anchor: Some(anchor.clone()),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.committee_cold_key.has_script_hash()
    }
}
