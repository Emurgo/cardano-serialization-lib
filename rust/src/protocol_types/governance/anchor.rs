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
pub struct Anchor {
    pub(crate) anchor_url: URL,
    pub(crate) anchor_data_hash: AnchorDataHash,
}

impl_to_from!(Anchor);

#[wasm_bindgen]
impl Anchor {
    pub fn anchor_url(&self) -> URL {
        self.anchor_url.clone()
    }

    pub fn anchor_data_hash(&self) -> AnchorDataHash {
        self.anchor_data_hash.clone()
    }

    pub fn new(anchor_url: &URL, anchor_data_hash: &AnchorDataHash) -> Self {
        Self {
            anchor_url: anchor_url.clone(),
            anchor_data_hash: anchor_data_hash.clone(),
        }
    }
}
