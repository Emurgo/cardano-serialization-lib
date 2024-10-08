use crate::*;
use crate::serialization::utils::{is_break_tag, skip_set_tag};

impl cbor_event::se::Serialize for VotingProposals {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(258)?;
        serializer.write_array(cbor_event::Len::Len(self.len() as u64))?;
        for element in self {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for VotingProposals {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let has_set_tag = skip_set_tag(raw)?;
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            skip_set_tag(raw)?;
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "VotingProposals")? {
                    break;
                }
                arr.push(VotingProposal::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("VotingProposals"))?;
        let mut proposals = Self::from_vec(arr);
        if has_set_tag {
            proposals.set_set_type(CborSetType::Tagged);
        } else {
            proposals.set_set_type(CborSetType::Untagged);
        }
        Ok(proposals)
    }
}
