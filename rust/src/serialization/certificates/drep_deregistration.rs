use crate::*;

pub(super) const DEREG_DREP_CERT_INDEX: u64 = 17;

impl cbor_event::se::Serialize for DrepDeregistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(DEREG_DREP_CERT_INDEX)?;
        self.voting_credential.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for DrepDeregistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;

            if let cbor_event::Len::Len(n) = len {
                if n != 3 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        3,
                        len,
                        "(cert_index, voting_credential, coin)",
                    ))
                        .into());
                }
            }

            let cert_index = raw.unsigned_integer()?;
            if cert_index != DEREG_DREP_CERT_INDEX {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(DEREG_DREP_CERT_INDEX),
                })
                    .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
            }

            let voting_credential =
                StakeCredential::deserialize(raw).map_err(|e| e.annotate("voting_credential"))?;

            let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            return Ok(DrepDeregistration {
                voting_credential,
                coin,
            });
        })()
            .map_err(|e| e.annotate("DrepDeregistration"))
    }
}
