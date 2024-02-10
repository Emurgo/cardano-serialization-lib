use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Vkey(pub(crate) PublicKey);

impl_to_from!(Vkey);

#[wasm_bindgen]
impl Vkey {
    pub fn new(pk: &PublicKey) -> Self {
        Self(pk.clone())
    }

    pub fn public_key(&self) -> PublicKey {
        self.0.clone()
    }
}