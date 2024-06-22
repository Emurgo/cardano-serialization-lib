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
pub struct VotingProposals {
    pub(crate) proposals: Vec<VotingProposal>,
    pub(crate) dedup: BTreeSet<VotingProposal>,
}

impl_to_from!(VotingProposals);

impl NoneOrEmpty for VotingProposals {
    fn is_none_or_empty(&self) -> bool {
        self.proposals.is_empty()
    }
}

#[wasm_bindgen]
impl VotingProposals {
    pub fn new() -> Self {
        Self {
            proposals: Vec::new(),
            dedup: BTreeSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.proposals.len()
    }

    pub fn get(&self, index: usize) -> VotingProposal {
        self.proposals[index].clone()
    }

    /// Add a proposal to the set of proposals
    /// Returns true if the proposal was added, false if it was already present
    pub fn add(&mut self, proposal: &VotingProposal) -> bool {
        if self.dedup.insert(proposal.clone()) {
            self.proposals.push(proposal.clone());
            true
        } else {
            false
        }
    }

    pub(crate) fn add_move(&mut self, proposal: VotingProposal) {
        if self.dedup.insert(proposal.clone()) {
            self.proposals.push(proposal);
        }
    }

    pub(crate) fn from_vec(proposals: Vec<VotingProposal>) -> Self {
        let mut voting_proposals = VotingProposals::new();
        for proposal in proposals {
            voting_proposals.add_move(proposal);
        }
        voting_proposals
    }

    pub(crate) fn contains(&self, proposal: &VotingProposal) -> bool {
        self.dedup.contains(proposal)
    }
}
