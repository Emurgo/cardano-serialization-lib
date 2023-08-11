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
pub struct CommitteeHotKeyDeregistration {
    pub(crate) committee_cold_keyhash: Ed25519KeyHash,
}

impl_to_from!(CommitteeHotKeyDeregistration);

#[wasm_bindgen]
impl CommitteeHotKeyDeregistration {
    pub fn committee_cold_keyhash(&self) -> Ed25519KeyHash {
        self.committee_cold_keyhash.clone()
    }

    pub fn new(
        committee_cold_keyhash: &Ed25519KeyHash,
    ) -> Self {
        Self {
            committee_cold_keyhash: committee_cold_keyhash.clone(),
        }
    }
}
