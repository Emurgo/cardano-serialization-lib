use crate::serialization::utils::serialize_and_check_index;
use crate::serialization::{check_len, deserialize_and_check_index};
use crate::*;
use map_names::VotingProposalIndexNames;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for ParameterChangeAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index = VotingProposalIndexNames::ParameterChangeAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "ParameterChangeAction")?;

        self.gov_action_id.serialize_nullable(serializer)?;
        self.protocol_param_updates.serialize(serializer)?;
        self.policy_hash.serialize_nullable(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(ParameterChangeAction);

impl DeserializeEmbeddedGroup for ParameterChangeAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let has_policy_hash = len == cbor_event::Len::Len(4) || len == cbor_event::Len::Indefinite;

        //for sancho backwards compatibility
        if !has_policy_hash {
            check_len(
                len,
                3,
                "(proposal_index, gov_action_id // null, protocol_param_updates)",
            )?;
        } else {
            check_len(
                len,
                4,
                "(proposal_index, gov_action_id // null, protocol_param_updates, policy_hash // null)",
            )?;
        }

        let desired_index = VotingProposalIndexNames::ParameterChangeAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let gov_action_id = GovernanceActionId::deserialize_nullable(raw)
            .map_err(|e| e.annotate("gov_action_id"))?;

        let protocol_param_updates = ProtocolParamUpdate::deserialize(raw)
            .map_err(|e| e.annotate("protocol_param_updates"))?;

        let policy_hash = if has_policy_hash {
            ScriptHash::deserialize_nullable(raw).map_err(|e| e.annotate("policy_hash"))?
        } else {
            None
        };

        return Ok(ParameterChangeAction {
            gov_action_id,
            protocol_param_updates,
            policy_hash,
        });
    }
}
