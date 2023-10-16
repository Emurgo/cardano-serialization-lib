use crate::serialization::utils::{check_len_indefinite, serialize_and_check_index};
use crate::serialization::{check_len, deserialize_and_check_index};
use crate::*;
use map_names::VotingProposalIndexNames;
use num_traits::ToPrimitive;

impl cbor_event::se::Serialize for InfoAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;

        let proposal_index = VotingProposalIndexNames::InfoAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "InfoAction")?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(InfoAction);

impl DeserializeEmbeddedGroup for InfoAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            1,
            "(proposal_index)",
        )?;

        let desired_index = VotingProposalIndexNames::InfoAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        return Ok(InfoAction());
    }
}