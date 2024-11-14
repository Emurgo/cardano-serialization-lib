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
pub struct DRepRegistration {
    pub(crate) voting_credential: Credential,
    pub(crate) coin: Coin,
    pub(crate) anchor: Option<Anchor>,
}

impl_to_from!(DRepRegistration);

#[wasm_bindgen]
impl DRepRegistration {
    pub fn voting_credential(&self) -> Credential {
        self.voting_credential.clone()
    }

    pub fn coin(&self) -> Coin {
        self.coin.clone()
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.anchor.clone()
    }

    pub fn new(voting_credential: &Credential, coin: &Coin) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            coin: coin.clone(),
            anchor: None,
        }
    }

    pub fn new_with_anchor(voting_credential: &Credential, coin: &Coin, anchor: &Anchor) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            coin: coin.clone(),
            anchor: Some(anchor.clone()),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.voting_credential.has_script_hash()
    }
}
