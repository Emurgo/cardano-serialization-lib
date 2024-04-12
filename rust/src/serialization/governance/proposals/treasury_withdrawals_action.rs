use crate::serialization::utils::serialize_and_check_index;
use crate::serialization::{check_len, deserialize_and_check_index};
use crate::*;
use map_names::VotingProposalIndexNames;
use num_traits::ToPrimitive;

impl Serialize for TreasuryWithdrawalsAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index = VotingProposalIndexNames::TreasuryWithdrawalsAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "TreasuryWithdrawalsAction")?;

        self.withdrawals.serialize(serializer)?;
        self.policy_hash.serialize_nullable(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(TreasuryWithdrawalsAction);

impl DeserializeEmbeddedGroup for TreasuryWithdrawalsAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {

        let has_policy_hash = len == cbor_event::Len::Len(3) || len == cbor_event::Len::Indefinite;

        //for sancho backwards compatibility
        if !has_policy_hash {
            check_len(len, 2, "(proposal_index, { reward_account => coin })")?;
        } else {
            check_len(len, 3, "(proposal_index, { reward_account => coin }, policy_hash / null)")?;
        }


        let desired_index = VotingProposalIndexNames::TreasuryWithdrawalsAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let withdrawals = TreasuryWithdrawals::deserialize(raw)?;

        let policy_hash = if has_policy_hash {
            ScriptHash::deserialize_nullable(raw)?
        } else {
            None
        };

        return Ok(TreasuryWithdrawalsAction { withdrawals , policy_hash});
    }
}
