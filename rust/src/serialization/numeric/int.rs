use crate::*;
use crate::serialization::utils::read_nint;

impl cbor_event::se::Serialize for Int {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.0 < 0 {
            serializer.write_negative_integer(self.0 as i64)
        } else {
            serializer.write_unsigned_integer(self.0 as u64)
        }
    }
}

impl Deserialize for Int {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::UnsignedInteger => Ok(Self(raw.unsigned_integer()? as i128)),
                cbor_event::Type::NegativeInteger => Ok(Self(read_nint(raw)?)),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
            .map_err(|e| e.annotate("Int"))
    }
}