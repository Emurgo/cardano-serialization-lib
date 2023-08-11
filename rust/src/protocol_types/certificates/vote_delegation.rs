use crate::*;

#[wasm_bindgen]
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
pub struct VoteDelegation {
    pub(crate) stake_credential: StakeCredential,
    pub(crate) drep: DRep,
}

impl_to_from!(VoteDelegation);

#[wasm_bindgen]
impl VoteDelegation {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
    }

    pub fn drep(&self) -> DRep {
        self.drep.clone()
    }

    pub fn new(stake_credential: &StakeCredential, drep: &DRep) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            drep: drep.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.stake_credential.has_script_hash()
    }
}
