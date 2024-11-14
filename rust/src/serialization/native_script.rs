use std::io::SeekFrom;
use crate::*;

impl cbor_event::se::Serialize for NativeScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for NativeScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(NativeScriptEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for NativeScriptEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            NativeScriptEnum::ScriptPubkey(x) => x.serialize(serializer),
            NativeScriptEnum::ScriptAll(x) => x.serialize(serializer),
            NativeScriptEnum::ScriptAny(x) => x.serialize(serializer),
            NativeScriptEnum::ScriptNOfK(x) => x.serialize(serializer),
            NativeScriptEnum::TimelockStart(x) => x.serialize(serializer),
            NativeScriptEnum::TimelockExpiry(x) => x.serialize(serializer),
        }
    }
}

impl Deserialize for NativeScriptEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            //let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ScriptPubkey::deserialize_as_embedded_group(
                    raw, /*&mut read_len, */ len,
                )?)
            })(raw)
            {
                Ok(variant) => return Ok(NativeScriptEnum::ScriptPubkey(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ScriptAll::deserialize_as_embedded_group(
                    raw, /*mut read_len, */ len,
                )?)
            })(raw)
            {
                Ok(variant) => return Ok(NativeScriptEnum::ScriptAll(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ScriptAny::deserialize_as_embedded_group(
                    raw, /*mut read_len, */ len,
                )?)
            })(raw)
            {
                Ok(variant) => return Ok(NativeScriptEnum::ScriptAny(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ScriptNOfK::deserialize_as_embedded_group(
                    raw, /*mut read_len, */ len,
                )?)
            })(raw)
            {
                Ok(variant) => return Ok(NativeScriptEnum::ScriptNOfK(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(TimelockStart::deserialize_as_embedded_group(
                    raw, /*mut read_len, */ len,
                )?)
            })(raw)
            {
                Ok(variant) => return Ok(NativeScriptEnum::TimelockStart(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(TimelockExpiry::deserialize_as_embedded_group(
                    raw, /*mut read_len, */ len,
                )?)
            })(raw)
            {
                Ok(variant) => return Ok(NativeScriptEnum::TimelockExpiry(variant)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match len {
                cbor_event::Len::Len(_) => (), /*read_len.finish()?*/
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (), /*read_len.finish()?*/
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new(
                "NativeScriptEnum",
                DeserializeFailure::NoVariantMatched.into(),
            ))
        })()
            .map_err(|e| e.annotate("NativeScriptEnum"))
    }
}


impl cbor_event::se::Serialize for ScriptPubkey {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ScriptPubkey {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0u64)?;
        self.addr_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ScriptPubkey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, /*mut read_len, */ len);
            match len {
                cbor_event::Len::Len(_) => read_len.finish()?,
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("ScriptPubkey"))
    }
}

impl DeserializeEmbeddedGroup for ScriptPubkey {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        /*read_len: &mut CBORReadLen, */ _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(0),
                }
                    .into());
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("index_0"))?;
        let addr_keyhash =
            (|| -> Result<_, DeserializeError> { Ok(Ed25519KeyHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("addr_keyhash"))?;
        Ok(ScriptPubkey { addr_keyhash })
    }
}

impl cbor_event::se::Serialize for ScriptAll {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ScriptAll {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1u64)?;
        self.native_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ScriptAll {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, /*mut read_len, */ len);
            match len {
                cbor_event::Len::Len(_) => read_len.finish()?,
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("ScriptAll"))
    }
}

impl DeserializeEmbeddedGroup for ScriptAll {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        /*read_len: &mut CBORReadLen, */ _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(1),
                }
                    .into());
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("index_0"))?;
        let native_scripts =
            (|| -> Result<_, DeserializeError> { Ok(NativeScripts::deserialize(raw)?) })()
                .map_err(|e| e.annotate("native_scripts"))?;
        Ok(ScriptAll { native_scripts })
    }
}

impl cbor_event::se::Serialize for ScriptAny {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ScriptAny {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2u64)?;
        self.native_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ScriptAny {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, /*mut read_len, */ len);
            match len {
                cbor_event::Len::Len(_) => read_len.finish()?,
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("ScriptAny"))
    }
}

impl DeserializeEmbeddedGroup for ScriptAny {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        /*/*read_len: &mut CBORReadLen, */*/ _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(2),
                }
                    .into());
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("index_0"))?;
        let native_scripts =
            (|| -> Result<_, DeserializeError> { Ok(NativeScripts::deserialize(raw)?) })()
                .map_err(|e| e.annotate("native_scripts"))?;
        Ok(ScriptAny { native_scripts })
    }
}

impl cbor_event::se::Serialize for ScriptNOfK {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ScriptNOfK {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(3u64)?;
        self.n.serialize(serializer)?;
        self.native_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ScriptNOfK {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let ret = Self::deserialize_as_embedded_group(raw, /*mut read_len, */ len);
            match len {
                cbor_event::Len::Len(_) => read_len.finish()?,
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("ScriptNOfK"))
    }
}

impl DeserializeEmbeddedGroup for ScriptNOfK {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        /*read_len: &mut CBORReadLen, */ _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 3 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(3),
                }
                    .into());
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("index_0"))?;
        let n = (|| -> Result<_, DeserializeError> { Ok(u32::deserialize(raw)?) })()
            .map_err(|e| e.annotate("n"))?;
        let native_scripts =
            (|| -> Result<_, DeserializeError> { Ok(NativeScripts::deserialize(raw)?) })()
                .map_err(|e| e.annotate("native_scripts"))?;
        Ok(ScriptNOfK { n, native_scripts })
    }
}

impl cbor_event::se::Serialize for TimelockStart {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for TimelockStart {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(4u64)?;
        self.slot.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for TimelockStart {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, /*mut read_len, */ len);
            match len {
                cbor_event::Len::Len(_) => read_len.finish()?,
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("TimelockStart"))
    }
}

impl DeserializeEmbeddedGroup for TimelockStart {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        /*read_len: &mut CBORReadLen, */ _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 4 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(4),
                }
                    .into());
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("index_0"))?;
        let slot = (|| -> Result<_, DeserializeError> { Ok(SlotBigNum::deserialize(raw)?) })()
            .map_err(|e| e.annotate("slot"))?;
        Ok(TimelockStart { slot })
    }
}

impl cbor_event::se::Serialize for TimelockExpiry {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for TimelockExpiry {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(5u64)?;
        self.slot.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for TimelockExpiry {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, /*&mut read_len, */ len);
            match len {
                cbor_event::Len::Len(_) => read_len.finish()?,
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })()
            .map_err(|e| e.annotate("TimelockExpiry"))
    }
}

impl DeserializeEmbeddedGroup for TimelockExpiry {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        /*read_len: &mut CBORReadLen, */ _: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 5 {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(index_0_value),
                    expected: Key::Uint(5),
                }
                    .into());
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("index_0"))?;
        let slot = (|| -> Result<_, DeserializeError> { Ok(SlotBigNum::deserialize(raw)?) })()
            .map_err(|e| e.annotate("slot"))?;
        Ok(TimelockExpiry { slot })
    }
}