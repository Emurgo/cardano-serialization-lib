use crate::*;

impl Deserialize for ScriptRefEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "[id, native_or_putus_script]",
                    ))
                        .into());
                }
            }
            let script_ref = match raw.unsigned_integer()? {
                0 => ScriptRefEnum::NativeScript(NativeScript::deserialize(raw)?),
                1 => ScriptRefEnum::PlutusScript(PlutusScript::deserialize(raw)?),
                2 => ScriptRefEnum::PlutusScript(
                    PlutusScript::deserialize(raw)?.clone_as_version(&Language::new_plutus_v2()),
                ),
                3 => ScriptRefEnum::PlutusScript(
                    PlutusScript::deserialize(raw)?.clone_as_version(&Language::new_plutus_v3()),
                ),
                n => {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(n),
                        expected: Key::Uint(0),
                    }
                        .into())
                }
            };
            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }
            Ok(script_ref)
        })()
            .map_err(|e| e.annotate("ScriptRefEnum"))
    }
}

impl cbor_event::se::Serialize for ScriptRefEnum {
    fn serialize<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match &self {
            ScriptRefEnum::NativeScript(native_script) => {
                serializer.write_unsigned_integer(0)?;
                native_script.serialize(serializer)?;
            }
            ScriptRefEnum::PlutusScript(plutus_script) => {
                serializer.write_unsigned_integer(plutus_script.script_namespace() as u64)?;
                plutus_script.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for ScriptRef {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.tag()? {
                //bytes string tag
                24 => Ok(ScriptRef(from_bytes(&raw.bytes()?)?)),
                tag => {
                    return Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                        .into());
                }
            }
        })()
            .map_err(|e| e.annotate("ScriptRef"))
    }
}

impl cbor_event::se::Serialize for ScriptRef {
    fn serialize<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        let bytes = to_bytes(&self.0);
        serializer.write_tag(24)?.write_bytes(&bytes)?;
        Ok(serializer)
    }
}