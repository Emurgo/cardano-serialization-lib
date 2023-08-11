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
pub struct CommitteeHotKeyRegistration {
    pub(crate) committee_cold_keyhash: Ed25519KeyHash,
    pub(crate) committee_hot_keyhash: Ed25519KeyHash,
}

impl_to_from!(CommitteeHotKeyRegistration);

#[wasm_bindgen]
impl CommitteeHotKeyRegistration {
    pub fn committee_cold_keyhash(&self) -> Ed25519KeyHash {
        self.committee_cold_keyhash.clone()
    }

    pub fn committee_hot_keyhash(&self) -> Ed25519KeyHash {
        self.committee_hot_keyhash.clone()
    }

    pub fn new(
        committee_cold_keyhash: &Ed25519KeyHash,
        committee_hot_keyhash: &Ed25519KeyHash,
    ) -> Self {
        Self {
            committee_cold_keyhash: committee_cold_keyhash.clone(),
            committee_hot_keyhash: committee_hot_keyhash.clone(),
        }
    }
}
