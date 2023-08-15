use crate::*;

pub(super) const STAKE_VOTE_REG_DELEG_CERT_INDEX: u64 = 13;

impl cbor_event::se::Serialize for StakeVoteRegistrationAndDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(5))?;
        serializer.write_unsigned_integer(STAKE_VOTE_REG_DELEG_CERT_INDEX)?;
        self.stake_credential.serialize(serializer)?;
        self.pool_keyhash.serialize(serializer)?;
        self.drep.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeVoteRegistrationAndDelegation {
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
        .map_err(|e| e.annotate("StakeVoteRegistrationAndDelegation"))
    }
}

impl DeserializeEmbeddedGroup for StakeVoteRegistrationAndDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        if let cbor_event::Len::Len(n) = len {
            if n != 5 {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    5,
                    len,
                    "(cert_index, stake_credential, pool_keyhash, drep, coin)",
                ))
                .into());
            }
        }

        let cert_index = raw.unsigned_integer()?;
        if cert_index != STAKE_VOTE_REG_DELEG_CERT_INDEX {
            return Err(DeserializeFailure::FixedValueMismatch {
                found: Key::Uint(cert_index),
                expected: Key::Uint(STAKE_VOTE_REG_DELEG_CERT_INDEX),
            })
            .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
        }

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let pool_keyhash =
            Ed25519KeyHash::deserialize(raw).map_err(|e| e.annotate("pool_keyhash"))?;

        let drep = DRep::deserialize(raw).map_err(|e| e.annotate("drep"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        Ok(StakeVoteRegistrationAndDelegation {
            stake_credential,
            pool_keyhash,
            drep,
            coin,
        })
    }
}
