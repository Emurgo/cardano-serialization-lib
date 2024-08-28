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
pub struct DRepDeregistration {
    pub(crate) voting_credential: Credential,
    pub(crate) coin: Coin,
}

impl_to_from!(DRepDeregistration);

#[wasm_bindgen]
impl DRepDeregistration {
    pub fn voting_credential(&self) -> Credential {
        self.voting_credential.clone()
    }

    pub fn coin(&self) -> Coin {
        self.coin.clone()
    }

    pub fn new(voting_credential: &Credential, coin: &Coin) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            coin: coin.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.voting_credential.has_script_hash()
    }
}
