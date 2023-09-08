#[derive(Eq, Hash, PartialEq, Clone, Debug, FromPrimitive, ToPrimitive)]
pub(crate) enum VotingProposalIndexNames {
    ParameterChangeAction = 0,
    HardForkInitiationAction = 1,
    TreasuryWithdrawalsAction = 2,
    NoConfidenceAction = 3,
    NewCommitteeAction = 4,
    NewConstitutionAction = 5,
    InfoAction = 6,
}
