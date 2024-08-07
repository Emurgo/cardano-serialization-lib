use crate::*;

impl cbor_event::se::Serialize for OperationalCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for OperationalCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.hot_vkey.serialize(serializer)?;
        self.sequence_number.serialize(serializer)?;
        self.kes_period.serialize(serializer)?;
        self.sigma.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for OperationalCert {
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
            .map_err(|e| e.annotate("OperationalCert"))
    }
}

impl DeserializeEmbeddedGroup for OperationalCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let hot_vkey = (|| -> Result<_, DeserializeError> { Ok(KESVKey::deserialize(raw)?) })()
            .map_err(|e| e.annotate("hot_vkey"))?;
        let sequence_number = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("sequence_number"))?;
        let kes_period = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("kes_period"))?;
        let sigma =
            (|| -> Result<_, DeserializeError> { Ok(Ed25519Signature::deserialize(raw)?) })()
                .map_err(|e| e.annotate("sigma"))?;
        Ok(OperationalCert {
            hot_vkey,
            sequence_number,
            kes_period,
            sigma,
        })
    }
}