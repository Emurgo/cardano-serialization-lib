use crate::*;

pub(super) const UNREG_COMMITTEE_HOT_KEY_CERT_INDEX: u64 = 15;

impl cbor_event::se::Serialize for CommitteeHotKeyDeregistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(UNREG_COMMITTEE_HOT_KEY_CERT_INDEX)?;
        self.committee_cold_key.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for CommitteeHotKeyDeregistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;

            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "(cert_index, committee_cold_key)",
                    ))
                    .into());
                }
            }

            let cert_index = raw.unsigned_integer()?;
            if cert_index != UNREG_COMMITTEE_HOT_KEY_CERT_INDEX {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(UNREG_COMMITTEE_HOT_KEY_CERT_INDEX),
                })
                .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
            }

            let committee_cold_key = StakeCredential::deserialize(raw)
                .map_err(|e| e.annotate("committee_cold_key"))?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            return Ok(CommitteeHotKeyDeregistration {
                committee_cold_key,
            });
        })()
        .map_err(|e| e.annotate("CommitteeHotKeyDeregistration"))
    }
}
