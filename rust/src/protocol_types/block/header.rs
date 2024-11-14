use crate::*;

#[wasm_bindgen]
#[derive(Clone, Eq, Debug, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Header {
    pub(crate) header_body: HeaderBody,
    pub(crate) body_signature: KESSignature,
}

impl_to_from!(Header);

#[wasm_bindgen]
impl Header {
    pub fn header_body(&self) -> HeaderBody {
        self.header_body.clone()
    }

    pub fn body_signature(&self) -> KESSignature {
        self.body_signature.clone()
    }

    pub fn new(header_body: &HeaderBody, body_signature: &KESSignature) -> Self {
        Self {
            header_body: header_body.clone(),
            body_signature: body_signature.clone(),
        }
    }
}