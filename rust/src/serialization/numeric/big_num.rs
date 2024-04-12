use crate::*;

impl Serialize for BigNum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(self.0)
    }
}

impl Deserialize for BigNum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        match raw.unsigned_integer() {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(DeserializeError::new("BigNum", DeserializeFailure::CBOR(e))),
        }
    }
}