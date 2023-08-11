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
pub struct DrepUpdate {
    pub(crate) voting_credential: StakeCredential,
    pub(crate) anchor: Option<Anchor>,
}

impl_to_from!(DrepUpdate);

#[wasm_bindgen]
impl DrepUpdate {
    pub fn voting_credential(&self) -> StakeCredential {
        self.voting_credential.clone()
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.anchor.clone()
    }

    pub fn new(voting_credential: &StakeCredential, anchor: Option<Anchor>) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            anchor: anchor.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.voting_credential.has_script_hash()
    }
}
