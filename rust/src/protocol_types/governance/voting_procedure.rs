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
pub enum VoteKind {
    No = 0,
    Yes = 1,
    Abstain = 2,
}

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
pub struct VotingProcedure {
    pub(crate) vote: VoteKind,
    pub(crate) anchor: Anchor,
}

impl_to_from!(VotingProcedure);

#[wasm_bindgen]
impl VotingProcedure {
    pub fn new(vote: VoteKind, anchor: &Anchor) -> Self {
        Self {
            vote,
            anchor: anchor.clone(),
        }
    }

    pub fn vote(&self) -> VoteKind {
        self.vote.clone()
    }

    pub fn anchor(&self) -> Anchor {
        self.anchor.clone()
    }
}
