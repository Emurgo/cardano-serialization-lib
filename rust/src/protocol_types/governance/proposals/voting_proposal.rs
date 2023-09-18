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
pub(crate) enum VotingProposalEnum {
    ParameterChangeProposal(ParameterChangeProposal),
    HardForkInitiationProposal(HardForkInitiationProposal),
    TreasuryWithdrawalsProposal(TreasuryWithdrawalsProposal),
    NoConfidenceProposal(NoConfidenceProposal),
    UpdateCommitteeProposal(UpdateCommitteeProposal),
    NewConstitutionProposal(NewConstitutionProposal),
    InfoProposal(InfoProposal),
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
pub enum VotingProposalKind {
    ParameterChangeProposal = 0,
    HardForkInitiationProposal = 1,
    TreasuryWithdrawalsProposal = 2,
    NoConfidenceProposal = 3,
    UpdateCommitteeProposal = 4,
    NewConstitutionProposal = 5,
    InfoProposal = 6,
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
pub struct VotingProposal(pub(crate) VotingProposalEnum);

impl_to_from!(VotingProposal);

#[wasm_bindgen]
impl VotingProposal {
    pub fn new_parameter_change_proposal(
        parameter_change_proposal: &ParameterChangeProposal,
    ) -> Self {
        Self(VotingProposalEnum::ParameterChangeProposal(
            parameter_change_proposal.clone(),
        ))
    }

    pub fn new_hard_fork_initiation_proposal(
        hard_fork_initiation_proposal: &HardForkInitiationProposal,
    ) -> Self {
        Self(VotingProposalEnum::HardForkInitiationProposal(
            hard_fork_initiation_proposal.clone(),
        ))
    }

    pub fn new_treasury_withdrawals_proposal(
        treasury_withdrawals_proposal: &TreasuryWithdrawalsProposal,
    ) -> Self {
        Self(VotingProposalEnum::TreasuryWithdrawalsProposal(
            treasury_withdrawals_proposal.clone(),
        ))
    }

    pub fn new_no_confidence_proposal(no_confidence_proposal: &NoConfidenceProposal) -> Self {
        Self(VotingProposalEnum::NoConfidenceProposal(
            no_confidence_proposal.clone(),
        ))
    }

    pub fn new_new_committee_proposal(new_committee_proposal: &UpdateCommitteeProposal) -> Self {
        Self(VotingProposalEnum::UpdateCommitteeProposal(
            new_committee_proposal.clone(),
        ))
    }

    pub fn new_new_constitution_proposal(
        new_constitution_proposal: &NewConstitutionProposal,
    ) -> Self {
        Self(VotingProposalEnum::NewConstitutionProposal(
            new_constitution_proposal.clone(),
        ))
    }

    pub fn new_info_proposal(info_proposal: &InfoProposal) -> Self {
        Self(VotingProposalEnum::InfoProposal(info_proposal.clone()))
    }

    pub fn kind(&self) -> VotingProposalKind {
        match &self.0 {
            VotingProposalEnum::ParameterChangeProposal(_) => {
                VotingProposalKind::ParameterChangeProposal
            }
            VotingProposalEnum::HardForkInitiationProposal(_) => {
                VotingProposalKind::HardForkInitiationProposal
            }
            VotingProposalEnum::TreasuryWithdrawalsProposal(_) => {
                VotingProposalKind::TreasuryWithdrawalsProposal
            }
            VotingProposalEnum::NoConfidenceProposal(_) => VotingProposalKind::NoConfidenceProposal,
            VotingProposalEnum::UpdateCommitteeProposal(_) => VotingProposalKind::UpdateCommitteeProposal,
            VotingProposalEnum::NewConstitutionProposal(_) => {
                VotingProposalKind::NewConstitutionProposal
            }
            VotingProposalEnum::InfoProposal(_) => VotingProposalKind::InfoProposal,
        }
    }

    pub fn as_parameter_change_proposal(&self) -> Option<ParameterChangeProposal> {
        match &self.0 {
            VotingProposalEnum::ParameterChangeProposal(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_hard_fork_initiation_proposal(&self) -> Option<HardForkInitiationProposal> {
        match &self.0 {
            VotingProposalEnum::HardForkInitiationProposal(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_treasury_withdrawals_proposal(&self) -> Option<TreasuryWithdrawalsProposal> {
        match &self.0 {
            VotingProposalEnum::TreasuryWithdrawalsProposal(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_no_confidence_proposal(&self) -> Option<NoConfidenceProposal> {
        match &self.0 {
            VotingProposalEnum::NoConfidenceProposal(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_new_committee_proposal(&self) -> Option<UpdateCommitteeProposal> {
        match &self.0 {
            VotingProposalEnum::UpdateCommitteeProposal(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_new_constitution_proposal(&self) -> Option<NewConstitutionProposal> {
        match &self.0 {
            VotingProposalEnum::NewConstitutionProposal(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn as_info_proposal(&self) -> Option<InfoProposal> {
        match &self.0 {
            VotingProposalEnum::InfoProposal(p) => Some(p.clone()),
            _ => None,
        }
    }
}
