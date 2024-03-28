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
pub struct VotingProposal {
    pub(crate) governance_action: GovernanceAction,
    pub(crate) anchor: Anchor,
    pub(crate) reward_account: RewardAddress,
    pub(crate) deposit: Coin,
}

impl_to_from!(VotingProposal);

#[wasm_bindgen]
impl VotingProposal {
    pub fn governance_action(&self) -> GovernanceAction {
        self.governance_action.clone()
    }

    pub fn anchor(&self) -> Anchor {
        self.anchor.clone()
    }

    pub fn reward_account(&self) -> RewardAddress {
        self.reward_account.clone()
    }

    pub fn deposit(&self) -> Coin {
        self.deposit.clone()
    }

    pub fn new(
        governance_action: &GovernanceAction,
        anchor: &Anchor,
        reward_account: &RewardAddress,
        deposit: &Coin,
    ) -> Self {
        Self {
            governance_action: governance_action.clone(),
            anchor: anchor.clone(),
            reward_account: reward_account.clone(),
            deposit: deposit.clone(),
        }
    }

    pub(crate) fn has_script_hash(&self) -> bool {
        match self.governance_action.0 {
            GovernanceActionEnum::ParameterChangeAction(ref action) => action.has_script_hash(),
            GovernanceActionEnum::TreasuryWithdrawalsAction(ref action) => action.has_script_hash(),
            _ => false,
        }
    }
}
