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
pub struct StakeAndVoteDelegation {
    pub(crate) stake_credential: StakeCredential,
    pub(crate) pool_keyhash: Ed25519KeyHash,
    pub(crate) drep: DRep,
}

impl_to_from!(StakeAndVoteDelegation);

#[wasm_bindgen]
impl StakeAndVoteDelegation {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
    }

    pub fn pool_keyhash(&self) -> Ed25519KeyHash {
        self.pool_keyhash.clone()
    }

    pub fn drep(&self) -> DRep {
        self.drep.clone()
    }

    pub fn new(
        stake_credential: &StakeCredential,
        pool_keyhash: &Ed25519KeyHash,
        drep: &DRep,
    ) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            pool_keyhash: pool_keyhash.clone(),
            drep: drep.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.stake_credential.has_script_hash()
    }
}
