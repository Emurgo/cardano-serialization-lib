use crate::*;

const REG_COMMITTEE_HOT_KEY_CERT: u64 = 14;

impl cbor_event::se::Serialize for CommitteeHotKeyRegistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(REG_COMMITTEE_HOT_KEY_CERT)?;
        self.committee_cold_keyhash.serialize(serializer)?;
        self.committee_hot_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for CommitteeHotKeyRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;

            if let cbor_event::Len::Len(n) = len {
                if n != 3 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        3,
                        len,
                        "(cert_index, committee_cold_keyhash, committee_hot_keyhash)",
                    ))
                    .into());
                }
            }

            let cert_index = raw.unsigned_integer()?;
            if cert_index != REG_COMMITTEE_HOT_KEY_CERT {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(REG_COMMITTEE_HOT_KEY_CERT),
                })
                .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
            }

            let committee_cold_keyhash = Ed25519KeyHash::deserialize(raw)
                .map_err(|e| e.annotate("committee_cold_keyhash"))?;

            let committee_hot_keyhash = Ed25519KeyHash::deserialize(raw)
                .map_err(|e| e.annotate("committee_hot_keyhash"))?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            return Ok(CommitteeHotKeyRegistration {
                committee_cold_keyhash,
                committee_hot_keyhash,
            });
        })()
        .map_err(|e| e.annotate("CommitteeHotKeyRegistration"))
    }
}
