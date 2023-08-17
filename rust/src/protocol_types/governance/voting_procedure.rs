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
    pub(crate) anchor: Option<Anchor>,
}

impl_to_from!(VotingProcedure);

#[wasm_bindgen]
impl VotingProcedure {
    pub fn new(vote: &VoteKind) -> Self {
        Self {
            vote: vote.clone(),
            anchor: None,
        }
    }

    pub fn new_with_anchor(vote: &VoteKind, anchor: &Anchor) -> Self {
        Self {
            vote: vote.clone(),
            anchor: Some(anchor.clone()),
        }
    }

    pub fn vote(&self) -> VoteKind {
        self.vote.clone()
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.anchor.clone()
    }
}
