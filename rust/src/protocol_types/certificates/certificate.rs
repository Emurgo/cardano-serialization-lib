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
    DRepDeregistration,
    DRepRegistration,
    DRepUpdate,
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
    DRepDeregistration(DRepDeregistration),
    DRepRegistration(DRepRegistration),
    DRepUpdate(DRepUpdate),
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

    /// Since StakeRegistration can represent stake_registration certificate or reg_cert certificate, because both certificates have the same semantics.
    /// And in some cases you want to create a reg_cert, this function is used to create a reg_cert.
    /// The function will return an error if StakeRegistration represents a stake_registration certificate.
    pub fn new_reg_cert(stake_registration: &StakeRegistration) -> Result<Certificate, JsError> {
        if stake_registration.coin.is_none() {
            return Err(JsError::from_str("coin is required"));
        } else {
            Ok(Self(CertificateEnum::StakeRegistration(
                stake_registration.clone(),
            )))
        }
    }

    pub fn new_stake_deregistration(stake_deregistration: &StakeDeregistration) -> Self {
        Self(CertificateEnum::StakeDeregistration(
            stake_deregistration.clone(),
        ))
    }

    /// Since StakeDeregistration can represent stake_deregistration certificate or unreg_cert certificate, because both certificates have the same semantics.
    /// And in some cases you want to create an unreg_cert, this function is used to create an unreg_cert.
    /// The function will return an error if StakeDeregistration represents a stake_deregistration certificate.
    pub fn new_unreg_cert(stake_deregistration: &StakeDeregistration) -> Result<Certificate, JsError> {
        if stake_deregistration.coin.is_none() {
            return Err(JsError::from_str("coin is required"));
        } else {
            Ok(Self(CertificateEnum::StakeDeregistration(
                stake_deregistration.clone(),
            )))
        }
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

    pub fn new_committee_hot_auth(
        committee_hot_auth: &CommitteeHotAuth,
    ) -> Self {
        Self(CertificateEnum::CommitteeHotAuth(
            committee_hot_auth.clone(),
        ))
    }

    pub fn new_committee_cold_resign(
        committee_cold_resign: &CommitteeColdResign,
    ) -> Self {
        Self(CertificateEnum::CommitteeColdResign(
            committee_cold_resign.clone(),
        ))
    }

    pub fn new_drep_deregistration(drep_deregistration: &DRepDeregistration) -> Self {
        Self(CertificateEnum::DRepDeregistration(
            drep_deregistration.clone(),
        ))
    }

    pub fn new_drep_registration(drep_registration: &DRepRegistration) -> Self {
        Self(CertificateEnum::DRepRegistration(drep_registration.clone()))
    }

    pub fn new_drep_update(drep_update: &DRepUpdate) -> Self {
        Self(CertificateEnum::DRepUpdate(drep_update.clone()))
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
            CertificateEnum::DRepDeregistration(_) => CertificateKind::DRepDeregistration,
            CertificateEnum::DRepRegistration(_) => CertificateKind::DRepRegistration,
            CertificateEnum::DRepUpdate(_) => CertificateKind::DRepUpdate,
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

    /// Since StakeRegistration can represent stake_registration certificate or reg_cert certificate, because both certificates have the same semantics.
    /// And in some cases you want to get a reg_cert, this function is used to get a reg_cert.
    /// The function will return None if StakeRegistration represents a stake_registration certificate or Certificate is not a StakeRegistration.
    pub fn as_reg_cert(&self) -> Option<StakeRegistration> {
        match &self.0 {
            CertificateEnum::StakeRegistration(x) => {
                return if x.coin.is_some() {
                    Some(x.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            CertificateEnum::StakeDeregistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    /// Since StakeDeregistration can represent stake_deregistration certificate or unreg_cert certificate, because both certificates have the same semantics.
    /// And in some cases you want to get an unreg_cert, this function is used to get an unreg_cert.
    /// The function will return None if StakeDeregistration represents a stake_deregistration certificate or Certificate is not a StakeDeregistration.
    pub fn as_unreg_cert(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            CertificateEnum::StakeDeregistration(x) => {
                return if x.coin.is_some() {
                    Some(x.clone())
                } else {
                    None
                }
            }
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

    pub fn as_committee_hot_auth(&self) -> Option<CommitteeHotAuth> {
        match &self.0 {
            CertificateEnum::CommitteeHotAuth(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_committee_cold_resign(&self) -> Option<CommitteeColdResign> {
        match &self.0 {
            CertificateEnum::CommitteeColdResign(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_drep_deregistration(&self) -> Option<DRepDeregistration> {
        match &self.0 {
            CertificateEnum::DRepDeregistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_drep_registration(&self) -> Option<DRepRegistration> {
        match &self.0 {
            CertificateEnum::DRepRegistration(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn as_drep_update(&self) -> Option<DRepUpdate> {
        match &self.0 {
            CertificateEnum::DRepUpdate(x) => Some(x.clone()),
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
            CertificateEnum::StakeRegistration(x) => {
                if x.coin.is_some() {
                    return x.has_script_credentials();
                } else {
                    return false;
                }
            }
            CertificateEnum::StakeDeregistration(x) => x.has_script_credentials(),
            CertificateEnum::StakeDelegation(x) => x.has_script_credentials(),
            CertificateEnum::VoteDelegation(x) => x.has_script_credentials(),
            CertificateEnum::StakeAndVoteDelegation(x) => x.has_script_credentials(),
            CertificateEnum::StakeRegistrationAndDelegation(x) => x.has_script_credentials(),
            CertificateEnum::StakeVoteRegistrationAndDelegation(x) => x.has_script_credentials(),
            CertificateEnum::VoteRegistrationAndDelegation(x) => x.has_script_credentials(),
            CertificateEnum::CommitteeHotAuth(x) => x.has_script_credentials(),
            CertificateEnum::CommitteeColdResign(x) => x.has_script_credentials(),
            CertificateEnum::DRepRegistration(x) => x.has_script_credentials(),
            CertificateEnum::DRepDeregistration(x) => x.has_script_credentials(),
            CertificateEnum::DRepUpdate(x) => x.has_script_credentials(),
            _ => false,
        }
    }
}
