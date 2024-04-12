use crate::*;

#[wasm_bindgen]
#[derive(
    Copy,
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
pub enum RedeemerTagKind {
    Spend,
    Mint,
    Cert,
    Reward,
    Vote,
    VotingProposal,
}

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
pub struct RedeemerTag(pub(crate) RedeemerTagKind);

impl_to_from!(RedeemerTag);

#[wasm_bindgen]
impl RedeemerTag {
    pub fn new_spend() -> Self {
        Self(RedeemerTagKind::Spend)
    }

    pub fn new_mint() -> Self {
        Self(RedeemerTagKind::Mint)
    }

    pub fn new_cert() -> Self {
        Self(RedeemerTagKind::Cert)
    }

    pub fn new_reward() -> Self {
        Self(RedeemerTagKind::Reward)
    }

    pub fn new_vote() -> Self {
        Self(RedeemerTagKind::Vote)
    }

    pub fn new_voting_proposal() -> Self {
        Self(RedeemerTagKind::VotingProposal)
    }

    pub fn kind(&self) -> RedeemerTagKind {
        self.0
    }
}
