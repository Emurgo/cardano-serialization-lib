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
    ConstitutionalCommitteeHotCred(Credential),
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
    pub fn new_constitutional_committee_hot_credential(cred: &Credential) -> Self {
        Self(VoterEnum::ConstitutionalCommitteeHotCred(cred.clone()))
    }

    pub fn new_drep_credential(cred: &Credential) -> Self {
        Self(VoterEnum::DRep(cred.clone()))
    }

    pub fn new_stake_pool_key_hash(key_hash: &Ed25519KeyHash) -> Self {
        Self(VoterEnum::StakingPool(key_hash.clone()))
    }

    pub fn kind(&self) -> VoterKind {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotCred(cred) => match cred.kind() {
                CredKind::Key => VoterKind::ConstitutionalCommitteeHotKeyHash,
                CredKind::Script => VoterKind::ConstitutionalCommitteeHotScriptHash,
            },
            VoterEnum::DRep(cred) => match cred.kind() {
                CredKind::Key => VoterKind::DRepKeyHash,
                CredKind::Script => VoterKind::DRepScriptHash,
            },
            VoterEnum::StakingPool(_) => VoterKind::StakingPoolKeyHash,
        }
    }

    pub fn to_constitutional_committee_hot_credential(&self) -> Option<Credential> {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotCred(cred) => Some(cred.clone()),
            _ => None,
        }
    }

    pub fn to_drep_credential(&self) -> Option<Credential> {
        match &self.0 {
            VoterEnum::DRep(cred) => Some(cred.clone()),
            _ => None,
        }
    }

    pub fn to_stake_pool_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            VoterEnum::StakingPool(key_hash) => Some(key_hash.clone()),
            _ => None,
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotCred(cred) => cred.has_script_hash(),
            VoterEnum::DRep(cred) => cred.has_script_hash(),
            VoterEnum::StakingPool(_) => false,
        }
    }

    pub fn to_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotCred(cred) => cred.to_keyhash(),
            VoterEnum::DRep(cred) => cred.to_keyhash(),
            VoterEnum::StakingPool(key_hash) => Some(key_hash.clone()),
        }
    }
}
