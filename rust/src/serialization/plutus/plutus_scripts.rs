use crate::*;
use crate::serialization::utils::{is_break_tag, skip_set_tag};

impl cbor_event::se::Serialize for PlutusScripts {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl PlutusScripts {
    pub(crate) fn serialize_by_version<'se, W: Write>(
        &self,
        version: &Language,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let view = self.view(version);
        serializer.write_array(cbor_event::Len::Len(view.len() as u64))?;
        for element in view {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }

    pub(crate) fn serialize_as_set_by_version<'se, W: Write>(
        &self,
        need_deduplication: bool,
        version: &Language,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        //TODO: uncomment this line when we conway ero will come
        //serializer.write_tag(258)?;
        let view = match need_deduplication {
            true => self.deduplicated_view(Some(version)),
            false => self.view(version),
        };
        serializer.write_array(cbor_event::Len::Len(view.len() as u64))?;
        for element in view {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }

}

impl Deserialize for PlutusScripts {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        skip_set_tag(raw)?;
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            skip_set_tag(raw)?;
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "PlutusScripts")? {
                    break;
                }
                arr.push(PlutusScript::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("PlutusScripts"))?;
        Ok(Self(arr))
    }
}

impl PlutusScripts {
    pub(crate) fn deserialize_with_version<R: BufRead + Seek>(raw: &mut Deserializer<R>, version: &Language) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            skip_set_tag(raw)?;
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "PlutusScripts")? {
                    break;
                }
                arr.push(PlutusScript::deserialize_with_version(raw, version)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("PlutusScripts"))?;
        Ok(Self(arr))
    }
}