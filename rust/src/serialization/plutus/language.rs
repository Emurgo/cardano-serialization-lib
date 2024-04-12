use crate::*;

impl cbor_event::se::Serialize for Language {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        // https://github.com/input-output-hk/cardano-ledger/blob/master/eras/babbage/test-suite/cddl-files/babbage.cddl#L324-L327
        serializer.write_unsigned_integer(self.kind() as u64)
    }
}

impl Deserialize for Language {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match LanguageKind::from_u64(raw.unsigned_integer()?) {
                Some(kind) => Ok(Language(kind)),
                _ => Err(DeserializeError::new(
                    "Language",
                    DeserializeFailure::NoVariantMatched.into(),
                )),
            }
        })()
            .map_err(|e| e.annotate("Language"))
    }
}