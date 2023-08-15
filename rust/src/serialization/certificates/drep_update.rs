use crate::*;

pub(super) const UPDATE_DREP_CERT_INDEX: u64 = 18;

impl cbor_event::se::Serialize for DrepUpdate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(UPDATE_DREP_CERT_INDEX)?;
        self.voting_credential.serialize(serializer)?;
        match &self.anchor {
            Some(anchor) => anchor.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for DrepUpdate {
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
        .map_err(|e| e.annotate("DrepUpdate"))
    }
}

impl DeserializeEmbeddedGroup for DrepUpdate {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        if let cbor_event::Len::Len(n) = len {
            if n != 3 {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    3,
                    len,
                    "(cert_index, voting_credential, anchor / null)",
                ))
                .into());
            }
        }

        let cert_index = raw.unsigned_integer()?;
        if cert_index != UPDATE_DREP_CERT_INDEX {
            return Err(DeserializeFailure::FixedValueMismatch {
                found: Key::Uint(cert_index),
                expected: Key::Uint(UPDATE_DREP_CERT_INDEX),
            })
            .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
        }

        let voting_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("voting_credential"))?;

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

        Ok(DrepUpdate {
            voting_credential,
            anchor,
        })
    }
}
