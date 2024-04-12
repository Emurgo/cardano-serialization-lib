use crate::serialization::map_names::CertificateIndexNames;
use crate::*;
use num_traits::FromPrimitive;
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
            CertificateEnum::CommitteeHotAuth(x) => x.serialize(serializer),
            CertificateEnum::CommitteeColdResign(x) => x.serialize(serializer),
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
        let index_enum =
            CertificateIndexNames::from_u64(cert_index).ok_or(DeserializeError::new(
                "CertificateEnum",
                DeserializeFailure::UnknownKey(Key::Uint(cert_index)),
            ))?;

        match index_enum {
            CertificateIndexNames::StakeRegistrationLegacy => {
                Ok(CertificateEnum::StakeRegistration(
                    StakeRegistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::StakeRegistrationConway => {
                Ok(CertificateEnum::StakeRegistration(
                    StakeRegistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::StakeDeregistrationLegacy => {
                Ok(CertificateEnum::StakeDeregistration(
                    StakeDeregistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::StakeDeregistrationConway => {
                Ok(CertificateEnum::StakeDeregistration(
                    StakeDeregistration::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::StakeDelegation => Ok(CertificateEnum::StakeDelegation(
                StakeDelegation::deserialize_as_embedded_group(raw, len)?,
            )),

            CertificateIndexNames::PoolRegistration => Ok(CertificateEnum::PoolRegistration(
                PoolRegistration::deserialize_as_embedded_group(raw, len)?,
            )),
            CertificateIndexNames::PoolRetirement => Ok(CertificateEnum::PoolRetirement(
                PoolRetirement::deserialize_as_embedded_group(raw, len)?,
            )),
            CertificateIndexNames::GenesisKeyDelegation => {
                Ok(CertificateEnum::GenesisKeyDelegation(
                    GenesisKeyDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::MoveInstantaneousRewardsCert => {
                Ok(CertificateEnum::MoveInstantaneousRewardsCert(
                    MoveInstantaneousRewardsCert::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::CommitteeHotAuth => {
                Ok(CertificateEnum::CommitteeHotAuth(
                    CommitteeHotAuth::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::CommitteeColdResign => {
                Ok(CertificateEnum::CommitteeColdResign(
                    CommitteeColdResign::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::DrepRegistration => Ok(CertificateEnum::DrepRegistration(
                DrepRegistration::deserialize_as_embedded_group(raw, len)?,
            )),
            CertificateIndexNames::DrepDeregistration => Ok(CertificateEnum::DrepDeregistration(
                DrepDeregistration::deserialize_as_embedded_group(raw, len)?,
            )),
            CertificateIndexNames::DrepUpdate => Ok(CertificateEnum::DrepUpdate(
                DrepUpdate::deserialize_as_embedded_group(raw, len)?,
            )),
            CertificateIndexNames::StakeAndVoteDelegation => {
                Ok(CertificateEnum::StakeAndVoteDelegation(
                    StakeAndVoteDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::StakeRegistrationAndDelegation => {
                Ok(CertificateEnum::StakeRegistrationAndDelegation(
                    StakeRegistrationAndDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::StakeVoteRegistrationAndDelegation => {
                Ok(CertificateEnum::StakeVoteRegistrationAndDelegation(
                    StakeVoteRegistrationAndDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            CertificateIndexNames::VoteDelegation => Ok(CertificateEnum::VoteDelegation(
                VoteDelegation::deserialize_as_embedded_group(raw, len)?,
            )),
            CertificateIndexNames::VoteRegistrationAndDelegation => {
                Ok(CertificateEnum::VoteRegistrationAndDelegation(
                    VoteRegistrationAndDelegation::deserialize_as_embedded_group(raw, len)?,
                ))
            }
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
