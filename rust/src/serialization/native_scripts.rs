use crate::*;
use crate::serialization::utils::{is_break_tag, skip_set_tag};

impl cbor_event::se::Serialize for NativeScripts {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.len() as u64))?;
        for element in self {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl NativeScripts {
    pub(crate) fn serialize_as_set<'se, W: Write>(
        &self,
        need_deduplication: bool,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(258)?;
        if need_deduplication {
            let view = self.deduplicated_view();
            serializer.write_array(cbor_event::Len::Len(self.scripts.len() as u64))?;
            for element in view {
                element.serialize(serializer)?;
            }
        } else {
            serializer.write_array(cbor_event::Len::Len(self.len() as u64))?;
            for element in self {
                element.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for NativeScripts {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let has_tag = skip_set_tag(raw)?;
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "NativeScripts")? {
                    break;
                }
                arr.push(NativeScript::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("NativeScripts"))?;

        let set_type = if has_tag {
            CborSetType::Tagged
        } else {
            CborSetType::Untagged
        };

        Ok(
            Self {
                scripts: arr,
                cbor_tag_type: Some(set_type),
            }
        )
    }
}