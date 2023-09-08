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
pub(crate) enum VoterEnum {
    ConstitutionalCommitteeHotKey(Credential),
    DRep(Credential),
    StakingPool(Ed25519KeyHash),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum VoterKind {
    ConstitutionalCommitteeHotKeyHash,
    ConstitutionalCommitteeHotScriptHash,
    DRepKeyHash,
    DRepScriptHash,
    StakingPoolKeyHash,
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
pub struct Voter(pub(crate) VoterEnum);

impl_to_from!(Voter);

#[wasm_bindgen]
impl Voter {
    pub fn new_constitutional_committee_hot_key(cred: &Credential) -> Self {
        Self(VoterEnum::ConstitutionalCommitteeHotKey(cred.clone()))
    }

    pub fn new_drep(cred: &Credential) -> Self {
        Self(VoterEnum::DRep(cred.clone()))
    }

    pub fn new_staking_pool(key_hash: &Ed25519KeyHash) -> Self {
        Self(VoterEnum::StakingPool(key_hash.clone()))
    }

    pub fn kind(&self) -> VoterKind {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => match cred.kind() {
                StakeCredKind::Key => VoterKind::ConstitutionalCommitteeHotKeyHash,
                StakeCredKind::Script => VoterKind::ConstitutionalCommitteeHotScriptHash,
            },
            VoterEnum::DRep(cred) => match cred.kind() {
                StakeCredKind::Key => VoterKind::DRepKeyHash,
                StakeCredKind::Script => VoterKind::DRepScriptHash,
            },
            VoterEnum::StakingPool(_) => VoterKind::StakingPoolKeyHash,
        }
    }

    pub fn to_constitutional_committee_hot_cred(&self) -> Option<Credential> {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => Some(cred.clone()),
            _ => None,
        }
    }

    pub fn to_drep_cred(&self) -> Option<Credential> {
        match &self.0 {
            VoterEnum::DRep(cred) => Some(cred.clone()),
            _ => None,
        }
    }

    pub fn to_staking_pool_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            VoterEnum::StakingPool(key_hash) => Some(key_hash.clone()),
            _ => None,
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => cred.has_script_hash(),
            VoterEnum::DRep(cred) => cred.has_script_hash(),
            VoterEnum::StakingPool(_) => false,
        }
    }

    pub fn to_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => cred.to_keyhash(),
            VoterEnum::DRep(cred) => cred.to_keyhash(),
            VoterEnum::StakingPool(key_hash) => Some(key_hash.clone()),
        }
    }
}
