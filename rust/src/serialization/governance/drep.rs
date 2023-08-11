use crate::*;

impl cbor_event::se::Serialize for DRep {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for DRep {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let drep_enum = DRepEnum::deserialize(raw)?;
            Ok(Self(drep_enum))
        })()
        .map_err(|e| e.annotate("DRep"))
    }
}

impl cbor_event::se::Serialize for DRepEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match &self {
            DRepEnum::KeyHash(keyhash) => {
                serializer.write_unsigned_integer(0u64)?;
                serializer.write_bytes(keyhash.to_bytes())
            }
            DRepEnum::ScriptHash(scripthash) => {
                serializer.write_unsigned_integer(1u64)?;
                serializer.write_bytes(scripthash.to_bytes())
            }
            DRepEnum::AlwaysAbstain => serializer.write_unsigned_integer(2u64),
            DRepEnum::AlwaysNoConfidence => serializer.write_unsigned_integer(3u64),
        }
    }
}

impl Deserialize for DRepEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 && n != 1 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "[id, hash] or [id] (for abstain and no confidence)",
                    ))
                    .into());
                }
            }
            let drep = match raw.unsigned_integer()? {
                0 => DRepEnum::KeyHash(Ed25519KeyHash::deserialize(raw)?),
                1 => DRepEnum::ScriptHash(ScriptHash::deserialize(raw)?),
                2 => DRepEnum::AlwaysAbstain,
                3 => DRepEnum::AlwaysNoConfidence,
                n => {
                    return Err(DeserializeFailure::FixedValuesMismatch {
                        found: Key::Uint(n),
                        expected: vec![Key::Uint(0), Key::Uint(1), Key::Uint(2), Key::Uint(3)],
                    }
                    .into())
                }
            };
            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }
            Ok(drep)
        })()
        .map_err(|e| e.annotate("DRepEnum"))
    }
}
