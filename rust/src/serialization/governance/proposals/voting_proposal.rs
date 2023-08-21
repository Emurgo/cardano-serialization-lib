use crate::serialization::map_names::VotingProposalIndexNames;
use crate::serialization::struct_checks::{check_len_indefinite, serialize_and_check_index};
use crate::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{Seek, SeekFrom};

impl Serialize for VotingProposal {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match &self.0 {
            VotingProposalEnum::ParameterChangeProposal(x) => x.serialize(serializer),
            VotingProposalEnum::HardForkInitiationProposal(x) => x.serialize(serializer),
            VotingProposalEnum::TreasuryWithdrawalsProposal(x) => x.serialize(serializer),
            VotingProposalEnum::NoConfidenceProposal(x) => x.serialize(serializer),
            VotingProposalEnum::NewCommitteeProposal(x) => x.serialize(serializer),
            VotingProposalEnum::NewConstitutionProposal(x) => x.serialize(serializer),
            VotingProposalEnum::InfoProposal(_) => {
                let index = VotingProposalIndexNames::InfoAction.to_u64();
                serialize_and_check_index(serializer, index, "VotingProposalEnum::InfoProposal")
            }
        }
    }
}

impl Deserialize for VotingProposal {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            if let Ok(index) = raw.unsigned_integer() {
                let expected_index = VotingProposalIndexNames::InfoAction.to_u64().ok_or(
                    DeserializeFailure::CustomError(
                        "unknown index of VotingProposalEnum::InfoProposal".to_string(),
                    ),
                )?;
                if index != expected_index {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index),
                        expected: Key::Uint(expected_index),
                    }
                    .into());
                }
            }

            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            check_len_indefinite(raw, len)?;
            ret
        })()
        .map_err(|e| e.annotate("VotingProposal"))
    }
}

impl DeserializeEmbeddedGroup for VotingProposal {
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
                Ok::<VotingProposalEnum, DeserializeError>(
                    VotingProposalEnum::ParameterChangeProposal(
                        ParameterChangeProposal::deserialize_as_embedded_group(raw, len)?,
                    ),
                )
            }
            VotingProposalIndexNames::HardForkInitiationAction => {
                Ok(VotingProposalEnum::HardForkInitiationProposal(
                    HardForkInitiationProposal::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::TreasuryWithdrawalsAction => {
                Ok(VotingProposalEnum::TreasuryWithdrawalsProposal(
                    TreasuryWithdrawalsProposal::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::NoConfidenceAction => {
                Ok(VotingProposalEnum::NoConfidenceProposal(
                    NoConfidenceProposal::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::NewCommitteeAction => {
                Ok(VotingProposalEnum::NewCommitteeProposal(
                    NewCommitteeProposal::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::NewConstitutionAction => {
                Ok(VotingProposalEnum::NewConstitutionProposal(
                    NewConstitutionProposal::deserialize_as_embedded_group(raw, len)?,
                ))
            }
            VotingProposalIndexNames::InfoAction => {
                Ok(VotingProposalEnum::InfoProposal(InfoProposal::new()))
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
