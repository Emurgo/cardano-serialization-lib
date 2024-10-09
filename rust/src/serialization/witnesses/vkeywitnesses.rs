use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::protocol_types::Deserialize;
use crate::{CborSetType, DeserializeError, Vkeywitness, Vkeywitnesses};
use crate::serialization::utils::skip_set_tag;

impl cbor_event::se::Serialize for Vkeywitnesses {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.force_original_cbor_set_type() {
            if self.get_set_type() == CborSetType::Tagged {
                serializer.write_tag(258)?;
            }
        } else {
            serializer.write_tag(258)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.len() as u64))?;
        for element in self {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Vkeywitnesses {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let has_set_tag = skip_set_tag(raw)?;
        let mut wits = Vkeywitnesses::new();
        let mut total = 0u64;
        (|| -> Result<_, DeserializeError> {
            skip_set_tag(raw)?;
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => total < n,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                wits.add_move(Vkeywitness::deserialize(raw)?);
                total += 1;
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("Vkeywitnesses"))?;

        if has_set_tag {
            wits.set_set_type(CborSetType::Tagged);
        } else {
            wits.set_set_type(CborSetType::Untagged);
        }
        Ok(wits)
    }
}