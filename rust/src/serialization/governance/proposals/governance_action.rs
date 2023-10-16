use crate::serialization::map_names::VotingProposalIndexNames;
use crate::serialization::utils::{check_len_indefinite, serialize_and_check_index};
use crate::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{Seek, SeekFrom};

impl Serialize for GovernanceAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match &self.0 {
            GovernanceActionEnum::ParameterChangeAction(x) => x.serialize(serializer),
            GovernanceActionEnum::HardForkInitiationAction(x) => x.serialize(serializer),
            GovernanceActionEnum::TreasuryWithdrawalsAction(x) => x.serialize(serializer),
            GovernanceActionEnum::NoConfidenceAction(x) => x.serialize(serializer),
            GovernanceActionEnum::UpdateCommitteeAction(x) => x.serialize(serializer),
            GovernanceActionEnum::NewConstitutionAction(x) => x.serialize(serializer),
            GovernanceActionEnum::InfoAction(x) => x.serialize(serializer),
        }
    }
}

impl Deserialize for GovernanceAction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len)?;
            check_len_indefinite(raw, len)?;
            Ok(ret)
        })()
        .map_err(|e| e.annotate("VotingProposal"))
    }
}

impl DeserializeEmbeddedGroup for GovernanceAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let cert_index = get_proposal_index(raw)?;
        let index_enum =
            VotingProposalIndexNames::from_u64(cert_index).ok_or(DeserializeError::new(
                "VotingProposal",
                DeserializeFailure::UnknownKey(Key::Uint(cert_index)),
            ))?;

        let proposal_enum = match index_enum {
            VotingProposalIndexNames::ParameterChangeAction => {
                Ok::<GovernanceActionEnum, DeserializeError>(
                    GovernanceActionEnum::ParameterChangeAction(
                        ParameterChangeAction::deserialize_as_embedded_group(raw, len)?,
                    ),
                )
            }
            VotingProposalIndexNames::HardForkInitiationAction => {
                Ok(GovernanceActionEnum::HardForkInitiationAction(
                    HardForkInitiationAction::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::TreasuryWithdrawalsAction => {
                Ok(GovernanceActionEnum::TreasuryWithdrawalsAction(
                    TreasuryWithdrawalsAction::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::NoConfidenceAction => {
                Ok(GovernanceActionEnum::NoConfidenceAction(
                    NoConfidenceAction::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::UpdateCommitteeAction => {
                Ok(GovernanceActionEnum::UpdateCommitteeAction(
                    UpdateCommitteeAction::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::NewConstitutionAction => {
                Ok(GovernanceActionEnum::NewConstitutionAction(
                    NewConstitutionAction::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::InfoAction => {
                Ok(GovernanceActionEnum::InfoAction(
                    InfoAction::deserialize_as_embedded_group(raw, len)?,
                ))
            }
        }?;

        Ok(Self(proposal_enum))
    }
}

fn get_proposal_index<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<u64, DeserializeError> {
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
