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
pub struct TreasuryWithdrawalsProposal {
    pub(crate) withdrawals: TreasuryWithdrawals,
}

impl_to_from!(TreasuryWithdrawalsProposal);

#[wasm_bindgen]
impl TreasuryWithdrawalsProposal {
    pub fn withdrawals(&self) -> TreasuryWithdrawals {
        self.withdrawals.clone()
    }

    pub fn new(withdrawals: TreasuryWithdrawals) -> Self {
        Self {
            withdrawals: withdrawals.clone(),
        }
    }
}
