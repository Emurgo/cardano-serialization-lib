use crate::*;

pub(super) const STAKE_DELEGATION_CERT_INDEX: u64 = 2;

impl cbor_event::se::Serialize for StakeDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for StakeDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(STAKE_DELEGATION_CERT_INDEX)?;
        self.stake_credential.serialize(serializer)?;
        self.pool_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeDelegation {
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
        .map_err(|e| e.annotate("StakeDelegation"))
    }
}

impl DeserializeEmbeddedGroup for StakeDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let cert_index = raw.unsigned_integer()?;
            if cert_index != STAKE_DELEGATION_CERT_INDEX {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(STAKE_DELEGATION_CERT_INDEX),
                }
                .into());
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("cert_index"))?;
        let stake_credential =
            (|| -> Result<_, DeserializeError> { Ok(StakeCredential::deserialize(raw)?) })()
                .map_err(|e| e.annotate("stake_credential"))?;
        let pool_keyhash =
            (|| -> Result<_, DeserializeError> { Ok(Ed25519KeyHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("pool_keyhash"))?;
        Ok(StakeDelegation {
            stake_credential,
            pool_keyhash,
        })
    }
}
