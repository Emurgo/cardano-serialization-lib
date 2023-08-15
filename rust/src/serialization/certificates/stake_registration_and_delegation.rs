use crate::*;

pub(super) const STAKE_REG_DELEG_CERT_INDEX: u64 = 11;

impl cbor_event::se::Serialize for StakeRegistrationAndDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_unsigned_integer(STAKE_REG_DELEG_CERT_INDEX)?;
        self.stake_credential.serialize(serializer)?;
        self.pool_keyhash.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeRegistrationAndDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;

            let cert = Self::deserialize_as_embedded_group(raw, len)?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            Ok(cert)
        })()
            .map_err(|e| e.annotate("StakeRegistrationAndDelegation"))
    }
}

impl DeserializeEmbeddedGroup for StakeRegistrationAndDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {

        if let cbor_event::Len::Len(n) = len {
            if n != 4 {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    4,
                    len,
                    "(cert_index, stake_credential, pool_keyhash, coin)",
                ))
                    .into());
            }
        }

        let cert_index = raw.unsigned_integer()?;
        if cert_index != STAKE_REG_DELEG_CERT_INDEX {
            return Err(DeserializeFailure::FixedValueMismatch {
                found: Key::Uint(cert_index),
                expected: Key::Uint(STAKE_REG_DELEG_CERT_INDEX),
            })
                .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
        }

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let pool_keyhash =
            Ed25519KeyHash::deserialize(raw).map_err(|e| e.annotate("pool_keyhash"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        Ok(StakeRegistrationAndDelegation {
            stake_credential,
            pool_keyhash,
            coin,
        })
    }
}