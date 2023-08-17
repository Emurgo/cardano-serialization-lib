use crate::serialization::struct_checks::{check_len_indefinite, serialize_and_check_index};
use crate::serialization::{check_len, deserialize_and_check_index};
use crate::*;
use map_names::VotingProposalIndexNames;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for HardForkInitiationProposal {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index = VotingProposalIndexNames::HardForkInitiationAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "HardForkInitiationAction")?;

        if let Some(gov_id) = &self.gov_action_id {
            gov_id.serialize(serializer)?;
        } else {
            serializer.write_special(CBORSpecial::Null)?;
        }

        serializer.write_array(cbor_event::Len::Len(1))?;
        self.protocol_version.serialize(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_tuple!(HardForkInitiationProposal);

impl DeserializeEmbeddedGroup for HardForkInitiationProposal {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            3,
            "(proposal_index, gov_action_id // null, protocol_version)",
        )?;

        let desired_index = VotingProposalIndexNames::HardForkInitiationAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let gov_action_id = GovernanceActionId::deserialize_nullable(raw)
            .map_err(|e| e.annotate("gov_action_id"))?;

        let protocol_version = deserialize_embedded_protocol_version(raw)?;

        return Ok(HardForkInitiationProposal {
            gov_action_id,
            protocol_version,
        });
    }
}

fn deserialize_embedded_protocol_version<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<ProtocolVersion, DeserializeError> {
    let len = raw.array()?;

    check_len(len, 1, "(protocol_version)")?;

    let protocol_version =
        ProtocolVersion::deserialize(raw).map_err(|e| e.annotate("protocol_version"))?;
    check_len_indefinite(raw, len)?;
    Ok(protocol_version)
}
