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
pub struct VotingProposals(pub(crate) Vec<VotingProposal>);

impl_to_from!(VotingProposals);

impl VotingProposals {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> VotingProposal {
        self.0[index].clone()
    }

    pub fn add(&mut self, proposal: &VotingProposal) {
        self.0.push(proposal.clone());
    }
}
