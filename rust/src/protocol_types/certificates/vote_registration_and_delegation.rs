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
pub struct VoteRegistrationAndDelegation {
    pub(crate) stake_credential: Credential,
    pub(crate) drep: DRep,
    pub(crate) coin: Coin,
}

impl_to_from!(VoteRegistrationAndDelegation);

#[wasm_bindgen]
impl VoteRegistrationAndDelegation {
    pub fn stake_credential(&self) -> Credential {
        self.stake_credential.clone()
    }

    pub fn drep(&self) -> DRep {
        self.drep.clone()
    }

    pub fn coin(&self) -> Coin {
        self.coin.clone()
    }

    pub fn new(stake_credential: &Credential, drep: &DRep, coin: &Coin) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            drep: drep.clone(),
            coin: coin.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.stake_credential.has_script_hash()
    }
}
