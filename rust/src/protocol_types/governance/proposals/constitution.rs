use crate::*;

#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
#[wasm_bindgen]
pub struct Constitution {
    pub(crate) anchor: Anchor,
    pub(crate) script_hash: Option<ScriptHash>,
}

impl_to_from!(Constitution);

#[wasm_bindgen]
impl Constitution {
    pub fn anchor(&self) -> Anchor {
        self.anchor.clone()
    }

    pub fn script_hash(&self) -> Option<ScriptHash> {
        self.script_hash.clone()
    }

    pub fn new(anchor: &Anchor) -> Self {
        Self {
            anchor: anchor.clone(),
            script_hash: None,
        }
    }

    pub fn new_with_script_hash(anchor: &Anchor, script_hash: &ScriptHash) -> Self {
        Self {
            anchor: anchor.clone(),
            script_hash: Some(script_hash.clone()),
        }
    }
}
