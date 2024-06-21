use crate::*;
use crate::serialization::utils::{is_break_tag, skip_set_tag};

impl cbor_event::se::Serialize for VotingProposals {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.0.is_empty() {
            return Ok(serializer);
        }
        //TODO: uncomment this line when we conway ero will come
        //serializer.write_tag(258)?;
        let ordered_dedup = self.0.iter().collect::<BTreeSet<_>>();
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in ordered_dedup {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for VotingProposals {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
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
        Ok(Self(arr))
    }
}
