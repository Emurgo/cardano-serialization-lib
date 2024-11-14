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
pub struct DRepUpdate {
    pub(crate) voting_credential: Credential,
    pub(crate) anchor: Option<Anchor>,
}

impl_to_from!(DRepUpdate);

#[wasm_bindgen]
impl DRepUpdate {
    pub fn voting_credential(&self) -> Credential {
        self.voting_credential.clone()
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.anchor.clone()
    }

    pub fn new(voting_credential: &Credential) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            anchor: None,
        }
    }

    pub fn new_with_anchor(voting_credential: &Credential, anchor: &Anchor) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            anchor: Some(anchor.clone()),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.voting_credential.has_script_hash()
    }
}
