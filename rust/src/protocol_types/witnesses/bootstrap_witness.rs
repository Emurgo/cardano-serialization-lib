use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct BootstrapWitness {
    pub(crate) vkey: Vkey,
    pub(crate) signature: Ed25519Signature,
    pub(crate) chain_code: Vec<u8>,
    pub(crate) attributes: Vec<u8>,
}

impl_to_from!(BootstrapWitness);

#[wasm_bindgen]
impl BootstrapWitness {
    pub fn vkey(&self) -> Vkey {
        self.vkey.clone()
    }

    pub fn signature(&self) -> Ed25519Signature {
        self.signature.clone()
    }

    pub fn chain_code(&self) -> Vec<u8> {
        self.chain_code.clone()
    }

    pub fn attributes(&self) -> Vec<u8> {
        self.attributes.clone()
    }

    pub fn new(
        vkey: &Vkey,
        signature: &Ed25519Signature,
        chain_code: Vec<u8>,
        attributes: Vec<u8>,
    ) -> Self {
        Self {
            vkey: vkey.clone(),
            signature: signature.clone(),
            chain_code: chain_code,
            attributes: attributes,
        }
    }
}