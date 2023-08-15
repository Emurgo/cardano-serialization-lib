use crate::*;
use std::io::{Seek, SeekFrom};

impl cbor_event::se::Serialize for CertificateEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            CertificateEnum::StakeRegistration(x) => x.serialize(serializer),
            CertificateEnum::StakeDeregistration(x) => x.serialize(serializer),
            CertificateEnum::StakeDelegation(x) => x.serialize(serializer),
            CertificateEnum::PoolRegistration(x) => x.serialize(serializer),
            CertificateEnum::PoolRetirement(x) => x.serialize(serializer),
            CertificateEnum::GenesisKeyDelegation(x) => x.serialize(serializer),
            CertificateEnum::MoveInstantaneousRewardsCert(x) => x.serialize(serializer),
            CertificateEnum::CommitteeHotKeyRegistration(x) => x.serialize(serializer),
            CertificateEnum::CommitteeHotKeyDeregistration(x) => x.serialize(serializer),
            CertificateEnum::DrepRegistration(x) => x.serialize(serializer),
            CertificateEnum::DrepDeregistration(x) => x.serialize(serializer),
            CertificateEnum::DrepUpdate(x) => x.serialize(serializer),
            CertificateEnum::StakeAndVoteDelegation(x) => x.serialize(serializer),
            CertificateEnum::StakeRegistrationAndDelegation(x) => x.serialize(serializer),
            CertificateEnum::StakeVoteRegistrationAndDelegation(x) => x.serialize(serializer),
            CertificateEnum::VoteDelegation(x) => x.serialize(serializer),
            CertificateEnum::VoteRegistrationAndDelegation(x) => x.serialize(serializer),
        }
    }
}

impl Deserialize for CertificateEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) =>
                /* TODO: check finite len somewhere */
                {
                    ()
                }
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break =>
                    /* it's ok */
                    {
                        ()
                    }
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
        .map_err(|e| e.annotate("CertificateEnum"))
    }
}

impl DeserializeEmbeddedGroup for CertificateEnum {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let cert_index = get_cert_index(raw)?;

        match cert_index {
            super::stake_registration::STAKE_REG_LEGACY_INDEX => {
                Ok(CertificateEnum::StakeRegistration(
                    StakeRegistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::stake_registration::STAKE_REG_CONWAY_INDEX => {
                Ok(CertificateEnum::StakeRegistration(
                    StakeRegistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::stake_deregistration::DEREG_STAKE_CERT_LEGACY_INDEX => {
                Ok(CertificateEnum::StakeDeregistration(
                    StakeDeregistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::stake_deregistration::DEREG_STAKE_CERT_CONWAY_INDEX => {
                Ok(CertificateEnum::StakeDeregistration(
                    StakeDeregistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::stake_delegation::STAKE_DELEGATION_CERT_INDEX => {
                Ok(CertificateEnum::StakeDelegation(
                    StakeDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::pool_registration::REG_POOL_CERT_INDEX => Ok(CertificateEnum::PoolRegistration(
                PoolRegistration::deserialize_as_embedded_group(raw, len)?,
            )),
            super::pool_retirement::RETIRE_POOL_CERT_INDEX => Ok(CertificateEnum::PoolRetirement(
                PoolRetirement::deserialize_as_embedded_group(raw, len)?,
            )),
            super::genesis_key_delegation::GENESIS_KEY_DELEGATION_INDEX => {
                Ok(CertificateEnum::GenesisKeyDelegation(
                    GenesisKeyDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::move_instantaneous_rewards_cert::MIR_CERT_INDEX => {
                Ok(CertificateEnum::MoveInstantaneousRewardsCert(
                    MoveInstantaneousRewardsCert::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::committee_hot_key_registration::REG_COMMITTEE_HOT_KEY_CERT_INDEX => {
                Ok(CertificateEnum::CommitteeHotKeyRegistration(
                    CommitteeHotKeyRegistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::committee_hot_key_deregistration::UNREG_COMMITTEE_HOT_KEY_CERT_INDEX => {
                Ok(CertificateEnum::CommitteeHotKeyDeregistration(
                    CommitteeHotKeyDeregistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::drep_registration::REG_DREP_CERT_INDEX => Ok(CertificateEnum::DrepRegistration(
                DrepRegistration::deserialize_as_embedded_group(raw, len)?,
            )),
            super::drep_deregistration::DEREG_DREP_CERT_INDEX => {
                Ok(CertificateEnum::DrepDeregistration(
                    DrepDeregistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::drep_update::UPDATE_DREP_CERT_INDEX => Ok(CertificateEnum::DrepUpdate(
                DrepUpdate::deserialize_as_embedded_group(raw, len)?,
            )),
            super::stake_and_vote_delegation::STAKE_VOTE_DELEG_CERT_INDEX => {
                Ok(CertificateEnum::StakeAndVoteDelegation(
                    StakeAndVoteDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::stake_registration_and_delegation::STAKE_REG_DELEG_CERT_INDEX => {
                Ok(CertificateEnum::StakeRegistrationAndDelegation(
                    StakeRegistrationAndDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::stake_vote_registration_and_delegation::STAKE_VOTE_REG_DELEG_CERT_INDEX => {
                Ok(CertificateEnum::StakeVoteRegistrationAndDelegation(
                    StakeVoteRegistrationAndDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            super::vote_delegation::VOTE_CERT_INDEX => Ok(CertificateEnum::VoteDelegation(
                VoteDelegation::deserialize_as_embedded_group(raw, len)?,
            )),
            super::vote_registration_and_delegation::VOTE_REG_DELEG_CERT_INDEX => {
                Ok(CertificateEnum::VoteRegistrationAndDelegation(
                    VoteRegistrationAndDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            _ => Err(DeserializeError::new(
                "CertificateEnum",
                DeserializeFailure::UnknownKey(Key::Uint(cert_index)),
            )),
        }
    }
}

impl cbor_event::se::Serialize for Certificate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for Certificate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(CertificateEnum::deserialize(raw)?))
    }
}

fn get_cert_index<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<u64, DeserializeError> {
    let initial_position = raw
        .as_mut_ref()
        .seek(SeekFrom::Current(0))
        .map_err(|err| DeserializeFailure::IoError(err.to_string()))?;
    let index = raw.unsigned_integer()?;
    raw.as_mut_ref()
        .seek(SeekFrom::Start(initial_position))
        .map_err(|err| DeserializeFailure::IoError(err.to_string()))?;
    Ok(index)
}
