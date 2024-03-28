use crate::*;
use std::collections::BTreeMap;

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
pub struct TreasuryWithdrawals(pub(crate) BTreeMap<RewardAddress, Coin>);

to_from_json!(TreasuryWithdrawals);

#[wasm_bindgen]
impl TreasuryWithdrawals {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, key: &RewardAddress) -> Option<Coin> {
        self.0.get(key).cloned()
    }

    pub fn insert(&mut self, key: &RewardAddress, value: &Coin) {
        self.0.insert(key.clone(), value.clone());
    }

    pub fn keys(&self) -> RewardAddresses {
        RewardAddresses(self.0.keys().cloned().collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
