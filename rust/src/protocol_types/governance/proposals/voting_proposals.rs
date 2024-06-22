use crate::*;

#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
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

    #[allow(dead_code)]
    pub(crate) fn contains(&self, proposal: &VotingProposal) -> bool {
        self.dedup.contains(proposal)
    }
}

impl serde::Serialize for VotingProposals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.proposals.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for VotingProposals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let vec = <Vec<_> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::from_vec(vec))
    }
}

impl JsonSchema for VotingProposals {
    fn schema_name() -> String {
        String::from("VotingProposals")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<VotingProposal>::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        Vec::<VotingProposal>::is_referenceable()
    }
}

