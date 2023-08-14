use crate::*;

const VOTE_REG_DELEG_CERT_INDEX: u64 = 12;

impl VoteRegistrationAndDelegation {
    pub(crate) const fn serialization_index() -> u64 {
        VOTE_REG_DELEG_CERT_INDEX
    }
}

impl cbor_event::se::Serialize for VoteRegistrationAndDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_unsigned_integer(VOTE_REG_DELEG_CERT_INDEX)?;
        self.stake_credential.serialize(serializer)?;
        self.drep.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for VoteRegistrationAndDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;

            if let cbor_event::Len::Len(n) = len {
                if n != 4 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        4,
                        len,
                        "(cert_index, stake_credential, drep, coin)",
                    ))
                        .into());
                }
            }

            let cert_index = raw.unsigned_integer()?;
            if cert_index != VOTE_REG_DELEG_CERT_INDEX {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(VOTE_REG_DELEG_CERT_INDEX),
                })
                    .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
            }

            let stake_credential =
                StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

            let drep = DRep::deserialize(raw).map_err(|e| e.annotate("drep"))?;

            let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            return Ok(VoteRegistrationAndDelegation {
                stake_credential,
                drep,
                coin,
            });
        })()
            .map_err(|e| e.annotate("VoteRegistrationAndDelegation"))
    }
}