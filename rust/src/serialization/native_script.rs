use crate::*;
use crate::serialization::utils::{check_len, check_len_indefinite, is_break_tag, skip_set_tag};
use crate::stack_safe::Remaining;

impl cbor_event::se::Serialize for NativeScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serialize_native_script_tree(self, serializer)
    }
}

fn serialize_native_script_tree<'se, W: Write>(
    root: &NativeScript,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    let mut stack: Vec<std::slice::Iter<'_, NativeScript>> = Vec::new();
    let mut current = root;
    loop {
        write_native_script_header(current, serializer)?;
        if let Some(children) = native_script_children(current) {
            serializer.write_array(cbor_event::Len::Len(children.len() as u64))?;
            let mut rest = children.iter();
            if let Some(first) = rest.next() {
                stack.push(rest);
                current = first;
                continue;
            }
        }
        loop {
            match stack.last_mut() {
                None => return Ok(serializer),
                Some(rest) => {
                    if let Some(next) = rest.next() {
                        current = next;
                        break;
                    } else {
                        stack.pop();
                    }
                }
            }
        }
    }
}

fn native_script_children(script: &NativeScript) -> Option<&[NativeScript]> {
    match &script.0 {
        NativeScriptEnum::ScriptAll(x) => Some(x.native_scripts.scripts.as_slice()),
        NativeScriptEnum::ScriptAny(x) => Some(x.native_scripts.scripts.as_slice()),
        NativeScriptEnum::ScriptNOfK(x) => Some(x.native_scripts.scripts.as_slice()),
        _ => None,
    }
}

fn write_native_script_header<'se, W: Write>(
    script: &NativeScript,
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    match &script.0 {
        NativeScriptEnum::ScriptPubkey(x) => {
            serializer.write_array(cbor_event::Len::Len(2))?;
            serializer.write_unsigned_integer(0u64)?;
            x.addr_keyhash.serialize(serializer)?;
        }
        NativeScriptEnum::ScriptAll(_) => {
            serializer.write_array(cbor_event::Len::Len(2))?;
            serializer.write_unsigned_integer(1u64)?;
        }
        NativeScriptEnum::ScriptAny(_) => {
            serializer.write_array(cbor_event::Len::Len(2))?;
            serializer.write_unsigned_integer(2u64)?;
        }
        NativeScriptEnum::ScriptNOfK(x) => {
            serializer.write_array(cbor_event::Len::Len(3))?;
            serializer.write_unsigned_integer(3u64)?;
            x.n.serialize(serializer)?;
        }
        NativeScriptEnum::TimelockStart(x) => {
            serializer.write_array(cbor_event::Len::Len(2))?;
            serializer.write_unsigned_integer(4u64)?;
            x.slot.serialize(serializer)?;
        }
        NativeScriptEnum::TimelockExpiry(x) => {
            serializer.write_array(cbor_event::Len::Len(2))?;
            serializer.write_unsigned_integer(5u64)?;
            x.slot.serialize(serializer)?;
        }
    }
    Ok(serializer)
}

impl Deserialize for NativeScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        deserialize_native_script_tree(raw)
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
        Ok(NativeScript::deserialize(raw)?.into_enum())
    }
}

enum NativeScriptKindFrame {
    All,
    Any,
    NOfK { n: u32 },
}

struct NativeScriptFrame {
    kind: NativeScriptKindFrame,
    remaining: Remaining,
    acc: Vec<NativeScript>,
    set_type: Option<CborSetType>,
    outer_len: cbor_event::Len,
}

