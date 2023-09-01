use crate::serialization::struct_checks::check_len;
use crate::*;
use std::collections::BTreeMap;

impl Serialize for Committee {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.quorum_threshold.serialize(serializer)?;
        serializer.write_map(cbor_event::Len::Len(self.members.len() as u64))?;
        for (key, value) in &self.members {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(Committee);

impl DeserializeEmbeddedGroup for Committee {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 2, "(quorum_threshold, members)")?;
        let quorum_threshold = UnitInterval::deserialize(raw)?;

        let mut table = BTreeMap::new();
        let map_len = raw.map()?;
        while match map_len {
            cbor_event::Len::Len(n) => table.len() < n as usize,
            cbor_event::Len::Indefinite => true,
        } {
            if raw.cbor_type()? == CBORType::Special {
                assert_eq!(raw.special()?, CBORSpecial::Break);
                break;
            }
            let key = Credential::deserialize(raw)?;
            let value = Epoch::deserialize(raw)?;
            if table.insert(key.clone(), value).is_some() {
                return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                    "some complicated/unsupported type",
                )))
                .into());
            }
        }
        Ok(Committee {
            quorum_threshold,
            members: table,
        })
    }
}
