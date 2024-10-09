use crate::serialization::utils::{is_break_tag, skip_set_tag};
use crate::*;

impl Serialize for Certificates {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(258)?;
        serializer.write_array(Len::Len(self.len() as u64))?;
        for element in &self.certs {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Certificates {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let has_set_tag= skip_set_tag(raw)?;
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "Certificates")? {
                    break;
                }
                arr.push(Certificate::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("Certificates"))?;
        let mut certs = Self::from_vec(arr);
        if has_set_tag {
            certs.set_set_type(CborSetType::Tagged);
        } else {
            certs.set_set_type(CborSetType::Untagged);
        }
        Ok(certs)
    }
}
