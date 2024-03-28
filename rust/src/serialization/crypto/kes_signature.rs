use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use crate::{DeserializeError, DeserializeFailure, KESSignature};
use crate::protocol_types::Deserialize;

impl cbor_event::se::Serialize for KESSignature {
    fn serialize<'se, W: std::io::Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for KESSignature {
    fn deserialize<R: std::io::BufRead>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<Self, DeserializeError> {
            let bytes = raw.bytes()?;
            if bytes.len() != Self::BYTE_COUNT {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    Self::BYTE_COUNT as u64,
                    cbor_event::Len::Len(bytes.len() as u64),
                    "hash length",
                ))
                    .into());
            }
            Ok(KESSignature(bytes))
        })()
            .map_err(|e| e.annotate("KESSignature"))
    }
}