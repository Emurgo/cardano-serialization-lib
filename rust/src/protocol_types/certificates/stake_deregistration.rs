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
pub struct StakeDeregistration {
    pub(crate) stake_credential: Credential,
    pub(crate) coin: Option<Coin>,
}

impl_to_from!(StakeDeregistration);

#[wasm_bindgen]
impl StakeDeregistration {
    pub fn stake_credential(&self) -> Credential {
        self.stake_credential.clone()
    }

    pub fn coin(&self) -> Option<Coin> {
        self.coin.clone()
    }

    pub fn new(stake_credential: &Credential) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            coin: None,
        }
    }

    pub fn new_with_coin(stake_credential: &Credential, coin: &Coin) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            coin: Some(coin.clone()),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.stake_credential.has_script_hash()
    }
}
