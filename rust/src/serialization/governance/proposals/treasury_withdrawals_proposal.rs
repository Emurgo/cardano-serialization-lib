use crate::serialization::struct_checks::serialize_and_check_index;
use crate::serialization::{check_len, deserialize_and_check_index};
use crate::*;
use map_names::VotingProposalIndexNames;
use num_traits::ToPrimitive;

impl Serialize for TreasuryWithdrawalsProposal {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;

        let proposal_index = VotingProposalIndexNames::TreasuryWithdrawalsAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "TreasuryWithdrawalsAction")?;

        self.withdrawals.serialize(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(TreasuryWithdrawalsProposal);

impl DeserializeEmbeddedGroup for TreasuryWithdrawalsProposal {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 2, "(proposal_index, withdrawals)")?;

        let desired_index = VotingProposalIndexNames::TreasuryWithdrawalsAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let withdrawals = TreasuryWithdrawals::deserialize(raw)?;

        return Ok(TreasuryWithdrawalsProposal { withdrawals });
    }
}
