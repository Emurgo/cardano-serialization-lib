use crate::serialization::struct_checks::check_len;
use crate::*;

impl Serialize for Constitution {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.anchor.serialize(serializer)?;
        self.script_hash.serialize_nullable(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_tuple!(Constitution);

impl DeserializeEmbeddedGroup for Constitution {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 2, "(anchor, scripthash / null)")?;
        let anchor = Anchor::deserialize(raw)?;
        let script_hash = ScriptHash::deserialize_nullable(raw)?;

        Ok(Constitution {
            anchor,
            script_hash,
        })
    }
}
