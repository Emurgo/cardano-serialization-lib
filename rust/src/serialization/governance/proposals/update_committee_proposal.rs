use crate::serialization::utils::serialize_and_check_index;
use crate::serialization::{check_len, deserialize_and_check_index};
use crate::*;
use map_names::VotingProposalIndexNames;
use num_traits::ToPrimitive;

impl Serialize for UpdateCommitteeProposal {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index = VotingProposalIndexNames::UpdateCommitteeAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "UpdateCommitteeAction")?;

        self.gov_action_id.serialize_nullable(serializer)?;

        let members_to_remove = Credentials(self.members_to_remove.iter().cloned().collect());
        members_to_remove.serialize(serializer)?;

        self.committee.serialize(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(UpdateCommitteeProposal);

impl DeserializeEmbeddedGroup for UpdateCommitteeProposal {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            4,
            "(proposal_index, gov_action_id / null, set<$committee_cold_credential>, committee)",
        )?;

        let desired_index = VotingProposalIndexNames::UpdateCommitteeAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let gov_action_id = GovernanceActionId::deserialize_nullable(raw)
            .map_err(|e| e.annotate("gov_action_id"))?;

        let members_to_remove =
            Credentials::deserialize(raw).map_err(|e| e.annotate("members_to_remove"))?;

        let committee = Committee::deserialize(raw).map_err(|e| e.annotate("committee"))?;

        return Ok(UpdateCommitteeProposal {
            gov_action_id,
            members_to_remove: members_to_remove.0.iter().cloned().collect(),
            committee,
        });
    }
}
