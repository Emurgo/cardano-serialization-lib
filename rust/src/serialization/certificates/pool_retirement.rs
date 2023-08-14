use crate::*;

const RETIRE_POOL_CERT_INDEX: u64 = 4;

impl PoolRetirement {
    pub(crate) const fn serialization_index() -> u64 {
        RETIRE_POOL_CERT_INDEX
    }
}

impl cbor_event::se::Serialize for PoolRetirement {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for PoolRetirement {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(RETIRE_POOL_CERT_INDEX)?;
        self.pool_keyhash.serialize(serializer)?;
        self.epoch.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for PoolRetirement {
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
        .map_err(|e| e.annotate("PoolRetirement"))
    }
}

impl DeserializeEmbeddedGroup for PoolRetirement {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let cert_index = raw.unsigned_integer()?;
            if cert_index != RETIRE_POOL_CERT_INDEX {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(RETIRE_POOL_CERT_INDEX),
                }
                .into());
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("cert_index"))?;
        let pool_keyhash =
            (|| -> Result<_, DeserializeError> { Ok(Ed25519KeyHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("pool_keyhash"))?;
        let epoch = (|| -> Result<_, DeserializeError> { Ok(Epoch::deserialize(raw)?) })()
            .map_err(|e| e.annotate("epoch"))?;
        Ok(PoolRetirement {
            pool_keyhash,
            epoch,
        })
    }
}
