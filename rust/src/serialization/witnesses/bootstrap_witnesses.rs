use std::io::{BufRead, Seek, Write};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::{BootstrapWitness, BootstrapWitnesses, CborSetType, DeserializeError};
use crate::protocol_types::Deserialize;
use crate::serialization::utils::skip_set_tag;

impl cbor_event::se::Serialize for BootstrapWitnesses {
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
        serializer.write_array(cbor_event::Len::Len(self.get_vec_wits().len() as u64))?;
        for element in self.get_vec_wits() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for BootstrapWitnesses {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let has_set_tag = skip_set_tag(raw)?;
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                arr.push(BootstrapWitness::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("BootstrapWitnesses"))?;

        let mut witnesses = Self::from_vec(arr);
        if has_set_tag {
            witnesses.set_set_type(CborSetType::Tagged);
        } else {
            witnesses.set_set_type(CborSetType::Untagged);
        }

        Ok(witnesses)
    }
}