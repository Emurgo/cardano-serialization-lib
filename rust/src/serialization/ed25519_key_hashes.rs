use crate::*;
use crate::serialization::utils::{is_break_tag, skip_set_tag};

impl Serialize for Ed25519KeyHashes {
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

impl Deserialize for Ed25519KeyHashes {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let has_set_tag = skip_set_tag(raw)?;
        let mut creds = Ed25519KeyHashes::new();
        let mut total = 0u64;
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => total < n,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "Ed25519KeyHashes")? {
                    break;
                }
                creds.add_move(Ed25519KeyHash::deserialize(raw)?);
                total += 1;
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("Ed25519KeyHashes"))?;
        if has_set_tag {
            creds.set_set_type(CborSetType::Tagged);
        } else {
            creds.set_set_type(CborSetType::Untagged);
        }
        Ok(creds)
    }
}