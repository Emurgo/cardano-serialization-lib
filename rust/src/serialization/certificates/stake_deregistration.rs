use crate::*;

pub(super) const DEREG_STAKE_CERT_INDEX: u64 = 1;

impl StakeDeregistration {
    pub(crate) const fn serialization_index() -> u64 {
        DEREG_STAKE_CERT_INDEX
    }
}

impl cbor_event::se::Serialize for StakeDeregistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for StakeDeregistration {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(DEREG_STAKE_CERT_INDEX)?;
        self.stake_credential.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeDeregistration {
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
        .map_err(|e| e.annotate("StakeDeregistration"))
    }
}

impl DeserializeEmbeddedGroup for StakeDeregistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let cert_index = raw.unsigned_integer()?;
            if cert_index != DEREG_STAKE_CERT_INDEX {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(DEREG_STAKE_CERT_INDEX),
                }
                .into());
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("cert_index"))?;
        let stake_credential =
            (|| -> Result<_, DeserializeError> { Ok(StakeCredential::deserialize(raw)?) })()
                .map_err(|e| e.annotate("stake_credential"))?;
        Ok(StakeDeregistration {
            stake_credential,
            coin: None,
        })
    }
}
