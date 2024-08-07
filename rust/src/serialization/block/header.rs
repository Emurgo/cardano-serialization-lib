use crate::*;

impl cbor_event::se::Serialize for Header {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.header_body.serialize(serializer)?;
        self.body_signature.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Header {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) =>
                /* TODO: check finite len somewhere */
                    {
                        ()
                    }
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break =>
                    /* it's ok */
                        {
                            ()
                        }
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("Header"))
    }
}

impl DeserializeEmbeddedGroup for Header {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let header_body =
            (|| -> Result<_, DeserializeError> { Ok(HeaderBody::deserialize(raw)?) })()
                .map_err(|e| e.annotate("header_body"))?;
        let body_signature =
            (|| -> Result<_, DeserializeError> { Ok(KESSignature::deserialize(raw)?) })()
                .map_err(|e| e.annotate("body_signature"))?;
        Ok(Header {
            header_body,
            body_signature,
        })
    }
}