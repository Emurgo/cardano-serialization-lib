use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CertificateKind {
    StakeRegistration,
    StakeDeregistration,
    StakeDelegation,
    PoolRegistration,
    PoolRetirement,
    GenesisKeyDelegation,
    MoveInstantaneousRewardsCert,
    CommitteeHotAuth,
    CommitteeColdResign,
    DrepDeregistration,
    DrepRegistration,
    DrepUpdate,
    StakeAndVoteDelegation,
    StakeRegistrationAndDelegation,
    StakeVoteRegistrationAndDelegation,
    VoteDelegation,
    VoteRegistrationAndDelegation,
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
pub enum CertificateEnum {
    StakeRegistration(StakeRegistration),
    StakeDeregistration(StakeDeregistration),
    StakeDelegation(StakeDelegation),
    PoolRegistration(PoolRegistration),
    PoolRetirement(PoolRetirement),
    GenesisKeyDelegation(GenesisKeyDelegation),
    MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert),
    CommitteeHotAuth(CommitteeHotAuth),
    CommitteeColdResign(CommitteeColdResign),
    DrepDeregistration(DrepDeregistration),
    DrepRegistration(DrepRegistration),
    DrepUpdate(DrepUpdate),
    StakeAndVoteDelegation(StakeAndVoteDelegation),
    StakeRegistrationAndDelegation(StakeRegistrationAndDelegation),
    StakeVoteRegistrationAndDelegation(StakeVoteRegistrationAndDelegation),
    VoteDelegation(VoteDelegation),
    VoteRegistrationAndDelegation(VoteRegistrationAndDelegation),
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
pub struct Certificate(pub(crate) CertificateEnum);

impl_to_from!(Certificate);

#[wasm_bindgen]
impl Certificate {
    pub fn new_stake_registration(stake_registration: &StakeRegistration) -> Self {
        Self(CertificateEnum::StakeRegistration(
            stake_registration.clone(),
        ))
    }

    pub fn new_stake_deregistration(stake_deregistration: &StakeDeregistration) -> Self {
        Self(CertificateEnum::StakeDeregistration(
            stake_deregistration.clone(),
        ))
    }

    pub fn new_stake_delegation(stake_delegation: &StakeDelegation) -> Self {
        Self(CertificateEnum::StakeDelegation(stake_delegation.clone()))
    }

    pub fn new_pool_registration(pool_registration: &PoolRegistration) -> Self {
        Self(CertificateEnum::PoolRegistration(pool_registration.clone()))
    }

    pub fn new_pool_retirement(pool_retirement: &PoolRetirement) -> Self {
        Self(CertificateEnum::PoolRetirement(pool_retirement.clone()))
    }

    pub fn new_genesis_key_delegation(genesis_key_delegation: &GenesisKeyDelegation) -> Self {
        Self(CertificateEnum::GenesisKeyDelegation(
            genesis_key_delegation.clone(),
        ))
    }

    pub fn new_move_instantaneous_rewards_cert(
        move_instantaneous_rewards_cert: &MoveInstantaneousRewardsCert,
    ) -> Self {
        Self(CertificateEnum::MoveInstantaneousRewardsCert(
            move_instantaneous_rewards_cert.clone(),
        ))
    }

    pub fn new_committee_hot_key_registration(
        committee_hot_key_registration: &CommitteeHotAuth,
    ) -> Self {
        Self(CertificateEnum::CommitteeHotAuth(
            committee_hot_key_registration.clone(),
        ))
    }

    pub fn new_committee_hot_key_deregistration(
        committee_hot_key_deregistration: &CommitteeColdResign,
    ) -> Self {
        Self(CertificateEnum::CommitteeColdResign(
            committee_hot_key_deregistration.clone(),
        ))
    }

    pub fn new_drep_deregistration(drep_deregistration: &DrepDeregistration) -> Self {
        Self(CertificateEnum::DrepDeregistration(
            drep_deregistration.clone(),
        ))
    }

    pub fn new_drep_registration(drep_registration: &DrepRegistration) -> Self {
        Self(CertificateEnum::DrepRegistration(drep_registration.clone()))
    }

    pub fn new_drep_update(drep_update: &DrepUpdate) -> Self {
        Self(CertificateEnum::DrepUpdate(drep_update.clone()))
    }

    pub fn new_stake_and_vote_delegation(
        stake_and_vote_delegation: &StakeAndVoteDelegation,
    ) -> Self {
        Self(CertificateEnum::StakeAndVoteDelegation(
            stake_and_vote_delegation.clone(),
        ))
    }

    pub fn new_stake_registration_and_delegation(
        stake_registration_and_delegation: &StakeRegistrationAndDelegation,
    ) -> Self {
        Self(CertificateEnum::StakeRegistrationAndDelegation(
            stake_registration_and_delegation.clone(),
        ))
    }

    pub fn new_stake_vote_registration_and_delegation(
        stake_vote_registration_and_delegation: &StakeVoteRegistrationAndDelegation,
    ) -> Self {
        Self(CertificateEnum::StakeVoteRegistrationAndDelegation(
            stake_vote_registration_and_delegation.clone(),
        ))
    }

    pub fn new_vote_delegation(vote_delegation: &VoteDelegation) -> Self {
        Self(CertificateEnum::VoteDelegation(vote_delegation.clone()))
    }

    pub fn new_vote_registration_and_delegation(
        vote_registration_and_delegation: &VoteRegistrationAndDelegation,
    ) -> Self {
        Self(CertificateEnum::VoteRegistrationAndDelegation(
            vote_registration_and_delegation.clone(),
        ))
    }

    pub fn kind(&self) -> CertificateKind {
        match &self.0 {
            CertificateEnum::StakeRegistration(_) => CertificateKind::StakeRegistration,
            CertificateEnum::StakeDeregistration(_) => CertificateKind::StakeDeregistration,
            CertificateEnum::StakeDelegation(_) => CertificateKind::StakeDelegation,
            CertificateEnum::PoolRegistration(_) => CertificateKind::PoolRegistration,
            CertificateEnum::PoolRetirement(_) => CertificateKind::PoolRetirement,
            CertificateEnum::GenesisKeyDelegation(_) => CertificateKind::GenesisKeyDelegation,
            CertificateEnum::MoveInstantaneousRewardsCert(_) => {
                CertificateKind::MoveInstantaneousRewardsCert
            }
            CertificateEnum::CommitteeHotAuth(_) => {
                CertificateKind::CommitteeHotAuth
            }
            CertificateEnum::CommitteeColdResign(_) => {
                CertificateKind::CommitteeColdResign
            }
            CertificateEnum::DrepDeregistration(_) => CertificateKind::DrepDeregistration,
            CertificateEnum::DrepRegistration(_) => CertificateKind::DrepRegistration,
            CertificateEnum::DrepUpdate(_) => CertificateKind::DrepUpdate,
            CertificateEnum::StakeAndVoteDelegation(_) => CertificateKind::StakeAndVoteDelegation,
            CertificateEnum::StakeRegistrationAndDelegation(_) => {
                CertificateKind::StakeRegistrationAndDelegation
            }
            CertificateEnum::StakeVoteRegistrationAndDelegation(_) => {
                CertificateKind::StakeVoteRegistrationAndDelegation
            }
            CertificateEnum::VoteDelegation(_) => CertificateKind::VoteDelegation,
            CertificateEnum::VoteRegistrationAndDelegation(_) => {
                CertificateKind::VoteRegistrationAndDelegation
            }
        }
    }

    pub fn as_stake_registration(&self) -> Option<StakeRegistration> {
        match &self.0 {
            CertificateEnum::StakeRegistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            CertificateEnum::StakeDeregistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_stake_delegation(&self) -> Option<StakeDelegation> {
        match &self.0 {
            CertificateEnum::StakeDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_pool_registration(&self) -> Option<PoolRegistration> {
        match &self.0 {
            CertificateEnum::PoolRegistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_pool_retirement(&self) -> Option<PoolRetirement> {
        match &self.0 {
            CertificateEnum::PoolRetirement(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_genesis_key_delegation(&self) -> Option<GenesisKeyDelegation> {
        match &self.0 {
            CertificateEnum::GenesisKeyDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_move_instantaneous_rewards_cert(&self) -> Option<MoveInstantaneousRewardsCert> {
        match &self.0 {
            CertificateEnum::MoveInstantaneousRewardsCert(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_committee_hot_key_registration(&self) -> Option<CommitteeHotAuth> {
        match &self.0 {
            CertificateEnum::CommitteeHotAuth(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_committee_hot_key_deregistration(&self) -> Option<CommitteeColdResign> {
        match &self.0 {
            CertificateEnum::CommitteeColdResign(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_drep_deregistration(&self) -> Option<DrepDeregistration> {
        match &self.0 {
            CertificateEnum::DrepDeregistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_drep_registration(&self) -> Option<DrepRegistration> {
        match &self.0 {
            CertificateEnum::DrepRegistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_drep_update(&self) -> Option<DrepUpdate> {
        match &self.0 {
            CertificateEnum::DrepUpdate(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_stake_and_vote_delegation(&self) -> Option<StakeAndVoteDelegation> {
        match &self.0 {
            CertificateEnum::StakeAndVoteDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_stake_registration_and_delegation(&self) -> Option<StakeRegistrationAndDelegation> {
        match &self.0 {
            CertificateEnum::StakeRegistrationAndDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_stake_vote_registration_and_delegation(
        &self,
    ) -> Option<StakeVoteRegistrationAndDelegation> {
        match &self.0 {
            CertificateEnum::StakeVoteRegistrationAndDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_vote_delegation(&self) -> Option<VoteDelegation> {
        match &self.0 {
            CertificateEnum::VoteDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_vote_registration_and_delegation(&self) -> Option<VoteRegistrationAndDelegation> {
        match &self.0 {
            CertificateEnum::VoteRegistrationAndDelegation(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn has_required_script_witness(&self) -> bool {
        match &self.0 {
            CertificateEnum::StakeDeregistration(x) => x.has_script_credentials(),
            CertificateEnum::StakeDelegation(x) => x.has_script_credentials(),
            CertificateEnum::VoteDelegation(x) => x.has_script_credentials(),
            CertificateEnum::StakeAndVoteDelegation(x) => x.has_script_credentials(),
            CertificateEnum::StakeRegistrationAndDelegation(x) => x.has_script_credentials(),
            CertificateEnum::StakeVoteRegistrationAndDelegation(x) => x.has_script_credentials(),
            CertificateEnum::VoteRegistrationAndDelegation(x) => x.has_script_credentials(),
            CertificateEnum::CommitteeHotAuth(x) => x.has_script_credentials(),
            CertificateEnum::CommitteeColdResign(x) => x.has_script_credentials(),
            CertificateEnum::DrepDeregistration(x) => x.has_script_credentials(),
            CertificateEnum::DrepUpdate(x) => x.has_script_credentials(),
            _ => false,
        }
    }
}
