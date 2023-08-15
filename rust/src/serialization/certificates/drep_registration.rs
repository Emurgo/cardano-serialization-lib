use crate::*;

pub(super) const REG_DREP_CERT_INDEX: u64 = 16;

impl cbor_event::se::Serialize for DrepRegistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_unsigned_integer(REG_DREP_CERT_INDEX)?;
        self.voting_credential.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        match &self.anchor {
            Some(anchor) => anchor.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for DrepRegistration {
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
        .map_err(|e| e.annotate("DrepRegistration"))
    }
}

impl DeserializeEmbeddedGroup for DrepRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        if let cbor_event::Len::Len(n) = len {
            if n != 4 {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    4,
                    len,
                    "(cert_index, voting_credential, coin, anchor / null)",
                ))
                .into());
            }
        }

        let cert_index = raw.unsigned_integer()?;
        if cert_index != REG_DREP_CERT_INDEX {
            return Err(DeserializeFailure::FixedValueMismatch {
                found: Key::Uint(cert_index),
                expected: Key::Uint(REG_DREP_CERT_INDEX),
            })
            .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
        }

        let voting_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("voting_credential"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        let anchor = (|| -> Result<_, DeserializeError> {
            if raw.cbor_type()? == CBORType::Special {
                if raw.special()? != CBORSpecial::Null {
                    return Err(DeserializeFailure::ExpectedNull.into());
                }
                Ok(None)
            } else {
                Ok(Some(Anchor::deserialize(raw)?))
            }
        })()
        .map_err(|e| e.annotate("anchor"))?;

        Ok(DrepRegistration {
            voting_credential,
            coin,
            anchor,
        })
    }
}
