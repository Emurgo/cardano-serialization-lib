use crate::*;
use std::collections::BTreeMap;

impl cbor_event::se::Serialize for VotingProcedures {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;

        for (voter, votes) in &self.0 {
            voter.serialize(serializer)?;
            serializer.write_map(cbor_event::Len::Len(votes.len() as u64))?;
            for (governance_action_id, voting_procedure) in votes {
                governance_action_id.serialize(serializer)?;
                voting_procedure.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for VotingProcedures {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut voter_to_vote = BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => voter_to_vote.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    return Err(
                        DeserializeError::from(DeserializeFailure::EndingBreakMissing)
                            .annotate("voting_procedure map"),
                    );
                }

                let key = Voter::deserialize(raw).map_err(|e| e.annotate("voter"))?;

                let value = deserialize_internal_map(raw)
                    .map_err(|e| e.annotate("voting_procedure map"))?;

                if voter_to_vote.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(Self(voter_to_vote))
        })()
        .map_err(|e| e.annotate("VotingProcedures"))
    }
}

fn deserialize_internal_map<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<BTreeMap<GovernanceActionId, VotingProcedure>, DeserializeError> {
    let mut gov_act_id_to_vote = BTreeMap::new();
    (|| -> Result<_, DeserializeError> {
        let len = raw.map()?;
        while match len {
            cbor_event::Len::Len(n) => gov_act_id_to_vote.len() < n as usize,
            cbor_event::Len::Indefinite => true,
        } {
            if raw.cbor_type()? == CBORType::Special {
                return Err(
                    DeserializeError::from(DeserializeFailure::EndingBreakMissing)
                        .annotate("gov_act_id_to_vote map"),
                );
            }

            let key = GovernanceActionId::deserialize(raw).map_err(|e| e.annotate("gov_act_id"))?;

            let value =
                VotingProcedure::deserialize(raw).map_err(|e| e.annotate("voting_procedure"))?;

            if gov_act_id_to_vote.insert(key.clone(), value).is_some() {
                return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                    "some complicated/unsupported type",
                )))
                .into());
            }
        }
        Ok(gov_act_id_to_vote)
    })()
    .map_err(|e| e.annotate("VotingProcedures (gov_act_id to vote_procedure map)"))
}
