use cbor_event::Len;
use crate::*;

pub(super) const DEREG_STAKE_CERT_LEGACY_INDEX: u64 = 1;
pub(super) const DEREG_STAKE_CERT_CONWAY_INDEX: u64 = 8;

impl cbor_event::se::Serialize for StakeDeregistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.coin.is_some() {
            serialize_as_conway(self, serializer)
        } else {
            serialize_as_legacy(self, serializer)
        }
    }
}

fn serialize_as_legacy<'se, W: Write>(
    cert: &StakeDeregistration,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    serializer.write_array(cbor_event::Len::Len(2))?;
    serializer.write_unsigned_integer(DEREG_STAKE_CERT_LEGACY_INDEX)?;
    cert.stake_credential.serialize(serializer)?;
    Ok(serializer)
}

fn serialize_as_conway<'se, W: Write>(
    cert: &StakeDeregistration,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    serializer.write_array(cbor_event::Len::Len(3))?;
    serializer.write_unsigned_integer(DEREG_STAKE_CERT_CONWAY_INDEX)?;
    cert.stake_credential.serialize(serializer)?;
    if let Some(coin) = cert.coin {
        coin.serialize(serializer)?;
    }
    Ok(serializer)
}

impl Deserialize for StakeDeregistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => {}
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
                _ => {}
            }
            ret
        })()
        .map_err(|e| e.annotate("StakeDeregistration"))
    }
}

impl DeserializeEmbeddedGroup for StakeDeregistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: Len,
    ) -> Result<Self, DeserializeError> {
        let cert_index = raw.unsigned_integer()?;
        match cert_index {
            DEREG_STAKE_CERT_LEGACY_INDEX => deserialize_legacy(raw, cert_index, len),
            DEREG_STAKE_CERT_CONWAY_INDEX => deserialize_conway(raw, cert_index, len),
            _ => Err(DeserializeFailure::FixedValuesMismatch {
                found: Key::Uint(cert_index),
                expected: vec![
                    Key::Uint(DEREG_STAKE_CERT_LEGACY_INDEX),
                    Key::Uint(DEREG_STAKE_CERT_CONWAY_INDEX),
                ],
            })
                .map_err(|e| DeserializeError::from(e).annotate("cert_index")),
        }
    }
}

fn deserialize_legacy<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    cert_index: u64,
    len: Len,
) -> Result<StakeDeregistration, DeserializeError> {
    (|| -> Result<_, DeserializeError> {
        if let Len::Len(n) = len {
            if n != 2 {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    2,
                    len,
                    "(cert_index, stake_credential)",
                ))
                    .into());
            }
        }

        if cert_index != DEREG_STAKE_CERT_LEGACY_INDEX {
            return Err(DeserializeFailure::FixedValueMismatch {
                found: Key::Uint(cert_index),
                expected: Key::Uint(DEREG_STAKE_CERT_LEGACY_INDEX),
            })
                .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
        }

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        return Ok(StakeDeregistration {
            stake_credential,
            coin: None,
        });
    })()
        .map_err(|e| e.annotate("StakeDeregistration (legacy)"))
}

fn deserialize_conway<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    cert_index: u64,
    len: Len,
) -> Result<StakeDeregistration, DeserializeError> {
    (|| -> Result<_, DeserializeError> {
        if let Len::Len(n) = len {
            if n != 3 {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    3,
                    len,
                    "(cert_index, stake_credential, coin)",
                ))
                    .into());
            }
        }

        if cert_index != DEREG_STAKE_CERT_CONWAY_INDEX {
            return Err(DeserializeFailure::FixedValueMismatch {
                found: Key::Uint(cert_index),
                expected: Key::Uint(DEREG_STAKE_CERT_CONWAY_INDEX),
            })
                .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
        }

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        return Ok(StakeDeregistration {
            stake_credential,
            coin: Some(coin),
        });
    })()
        .map_err(|e| e.annotate("StakeDeregistration (conway)"))
}

