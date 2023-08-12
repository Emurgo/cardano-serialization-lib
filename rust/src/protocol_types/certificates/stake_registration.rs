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
pub struct StakeRegistration {
    pub(crate) stake_credential: StakeCredential,
    pub(crate) coin: Option<Coin>
}

impl_to_from!(StakeRegistration);

#[wasm_bindgen]
impl StakeRegistration {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
    }

    pub fn coin(&self) -> Option<Coin> {
        self.coin.clone()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            coin: None
        }
    }

    pub fn new_with_coin(stake_credential: &StakeCredential, coin: &Coin) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            coin: Some(coin.clone())
        }
    }
}
