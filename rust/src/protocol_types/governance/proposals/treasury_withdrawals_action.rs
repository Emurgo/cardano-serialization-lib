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
pub struct TreasuryWithdrawalsAction {
    pub(crate) withdrawals: TreasuryWithdrawals,
    pub(crate) policy_hash: Option<ScriptHash>,
}

impl_to_from!(TreasuryWithdrawalsAction);

#[wasm_bindgen]
impl TreasuryWithdrawalsAction {
    pub fn withdrawals(&self) -> TreasuryWithdrawals {
        self.withdrawals.clone()
    }

    pub fn policy_hash(&self) -> Option<ScriptHash> {
        self.policy_hash.clone()
    }

    pub fn new(withdrawals: &TreasuryWithdrawals) -> Self {
        Self {
            withdrawals: withdrawals.clone(),
            policy_hash: None,
        }
    }

    pub fn new_with_policy_hash(
        withdrawals: &TreasuryWithdrawals,
        policy_hash: &ScriptHash,
    ) -> Self {
        Self {
            withdrawals: withdrawals.clone(),
            policy_hash: Some(policy_hash.clone()),
        }
    }
}
