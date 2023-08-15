use crate::*;

pub(super) const VOTE_CERT_INDEX: u64 = 9;

impl cbor_event::se::Serialize for VoteDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(VOTE_CERT_INDEX)?;
        self.stake_credential.serialize(serializer)?;
        self.drep.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for VoteDelegation {
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
        .map_err(|e| e.annotate("VoteDelegation"))
    }
}

impl DeserializeEmbeddedGroup for VoteDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        if let cbor_event::Len::Len(n) = len {
            if n != 3 {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    3,
                    len,
                    "(cert_index, stake_credential, drep)",
                ))
                .into());
            }
        }

        let cert_index = raw.unsigned_integer()?;
        if cert_index != VOTE_CERT_INDEX {
            return Err(DeserializeFailure::FixedValueMismatch {
                found: Key::Uint(cert_index),
                expected: Key::Uint(VOTE_CERT_INDEX),
            })
            .map_err(|e| DeserializeError::from(e).annotate("cert_index"));
        }

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let drep = DRep::deserialize(raw).map_err(|e| e.annotate("drep"))?;

        Ok(VoteDelegation {
            stake_credential,
            drep,
        })
    }
}
