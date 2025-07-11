use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::slice;
use std::iter::Map;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::sync::Arc;
use itertools::Itertools;
use schemars::JsonSchema;
use crate::*;

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
)]
pub struct VotingProposals {
    proposals: Vec<Arc<VotingProposal>>,
    dedup: HashSet<Arc<VotingProposal>>,
    cbor_set_type: CborSetType,
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
            dedup: HashSet::new(),
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub(crate) fn new_from_prepared_fields(
        proposals: Vec<Arc<VotingProposal>>,
        dedup: HashSet<Arc<VotingProposal>>,
    ) -> Self {
        Self {
            proposals,
            dedup,
            cbor_set_type: CborSetType::Tagged,
        }
    }

    pub fn len(&self) -> usize {
        self.proposals.len()
    }

    pub fn get(&self, index: usize) -> VotingProposal {
        self.proposals[index].deref().clone()
    }

    /// Add a new `VotingProposal` to the set.
    /// Returns `true` if the element was not already present in the set.
    pub fn add(&mut self, proposal: &VotingProposal) -> bool {
        let proposal_rc = Arc::new(proposal.clone());
        if self.dedup.insert(proposal_rc.clone()) {
            self.proposals.push(proposal_rc.clone());
            true
        } else {
            false
        }
    }

    pub fn contains(&self, elem: &VotingProposal) -> bool {
        self.dedup.contains(elem)
    }

    pub fn to_option(&self) -> Option<VotingProposals> {
        if !self.proposals.is_empty() {
            Some(self.clone())
        } else {
            None
        }
    }

    pub(crate) fn from_vec(proposal_vec: Vec<VotingProposal>) -> Self {
        let mut dedup = HashSet::new();
        let mut proposals = Vec::new();
        for proposal in proposal_vec {
            let proposal_rc = Arc::new(proposal.clone());
            if dedup.insert(proposal_rc.clone()) {
                proposals.push(proposal_rc);
            }
        }
        Self::new_from_prepared_fields(proposals, dedup)
    }

    pub(crate) fn get_set_type(&self) -> CborSetType {
        self.cbor_set_type.clone()
    }

    pub(crate) fn set_set_type(&mut self, cbor_set_type: CborSetType) {
        self.cbor_set_type = cbor_set_type;
    }
}

impl<'a> IntoIterator for &'a VotingProposals {
    type Item = &'a VotingProposal;
    type IntoIter = Map<
        slice::Iter<'a, Arc<VotingProposal>>,
        fn(&'a Arc<VotingProposal>) -> &'a VotingProposal,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.proposals.iter().map(|rc| rc.as_ref())
    }
}

impl PartialEq for VotingProposals {
    fn eq(&self, other: &Self) -> bool {
        self.proposals == other.proposals
    }
}

impl Eq for VotingProposals {}

impl PartialOrd for VotingProposals {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.proposals.partial_cmp(&other.proposals)
    }
}

impl Ord for VotingProposals {
    fn cmp(&self, other: &Self) -> Ordering {
        self.proposals.cmp(&other.proposals)
    }
}

impl Hash for VotingProposals {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.proposals.hash(state);
    }
}

impl serde::Serialize for VotingProposals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.proposals
            .iter()
            .map(|x| x.deref())
            .collect_vec()
            .serialize(serializer)
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