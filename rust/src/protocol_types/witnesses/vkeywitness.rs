use crate::*;

#[wasm_bindgen]
#[derive(Clone, Hash, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Vkeywitness {
    pub(crate) vkey: Vkey,
    pub(crate) signature: Ed25519Signature,
}

impl_to_from!(Vkeywitness);

#[wasm_bindgen]
impl Vkeywitness {
    pub fn new(vkey: &Vkey, signature: &Ed25519Signature) -> Self {
        Self {
            vkey: vkey.clone(),
            signature: signature.clone(),
        }
    }

    pub fn vkey(&self) -> Vkey {
        self.vkey.clone()
    }

    pub fn signature(&self) -> Ed25519Signature {
        self.signature.clone()
    }
}