fn deserialize_native_script_tree<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<NativeScript, DeserializeError> {
    (|| -> Result<_, DeserializeError> {
        let mut stack: Vec<NativeScriptFrame> = Vec::new();
        loop {
            let produced = match deserialize_native_script_node(raw)? {
                NativeNode::Complete(script) => script,
                NativeNode::Open(frame) => {
                    if children_finished(raw, &frame.remaining)? {
                        finish_native_script_frame(raw, frame)?
                    } else {
                        stack.push(frame);
                        continue;
                    }
                }
            };
            let mut completed = produced;
            loop {
                match stack.last_mut() {
                    None => return Ok(completed),
                    Some(frame) => {
                        frame.acc.push(completed);
                        frame.remaining.decrement();
                        if children_finished(raw, &frame.remaining)? {
                            let frame = stack.pop().unwrap();
                            completed = finish_native_script_frame(raw, frame)?;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    })()
    .map_err(|e: DeserializeError| e.annotate("NativeScriptEnum"))
}

enum NativeNode {
    Complete(NativeScript),
    Open(NativeScriptFrame),
}

fn children_finished<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    remaining: &Remaining,
) -> Result<bool, DeserializeError> {
    match remaining {
        Remaining::Definite(0) => Ok(true),
        Remaining::Definite(_) => Ok(false),
        Remaining::Indefinite => is_break_tag(raw, "NativeScripts"),
    }
}

fn deserialize_native_script_node<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<NativeNode, DeserializeError> {
    let outer_len = raw.array()?;
    let tag = raw.unsigned_integer()?;
    match tag {
        0 => {
            check_len(outer_len, 2, "[script_pubkey]")?;
            let addr_keyhash = Ed25519KeyHash::deserialize(raw)?;
            check_len_indefinite(raw, outer_len)?;
            Ok(NativeNode::Complete(NativeScript(
                NativeScriptEnum::ScriptPubkey(ScriptPubkey { addr_keyhash }),
            )))
        }
        1 => {
            check_len(outer_len, 2, "[script_all]")?;
            Ok(NativeNode::Open(open_scripts_frame(
                raw,
                NativeScriptKindFrame::All,
                outer_len,
            )?))
        }
        2 => {
            check_len(outer_len, 2, "[script_any]")?;
            Ok(NativeNode::Open(open_scripts_frame(
                raw,
                NativeScriptKindFrame::Any,
                outer_len,
            )?))
        }
        3 => {
            check_len(outer_len, 3, "[script_n_of_k]")?;
            let n = u32::deserialize(raw)?;
            Ok(NativeNode::Open(open_scripts_frame(
                raw,
                NativeScriptKindFrame::NOfK { n },
                outer_len,
            )?))
        }
        4 => {
            check_len(outer_len, 2, "[timelock_start]")?;
            let slot = SlotBigNum::deserialize(raw)?;
            check_len_indefinite(raw, outer_len)?;
            Ok(NativeNode::Complete(NativeScript(
                NativeScriptEnum::TimelockStart(TimelockStart { slot }),
            )))
        }
        5 => {
            check_len(outer_len, 2, "[timelock_expiry]")?;
            let slot = SlotBigNum::deserialize(raw)?;
            check_len_indefinite(raw, outer_len)?;
            Ok(NativeNode::Complete(NativeScript(
                NativeScriptEnum::TimelockExpiry(TimelockExpiry { slot }),
            )))
        }
        n => Err(DeserializeFailure::FixedValuesMismatch {
            found: Key::Uint(n),
            expected: vec![
                Key::Uint(0),
                Key::Uint(1),
                Key::Uint(2),
                Key::Uint(3),
                Key::Uint(4),
                Key::Uint(5),
            ],
        }
        .into()),
    }
}

fn open_scripts_frame<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    kind: NativeScriptKindFrame,
    outer_len: cbor_event::Len,
) -> Result<NativeScriptFrame, DeserializeError> {
    let has_tag = skip_set_tag(raw)?;
    let scripts_len = raw.array()?;
    let set_type = Some(if has_tag {
        CborSetType::Tagged
    } else {
        CborSetType::Untagged
    });
    Ok(NativeScriptFrame {
        kind,
        remaining: Remaining::from_cbor_len(scripts_len),
        acc: Vec::new(),
        set_type,
        outer_len,
    })
}

fn finish_native_script_frame<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
    frame: NativeScriptFrame,
) -> Result<NativeScript, DeserializeError> {
    check_len_indefinite(raw, frame.outer_len)?;
    let native_scripts = NativeScripts::from_vec_and_tag(frame.acc, frame.set_type);
    Ok(match frame.kind {
        NativeScriptKindFrame::All => NativeScript::from_script_all_moved(native_scripts),
        NativeScriptKindFrame::Any => NativeScript::from_script_any_moved(native_scripts),
        NativeScriptKindFrame::NOfK { n } => {
            NativeScript::from_script_n_of_k_moved(n, native_scripts)
        }
    })
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