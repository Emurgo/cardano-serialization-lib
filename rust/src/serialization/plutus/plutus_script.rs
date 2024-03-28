use std::io::Read;
use crate::*;

impl cbor_event::se::Serialize for PlutusScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.bytes)
    }
}

impl Deserialize for PlutusScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self::new(raw.bytes()?))
    }
}

impl PlutusScript {
    pub(crate) fn deserialize_with_version<R: BufRead + Seek>(raw: &mut Deserializer<R>, version: &Language) -> Result<Self, DeserializeError> {
        Ok(Self::new_with_version(raw.bytes()?, version))
    }
}