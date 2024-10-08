#[derive(Eq, Hash, PartialEq, Clone, Debug, FromPrimitive, ToPrimitive)]
pub(crate) enum CertificateIndexNames {
    StakeRegistrationLegacy = 0,
    StakeDeregistrationLegacy = 1,
    StakeDelegation = 2,
    PoolRegistration = 3,
    PoolRetirement = 4,
    GenesisKeyDelegation = 5,
    MoveInstantaneousRewardsCert = 6,
    StakeRegistrationConway = 7,
    StakeDeregistrationConway = 8,
    VoteDelegation = 9,
    StakeAndVoteDelegation = 10,
    StakeRegistrationAndDelegation = 11,
    VoteRegistrationAndDelegation = 12,
    StakeVoteRegistrationAndDelegation = 13,
    CommitteeHotAuth = 14,
    CommitteeColdResign = 15,
    DRepRegistration = 16,
    DRepDeregistration = 17,
    DRepUpdate = 18,
}
