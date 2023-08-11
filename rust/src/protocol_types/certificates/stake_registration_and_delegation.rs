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
pub struct StakeRegistrationAndDelegation {
    pub(crate) stake_credential: StakeCredential,
    pub(crate) pool_keyhash: Ed25519KeyHash,
    pub(crate) coin: Coin,
}

impl_to_from!(StakeRegistrationAndDelegation);

#[wasm_bindgen]
impl StakeRegistrationAndDelegation {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
    }

    pub fn pool_keyhash(&self) -> Ed25519KeyHash {
        self.pool_keyhash.clone()
    }

    pub fn coin(&self) -> Coin {
        self.coin.clone()
    }

    pub fn new(
        stake_credential: &StakeCredential,
        pool_keyhash: &Ed25519KeyHash,
        coin: &Coin,
    ) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            pool_keyhash: pool_keyhash.clone(),
            coin: coin.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.stake_credential.has_script_hash()
    }
}
