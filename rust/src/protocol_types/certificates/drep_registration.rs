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
pub struct DrepRegistration {
    pub(crate) voting_credential: Credential,
    pub(crate) coin: Coin,
    pub(crate) anchor: Option<Anchor>,
}

impl_to_from!(DrepRegistration);

#[wasm_bindgen]
impl DrepRegistration {
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

    pub fn new_with_anchor(
        voting_credential: &Credential,
        coin: &Coin,
        anchor: &Anchor,
    ) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            coin: coin.clone(),
            anchor: Some(anchor.clone()),
        }
    }
}
