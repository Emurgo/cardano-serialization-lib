use crate::serialization::{check_len};
use crate::*;

impl Serialize for VotingProposal {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.deposit.serialize(serializer)?;
        self.reward_account.serialize(serializer)?;
        self.governance_action.serialize(serializer)?;
        self.anchor.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(VotingProposal);

impl DeserializeEmbeddedGroup for VotingProposal {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            4,
            "(deposit, reward_account, gov_action, anchor)",
        )?;

        let deposit = Coin::deserialize(raw)
            .map_err(|e| e.annotate("deposit"))?;
        let reward_account = RewardAddress::deserialize(raw)
            .map_err(|e| e.annotate("reward_account"))?;
        let gov_action = GovernanceAction::deserialize(raw)
            .map_err(|e| e.annotate("gov_action"))?;
        let anchor = Anchor::deserialize(raw)
            .map_err(|e| e.annotate("anchor"))?;

        return Ok(VotingProposal {
            deposit,
            reward_account,
            governance_action: gov_action,
            anchor,
        });
    }
}
