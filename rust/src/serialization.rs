use super::*;
use address::*;
use std::io::{Seek, SeekFrom};

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

impl cbor_event::se::Serialize for UnitInterval {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag(30u64)?;
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for UnitInterval {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Primitive(U32)
        self.index_0.serialize(serializer)?;
        // DEBUG - generated from: Primitive(U32)
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for UnitInterval {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let tag = raw.tag()?;
            if tag != 30 {
                return Err(DeserializeError::new("UnitInterval", DeserializeFailure::TagMismatch{ found: tag, expected: 30 }));
            }
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("UnitInterval"))
    }
}

impl DeserializeEmbeddedGroup for UnitInterval {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let index_0 = (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("index_0"))?;
        let index_1 = (|| -> Result<_, DeserializeError> {
            println!("deserializing index_1");
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("index_1"))?;
        Ok(UnitInterval::new(index_0, index_1))
    }
}

impl cbor_event::se::Serialize for Vkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Primitive(Bytes)
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for Vkey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        println!("deserializing ");
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for Signature {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Primitive(Bytes)
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for Signature {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        println!("deserializing ");
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for Vkeywitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for Vkeywitness {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Rust(RustIdent("Vkey"))
        self.vkey.serialize(serializer)?;
        // DEBUG - generated from: Rust(RustIdent("Signature"))
        self.signature.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Vkeywitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Vkeywitness"))
    }
}

impl DeserializeEmbeddedGroup for Vkeywitness {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let vkey = (|| -> Result<_, DeserializeError> {
            println!("deserializing vkey");
            Ok(Vkey::deserialize(raw)?)
        })().map_err(|e| e.annotate("vkey"))?;
        let signature = (|| -> Result<_, DeserializeError> {
            println!("deserializing signature");
            Ok(Signature::deserialize(raw)?)
        })().map_err(|e| e.annotate("signature"))?;
        Ok(Vkeywitness::new(vkey, signature))
    }
}

impl cbor_event::se::Serialize for MsigPubkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MsigPubkey {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(0))
        serializer.write_unsigned_integer(0u64)?;
        // DEBUG - generated from: Rust(RustIdent("Hash28"))
        self.addr_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MsigPubkey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MsigPubkey"))
    }
}

impl DeserializeEmbeddedGroup for MsigPubkey {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let addr_keyhash = (|| -> Result<_, DeserializeError> {
            println!("deserializing addr_keyhash");
            Ok(AddrKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("addr_keyhash"))?;
        Ok(MsigPubkey::new(addr_keyhash))
    }
}

impl cbor_event::se::Serialize for MultisigScripts {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MultisigScripts {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing MultisigScripts");
                arr.push(MultisigScript::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("MultisigScripts"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for MsigAll {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MsigAll {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(1))
        serializer.write_unsigned_integer(1u64)?;
        // DEBUG - generated from: Array(Rust(RustIdent("MultisigScript")))
        self.multisig_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MsigAll {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MsigAll"))
    }
}

impl DeserializeEmbeddedGroup for MsigAll {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let multisig_scripts = (|| -> Result<_, DeserializeError> {
            println!("deserializing multisig_scripts");
            Ok(MultisigScripts::deserialize(raw)?)
        })().map_err(|e| e.annotate("multisig_scripts"))?;
        Ok(MsigAll::new(multisig_scripts))
    }
}

impl cbor_event::se::Serialize for MsigAny {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MsigAny {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(2))
        serializer.write_unsigned_integer(2u64)?;
        // DEBUG - generated from: Array(Rust(RustIdent("MultisigScript")))
        self.multisig_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MsigAny {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MsigAny"))
    }
}

impl DeserializeEmbeddedGroup for MsigAny {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let multisig_scripts = (|| -> Result<_, DeserializeError> {
            println!("deserializing multisig_scripts");
            Ok(MultisigScripts::deserialize(raw)?)
        })().map_err(|e| e.annotate("multisig_scripts"))?;
        Ok(MsigAny::new(multisig_scripts))
    }
}

impl cbor_event::se::Serialize for MsigNOfK {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MsigNOfK {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(3))
        serializer.write_unsigned_integer(3u64)?;
        // DEBUG - generated from: Primitive(U32)
        self.n.serialize(serializer)?;
        // DEBUG - generated from: Array(Rust(RustIdent("MultisigScript")))
        self.multisig_scripts.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MsigNOfK {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MsigNOfK"))
    }
}

impl DeserializeEmbeddedGroup for MsigNOfK {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 3 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(3) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let n = (|| -> Result<_, DeserializeError> {
            println!("deserializing n");
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("n"))?;
        let multisig_scripts = (|| -> Result<_, DeserializeError> {
            println!("deserializing multisig_scripts");
            Ok(MultisigScripts::deserialize(raw)?)
        })().map_err(|e| e.annotate("multisig_scripts"))?;
        Ok(MsigNOfK::new(n, multisig_scripts))
    }
}

impl cbor_event::se::Serialize for MultisigScriptEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MultisigScriptEnum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            MultisigScriptEnum::MsigPubkey(x) => {
                // DEBUG - generated from: Rust(RustIdent("MsigPubkey"))
                x.serialize(serializer)
            },
            MultisigScriptEnum::MsigAll(x) => {
                // DEBUG - generated from: Rust(RustIdent("MsigAll"))
                x.serialize(serializer)
            },
            MultisigScriptEnum::MsigAny(x) => {
                // DEBUG - generated from: Rust(RustIdent("MsigAny"))
                x.serialize(serializer)
            },
            MultisigScriptEnum::MsigNOfK(x) => {
                // DEBUG - generated from: Rust(RustIdent("MsigNOfK"))
                x.serialize(serializer)
            },
        }
    }
}

impl Deserialize for MultisigScriptEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MultisigScriptEnum"))
    }
}

impl DeserializeEmbeddedGroup for MultisigScriptEnum {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing msig_pubkey");
            Ok(MsigPubkey::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(MultisigScriptEnum::MsigPubkey(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing msig_all");
            Ok(MsigAll::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(MultisigScriptEnum::MsigAll(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing msig_any");
            Ok(MsigAny::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(MultisigScriptEnum::MsigAny(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing msig_n_of_k");
            Ok(MsigNOfK::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(MultisigScriptEnum::MsigNOfK(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        Err(DeserializeError::new("MultisigScriptEnum", DeserializeFailure::NoVariantMatched.into()))
    }
}

impl cbor_event::se::Serialize for MultisigScript {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MultisigScript {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize_as_embedded_group(serializer)
    }
}

impl Deserialize for MultisigScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(MultisigScriptEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for TransactionInput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionInput {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Rust(RustIdent("Hash32"))
        self.transaction_id.serialize(serializer)?;
        // DEBUG - generated from: Primitive(U32)
        self.index.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for TransactionInput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("TransactionInput"))
    }
}

impl DeserializeEmbeddedGroup for TransactionInput {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let transaction_id = (|| -> Result<_, DeserializeError> {
            println!("deserializing transaction_id");
            Ok(TransactionHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("transaction_id"))?;
        let index = (|| -> Result<_, DeserializeError> {
            println!("deserializing index");
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("index"))?;
        Ok(TransactionInput::new(transaction_id, index))
    }
}

impl cbor_event::se::Serialize for TransactionOutput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionOutput {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Rust(RustIdent("Address"))
        self.address.serialize(serializer)?;
        // DEBUG - generated from: Primitive(U32)
        self.amount.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for TransactionOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("TransactionOutput"))
    }
}

impl DeserializeEmbeddedGroup for TransactionOutput {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let address = (|| -> Result<_, DeserializeError> {
            println!("deserializing address");
            Ok(Address::deserialize(raw)?)
        })().map_err(|e| e.annotate("address"))?;
        let amount = (|| -> Result<_, DeserializeError> {
            println!("deserializing amount");
            Ok(Coin::deserialize(raw)?)
        })().map_err(|e| e.annotate("amount"))?;
        Ok(TransactionOutput::new(address, amount))
    }
}

impl cbor_event::se::Serialize for Withdrawals {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for Withdrawals {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        for (key, value) in &self.0 {
            // DEBUG - generated from: Rust(RustIdent("StakeCredential"))
            key.serialize(serializer)?;
            // DEBUG - generated from: Primitive(U32)
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Withdrawals {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing key");
                let key = StakeCredential::deserialize(raw)?;
                println!("deserializing value");
                let value = u32::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("Withdrawals"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for MoveInstantaneousReward {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MoveInstantaneousReward {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        for (key, value) in &self.0 {
            // DEBUG - generated from: Rust(RustIdent("StakeCredential"))
            key.serialize(serializer)?;
            // DEBUG - generated from: Primitive(U32)
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MoveInstantaneousReward {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing key");
                let key = StakeCredential::deserialize(raw)?;
                println!("deserializing value");
                let value = u32::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("MoveInstantaneousReward"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for SingleHostAddr {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for SingleHostAddr {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(0))
        serializer.write_unsigned_integer(0u64)?;
        // DEBUG - generated from: Optional(Rust(RustIdent("Port")))
        match &self.port {
            Some(x) => {
                // DEBUG - generated from: Rust(RustIdent("Port"))
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        // DEBUG - generated from: Optional(Rust(RustIdent("Ipv4")))
        match &self.ipv4 {
            Some(x) => {
                // DEBUG - generated from: Rust(RustIdent("Ipv4"))
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        // DEBUG - generated from: Optional(Rust(RustIdent("Ipv6")))
        match &self.ipv6 {
            Some(x) => {
                // DEBUG - generated from: Rust(RustIdent("Ipv6"))
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for SingleHostAddr {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SingleHostAddr"))
    }
}

impl DeserializeEmbeddedGroup for SingleHostAddr {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let port = (|| -> Result<_, DeserializeError> {
            println!("deserializing port");
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    println!("deserializing port");
                    Some(Port::deserialize(raw)?)
                },
                false => {
                    println!("deserializing port");
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("port"))?;
        let ipv4 = (|| -> Result<_, DeserializeError> {
            println!("deserializing ipv4");
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    println!("deserializing ipv4");
                    Some(Ipv4::deserialize(raw)?)
                },
                false => {
                    println!("deserializing ipv4");
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("ipv4"))?;
        let ipv6 = (|| -> Result<_, DeserializeError> {
            println!("deserializing ipv6");
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    println!("deserializing ipv6");
                    Some(Ipv6::deserialize(raw)?)
                },
                false => {
                    println!("deserializing ipv6");
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("ipv6"))?;
        Ok(SingleHostAddr::new(port, ipv4, ipv6))
    }
}

impl cbor_event::se::Serialize for SingleHostName {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for SingleHostName {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(1))
        serializer.write_unsigned_integer(1u64)?;
        // DEBUG - generated from: Optional(Rust(RustIdent("Port")))
        match &self.port {
            Some(x) => {
                // DEBUG - generated from: Rust(RustIdent("Port"))
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        // DEBUG - generated from: Rust(RustIdent("DnsName"))
        self.dns_name.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for SingleHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SingleHostName"))
    }
}

impl DeserializeEmbeddedGroup for SingleHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let port = (|| -> Result<_, DeserializeError> {
            println!("deserializing port");
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    println!("deserializing port");
                    Some(Port::deserialize(raw)?)
                },
                false => {
                    println!("deserializing port");
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("port"))?;
        let dns_name = (|| -> Result<_, DeserializeError> {
            println!("deserializing dns_name");
            Ok(DnsName::deserialize(raw)?)
        })().map_err(|e| e.annotate("dns_name"))?;
        Ok(SingleHostName::new(port, dns_name))
    }
}

impl cbor_event::se::Serialize for MultiHostName {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MultiHostName {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(2))
        serializer.write_unsigned_integer(2u64)?;
        // DEBUG - generated from: Optional(Rust(RustIdent("Port")))
        match &self.port {
            Some(x) => {
                // DEBUG - generated from: Rust(RustIdent("Port"))
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        // DEBUG - generated from: Rust(RustIdent("DnsName"))
        self.dns_name.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MultiHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MultiHostName"))
    }
}

impl DeserializeEmbeddedGroup for MultiHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let port = (|| -> Result<_, DeserializeError> {
            println!("deserializing port");
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    println!("deserializing port");
                    Some(Port::deserialize(raw)?)
                },
                false => {
                    println!("deserializing port");
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("port"))?;
        let dns_name = (|| -> Result<_, DeserializeError> {
            println!("deserializing dns_name");
            Ok(DnsName::deserialize(raw)?)
        })().map_err(|e| e.annotate("dns_name"))?;
        Ok(MultiHostName::new(port, dns_name))
    }
}

impl cbor_event::se::Serialize for RelayEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for RelayEnum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            RelayEnum::SingleHostAddr(x) => x.serialize_as_embedded_group(serializer),
            RelayEnum::SingleHostName(x) => x.serialize_as_embedded_group(serializer),
            RelayEnum::MultiHostName(x) => x.serialize_as_embedded_group(serializer),
        }
    }
}

impl Deserialize for RelayEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("RelayEnum"))
    }
}

impl DeserializeEmbeddedGroup for RelayEnum {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing single_host_addr");
            Ok(SingleHostAddr::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::SingleHostAddr(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing single_host_name");
            Ok(SingleHostName::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::SingleHostName(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing multi_host_name");
            Ok(MultiHostName::deserialize_as_embedded_group(raw, len)?)
        })(raw)
        {
            Ok(variant) => return Ok(RelayEnum::MultiHostName(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        Err(DeserializeError::new("RelayEnum", DeserializeFailure::NoVariantMatched.into()))
    }
}

impl cbor_event::se::Serialize for Relay {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for Relay {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize_as_embedded_group(serializer)
    }
}

impl Deserialize for Relay {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(RelayEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for Ipv4 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Primitive(Bytes)
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for Ipv4 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        println!("deserializing ");
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for Ipv6 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Primitive(Bytes)
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for Ipv6 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        println!("deserializing ");
        Ok(Self(raw.bytes()?))
    }
}

impl cbor_event::se::Serialize for PoolMetadata {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for PoolMetadata {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Rust(RustIdent("Url"))
        self.url.serialize(serializer)?;
        // DEBUG - generated from: Rust(RustIdent("Hash32"))
        self.metadata_hash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for PoolMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolMetadata"))
    }
}

impl DeserializeEmbeddedGroup for PoolMetadata {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let url = (|| -> Result<_, DeserializeError> {
            println!("deserializing url");
            Ok(Url::deserialize(raw)?)
        })().map_err(|e| e.annotate("url"))?;
        let metadata_hash = (|| -> Result<_, DeserializeError> {
            println!("deserializing metadata_hash");
            Ok(MetadataHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("metadata_hash"))?;
        Ok(PoolMetadata::new(url, metadata_hash))
    }
}

impl cbor_event::se::Serialize for StakeRegistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for StakeRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(0))
        serializer.write_unsigned_integer(0u64)?;
        // DEBUG - generated from: Rust(RustIdent("StakeCredential"))
        self.stake_credential.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeRegistration"))
    }
}

impl DeserializeEmbeddedGroup for StakeRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            println!("deserializing stake_credential");
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        Ok(StakeRegistration::new(stake_credential))
    }
}

impl cbor_event::se::Serialize for StakeDeregistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for StakeDeregistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(1))
        serializer.write_unsigned_integer(1u64)?;
        // DEBUG - generated from: Rust(RustIdent("StakeCredential"))
        self.stake_credential.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeDeregistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeDeregistration"))
    }
}

impl DeserializeEmbeddedGroup for StakeDeregistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            println!("deserializing stake_credential");
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        Ok(StakeDeregistration::new(stake_credential))
    }
}

impl cbor_event::se::Serialize for StakeDelegation {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for StakeDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(2))
        serializer.write_unsigned_integer(2u64)?;
        // DEBUG - generated from: Rust(RustIdent("StakeCredential"))
        self.stake_credential.serialize(serializer)?;
        // DEBUG - generated from: Rust(RustIdent("Hash32"))
        self.pool_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for StakeDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeDelegation"))
    }
}

impl DeserializeEmbeddedGroup for StakeDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            println!("deserializing stake_credential");
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        let pool_keyhash = (|| -> Result<_, DeserializeError> {
            println!("deserializing pool_keyhash");
            Ok(PoolKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("pool_keyhash"))?;
        Ok(StakeDelegation::new(stake_credential, pool_keyhash))
    }
}

impl cbor_event::se::Serialize for AddrKeyHashes {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for AddrKeyHashes {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing AddrKeyHashes");
                arr.push(AddrKeyHash::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("AddrKeyHashes"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Relays {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Relays {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing Relays");
                arr.push(Relay::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Relays"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for PoolParams {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for PoolParams {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Rust(RustIdent("Hash32"))
        self.operator.serialize(serializer)?;
        // DEBUG - generated from: Rust(RustIdent("Hash32"))
        self.vrf_keyhash.serialize(serializer)?;
        // DEBUG - generated from: Primitive(U32)
        self.pledge.serialize(serializer)?;
        // DEBUG - generated from: Primitive(U32)
        self.cost.serialize(serializer)?;
        // DEBUG - generated from: Rust(RustIdent("UnitInterval"))
        self.margin.serialize(serializer)?;
        // DEBUG - generated from: Rust(RustIdent("StakeCredential"))
        self.reward_account.serialize(serializer)?;
        // DEBUG - generated from: Array(Rust(RustIdent("Hash28")))
        self.pool_owners.serialize(serializer)?;
        // DEBUG - generated from: Array(Rust(RustIdent("Relay")))
        self.relays.serialize(serializer)?;
        // DEBUG - generated from: Optional(Rust(RustIdent("PoolMetadata")))
        match &self.pool_metadata {
            Some(x) => {
                // DEBUG - generated from: Rust(RustIdent("PoolMetadata"))
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for PoolParams {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolParams"))
    }
}

impl DeserializeEmbeddedGroup for PoolParams {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let operator = (|| -> Result<_, DeserializeError> {
            println!("deserializing operator");
            Ok(PoolKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("operator"))?;
        let vrf_keyhash = (|| -> Result<_, DeserializeError> {
            println!("deserializing vrf_keyhash");
            Ok(VrfKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("vrf_keyhash"))?;
        let pledge = (|| -> Result<_, DeserializeError> {
            println!("deserializing pledge");
            Ok(Coin::deserialize(raw)?)
        })().map_err(|e| e.annotate("pledge"))?;
        let cost = (|| -> Result<_, DeserializeError> {
            println!("deserializing cost");
            Ok(Coin::deserialize(raw)?)
        })().map_err(|e| e.annotate("cost"))?;
        let margin = (|| -> Result<_, DeserializeError> {
            println!("deserializing margin");
            Ok(UnitInterval::deserialize(raw)?)
        })().map_err(|e| e.annotate("margin"))?;
        let reward_account = (|| -> Result<_, DeserializeError> {
            println!("deserializing reward_account");
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("reward_account"))?;
        let pool_owners = (|| -> Result<_, DeserializeError> {
            println!("deserializing pool_owners");
            Ok(AddrKeyHashes::deserialize(raw)?)
        })().map_err(|e| e.annotate("pool_owners"))?;
        let relays = (|| -> Result<_, DeserializeError> {
            println!("deserializing relays");
            Ok(Relays::deserialize(raw)?)
        })().map_err(|e| e.annotate("relays"))?;
        let pool_metadata = (|| -> Result<_, DeserializeError> {
            println!("deserializing pool_metadata");
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    println!("deserializing pool_metadata");
                    Some(PoolMetadata::deserialize(raw)?)
                },
                false => {
                    println!("deserializing pool_metadata");
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("pool_metadata"))?;
        Ok(PoolParams::new(operator, vrf_keyhash, pledge, cost, margin, reward_account, pool_owners, relays, pool_metadata))
    }
}

impl cbor_event::se::Serialize for PoolRegistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for PoolRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(3))
        serializer.write_unsigned_integer(3u64)?;
        // DEBUG - generated from: Rust(RustIdent("PoolParams"))
        self.pool_params.serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for PoolRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolRegistration"))
    }
}

impl DeserializeEmbeddedGroup for PoolRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 3 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(3) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let pool_params = (|| -> Result<_, DeserializeError> {
            println!("deserializing pool_params");
            Ok(PoolParams::deserialize_as_embedded_group(raw, len)?)
        })().map_err(|e| e.annotate("pool_params"))?;
        Ok(PoolRegistration::new(pool_params))
    }
}

impl cbor_event::se::Serialize for PoolRetirement {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for PoolRetirement {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(4))
        serializer.write_unsigned_integer(4u64)?;
        // DEBUG - generated from: Rust(RustIdent("Hash32"))
        self.pool_keyhash.serialize(serializer)?;
        // DEBUG - generated from: Primitive(U32)
        self.epoch.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for PoolRetirement {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolRetirement"))
    }
}

impl DeserializeEmbeddedGroup for PoolRetirement {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 4 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(4) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let pool_keyhash = (|| -> Result<_, DeserializeError> {
            println!("deserializing pool_keyhash");
            Ok(PoolKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("pool_keyhash"))?;
        let epoch = (|| -> Result<_, DeserializeError> {
            println!("deserializing epoch");
            Ok(u32::deserialize(raw)?)
        })().map_err(|e| e.annotate("epoch"))?;
        Ok(PoolRetirement::new(pool_keyhash, epoch))
    }
}

impl cbor_event::se::Serialize for GenesisKeyDelegation {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for GenesisKeyDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(5))
        serializer.write_unsigned_integer(5u64)?;
        // DEBUG - generated from: Rust(RustIdent("Hash32"))
        self.genesishash.serialize(serializer)?;
        // DEBUG - generated from: Rust(RustIdent("Hash32"))
        self.genesis_delegate_hash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for GenesisKeyDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("GenesisKeyDelegation"))
    }
}

impl DeserializeEmbeddedGroup for GenesisKeyDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 5 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(5) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let genesishash = (|| -> Result<_, DeserializeError> {
            println!("deserializing genesishash");
            Ok(GenesisHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("genesishash"))?;
        let genesis_delegate_hash = (|| -> Result<_, DeserializeError> {
            println!("deserializing genesis_delegate_hash");
            Ok(GenesisDelegateHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("genesis_delegate_hash"))?;
        Ok(GenesisKeyDelegation::new(genesishash, genesis_delegate_hash))
    }
}

impl cbor_event::se::Serialize for MoveInstantaneousRewardsCert {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MoveInstantaneousRewardsCert {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        // DEBUG - generated from: Fixed(Uint(6))
        serializer.write_unsigned_integer(6u64)?;
        // DEBUG - generated from: Rust(RustIdent("MoveInstantaneousReward"))
        self.move_instantaneous_reward.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for MoveInstantaneousRewardsCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MoveInstantaneousRewardsCert"))
    }
}

impl DeserializeEmbeddedGroup for MoveInstantaneousRewardsCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            println!("deserializing index_0");
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 6 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(6) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let move_instantaneous_reward = (|| -> Result<_, DeserializeError> {
            println!("deserializing move_instantaneous_reward");
            Ok(MoveInstantaneousReward::deserialize(raw)?)
        })().map_err(|e| e.annotate("move_instantaneous_reward"))?;
        Ok(MoveInstantaneousRewardsCert::new(move_instantaneous_reward))
    }
}

impl cbor_event::se::Serialize for CertificateEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for CertificateEnum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            CertificateEnum::StakeRegistration(x) => {
                // DEBUG - generated from: Rust(RustIdent("StakeRegistration"))
                x.serialize(serializer)
            },
            CertificateEnum::StakeDeregistration(x) => {
                // DEBUG - generated from: Rust(RustIdent("StakeDeregistration"))
                x.serialize(serializer)
            },
            CertificateEnum::StakeDelegation(x) => {
                // DEBUG - generated from: Rust(RustIdent("StakeDelegation"))
                x.serialize(serializer)
            },
            CertificateEnum::PoolRegistration(x) => {
                // DEBUG - generated from: Rust(RustIdent("PoolRegistration"))
                x.serialize(serializer)
            },
            CertificateEnum::PoolRetirement(x) => {
                // DEBUG - generated from: Rust(RustIdent("PoolRetirement"))
                x.serialize(serializer)
            },
            CertificateEnum::GenesisKeyDelegation(x) => {
                // DEBUG - generated from: Rust(RustIdent("GenesisKeyDelegation"))
                x.serialize(serializer)
            },
            CertificateEnum::MoveInstantaneousRewardsCert(x) => {
                // DEBUG - generated from: Rust(RustIdent("MoveInstantaneousRewardsCert"))
                x.serialize(serializer)
            },
        }
    }
}

impl Deserialize for CertificateEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("CertificateEnum"))
    }
}

impl DeserializeEmbeddedGroup for CertificateEnum {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing stake_registration");
            Ok(StakeRegistration::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::StakeRegistration(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing stake_deregistration");
            Ok(StakeDeregistration::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::StakeDeregistration(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing stake_delegation");
            Ok(StakeDelegation::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::StakeDelegation(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing pool_registration");
            Ok(PoolRegistration::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::PoolRegistration(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing pool_retirement");
            Ok(PoolRetirement::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::PoolRetirement(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing genesis_key_delegation");
            Ok(GenesisKeyDelegation::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::GenesisKeyDelegation(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            println!("deserializing move_instantaneous_rewards_cert");
            Ok(MoveInstantaneousRewardsCert::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(CertificateEnum::MoveInstantaneousRewardsCert(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        Err(DeserializeError::new("CertificateEnum", DeserializeFailure::NoVariantMatched.into()))
    }
}

impl cbor_event::se::Serialize for Certificate {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for Certificate {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize_as_embedded_group(serializer)
    }
}

impl Deserialize for Certificate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(CertificateEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for TransactionInputs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionInputs {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing TransactionInputs");
                arr.push(TransactionInput::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionInputs"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionOutputs {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionOutputs {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing TransactionOutputs");
                arr.push(TransactionOutput::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionOutputs"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for Certificates {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Certificates {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing Certificates");
                arr.push(Certificate::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Certificates"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionBody {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionBody {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0)?;
        // DEBUG - generated from: Array(Rust(RustIdent("TransactionInput")))
        self.inputs.serialize(serializer)?;
        serializer.write_unsigned_integer(1)?;
        // DEBUG - generated from: Array(Rust(RustIdent("TransactionOutput")))
        self.outputs.serialize(serializer)?;
        serializer.write_unsigned_integer(2)?;
        // DEBUG - generated from: Primitive(U32)
        self.fee.serialize(serializer)?;
        serializer.write_unsigned_integer(3)?;
        // DEBUG - generated from: Primitive(U32)
        self.ttl.serialize(serializer)?;
        if let Some(field) = &self.certs {
            serializer.write_unsigned_integer(4)?;
            // DEBUG - generated from: Array(Rust(RustIdent("Certificate")))
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.withdrawals {
            serializer.write_unsigned_integer(5)?;
            // DEBUG - generated from: Rust(RustIdent("Withdrawals"))
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.metadata_hash {
            serializer.write_unsigned_integer(7)?;
            // DEBUG - generated from: Rust(RustIdent("MetadataHash"))
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            Self::deserialize_as_embedded_group(raw, len)
        })().map_err(|e| e.annotate("TransactionBody"))
    }
}

impl DeserializeEmbeddedGroup for TransactionBody {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let mut inputs = None;
        let mut outputs = None;
        let mut fee = None;
        let mut ttl = None;
        let mut certs = None;
        let mut withdrawals = None;
        let mut metadata_hash = None;
        let mut read = 0;
        while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
            match raw.cbor_type()? {
                CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                    0 =>  {
                        if inputs.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                        }
                        inputs = Some((|| -> Result<_, DeserializeError> {
                            println!("deserializing inputs");
                            Ok(TransactionInputs::deserialize(raw)?)
                        })().map_err(|e| e.annotate("inputs"))?);
                    },
                    1 =>  {
                        if outputs.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                        }
                        outputs = Some((|| -> Result<_, DeserializeError> {
                            println!("deserializing outputs");
                            Ok(TransactionOutputs::deserialize(raw)?)
                        })().map_err(|e| e.annotate("outputs"))?);
                    },
                    2 =>  {
                        if fee.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                        }
                        fee = Some((|| -> Result<_, DeserializeError> {
                            println!("deserializing fee");
                            Ok(Coin::deserialize(raw)?)
                        })().map_err(|e| e.annotate("fee"))?);
                    },
                    3 =>  {
                        if ttl.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                        }
                        ttl = Some((|| -> Result<_, DeserializeError> {
                            println!("deserializing ttl");
                            Ok(u32::deserialize(raw)?)
                        })().map_err(|e| e.annotate("ttl"))?);
                    },
                    4 =>  {
                        if certs.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                        }
                        certs = Some((|| -> Result<_, DeserializeError> {
                            println!("deserializing certs");
                            Ok(Certificates::deserialize(raw)?)
                        })().map_err(|e| e.annotate("certs"))?);
                    },
                    5 =>  {
                        if withdrawals.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                        }
                        withdrawals = Some((|| -> Result<_, DeserializeError> {
                            println!("deserializing withdrawals");
                            Ok(Withdrawals::deserialize(raw)?)
                        })().map_err(|e| e.annotate("withdrawals"))?);
                    },
                    7 =>  {
                        if withdrawals.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                        }
                        metadata_hash = Some((|| -> Result<_, DeserializeError> {
                            Ok(MetadataHash::deserialize(raw)?)
                        })().map_err(|e| e.annotate("metadata_hash"))?);
                    },
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                },
                CBORType::Text => match raw.text()?.as_str() {
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                },
                CBORType::Special => match len {
                    cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                    cbor_event::Len::Indefinite => break,
                },
                other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
            }
            read += 1;
        }
        let inputs = match inputs {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(0)).into()),
        };
        let outputs = match outputs {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
        };
        let fee = match fee {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(2)).into()),
        };
        let ttl = match ttl {
            Some(x) => x,
            None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(3)).into()),
        };
        Ok(Self {
            inputs,
            outputs,
            fee,
            ttl,
            certs,
            withdrawals,
            metadata_hash,
        })
    }
}

impl cbor_event::se::Serialize for Vkeywitnesss {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Vkeywitnesss {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                println!("deserializing Vkeywitnesss");
                arr.push(Vkeywitness::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("Vkeywitnesss"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionWitnessSet {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionWitnessSet {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        if let Some(field) = &self.vkeys {
            serializer.write_unsigned_integer(0)?;
            // DEBUG - generated from: Array(Rust(RustIdent("Vkeywitness")))
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.scripts {
            serializer.write_unsigned_integer(1)?;
            // DEBUG - generated from: Array(Rust(RustIdent("MultisigScript")))
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionWitnessSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            Self::deserialize_as_embedded_group(raw, len)
        })().map_err(|e| e.annotate("TransactionWitnessSet"))
    }
}

impl DeserializeEmbeddedGroup for TransactionWitnessSet {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let mut vkeys = None;
        let mut scripts = None;
        let mut read = 0;
        while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
            match raw.cbor_type()? {
                CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                    0 =>  {
                        if vkeys.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                        }
                        vkeys = Some((|| -> Result<_, DeserializeError> {
                            println!("deserializing vkeys");
                            Ok(Vkeywitnesss::deserialize(raw)?)
                        })().map_err(|e| e.annotate("vkeys"))?);
                    },
                    1 =>  {
                        if scripts.is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                        }
                        scripts = Some((|| -> Result<_, DeserializeError> {
                            println!("deserializing scripts");
                            Ok(MultisigScripts::deserialize(raw)?)
                        })().map_err(|e| e.annotate("scripts"))?);
                    },
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                },
                CBORType::Text => match raw.text()?.as_str() {
                    unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                },
                CBORType::Special => match len {
                    cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                    cbor_event::Len::Indefinite => break,
                },
                other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
            }
            read += 1;
        }
        Ok(Self {
            vkeys,
            scripts,
        })
    }
}


impl cbor_event::se::Serialize for MapTransactionMetadatumToTransactionMetadatum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for MapTransactionMetadatumToTransactionMetadatum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MapTransactionMetadatumToTransactionMetadatum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = TransactionMetadatum::deserialize(raw)?;
                let value = TransactionMetadatum::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("MapTransactionMetadatumToTransactionMetadatum"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for TransactionMetadatums {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionMetadatums {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len { cbor_event::Len::Len(n) => arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                arr.push(TransactionMetadatum::deserialize(raw)?);
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionMetadatums"))?;
        Ok(Self(arr))
    }
}

impl cbor_event::se::Serialize for TransactionMetadatumEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for TransactionMetadatumEnum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            TransactionMetadatumEnum::MapTransactionMetadatumToTransactionMetadatum(x) => {
                x.serialize(serializer)
            },
            TransactionMetadatumEnum::ArrTransactionMetadatum(x) => {
                x.serialize(serializer)
            },
            TransactionMetadatumEnum::Int(x) => {
                x.serialize(serializer)
            },
            TransactionMetadatumEnum::Bytes(x) => {
                serializer.write_bytes(&x)
            },
            TransactionMetadatumEnum::Text(x) => {
                serializer.write_text(&x)
            },
        }
    }
}

impl Deserialize for TransactionMetadatumEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(MapTransactionMetadatumToTransactionMetadatum::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::MapTransactionMetadatumToTransactionMetadatum(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(TransactionMetadatums::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::ArrTransactionMetadatum(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(Int::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::Int(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(raw.bytes()?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::Bytes(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
            Ok(String::deserialize(raw)?)
        })(raw)
        {
            Ok(variant) => return Ok(TransactionMetadatumEnum::Text(variant)),
            Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
        };
        Err(DeserializeError::new("TransactionMetadatumEnum", DeserializeFailure::NoVariantMatched.into()))
    }
}

impl cbor_event::se::Serialize for TransactionMetadatum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for TransactionMetadatum {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize_as_embedded_group(serializer)
    }
}

impl Deserialize for TransactionMetadatum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(TransactionMetadatumEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for TransactionMetadata {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for TransactionMetadata {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = std::collections::BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len { cbor_event::Len::Len(n) => table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                let key = u32::deserialize(raw)?;
                let value = TransactionMetadatum::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Uint(key.into())).into());
                }
            }
            Ok(())
        })().map_err(|e| e.annotate("TransactionMetadata"))?;
        Ok(Self(table))
    }
}

impl cbor_event::se::Serialize for Transaction {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Indefinite)?;
        self.serialize_as_embedded_group(serializer)?;
        serializer.write_special(CBORSpecial::Break)
    }
}

impl SerializeEmbeddedGroup for Transaction {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.body.serialize(serializer)?;
        self.witness_set.serialize(serializer)?;
        match &self.metadata {
            Some(x) => {
                x.serialize(serializer)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for Transaction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Transaction"))
    }
}

impl DeserializeEmbeddedGroup for Transaction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        let body = (|| -> Result<_, DeserializeError> {
            Ok(TransactionBody::deserialize(raw)?)
        })().map_err(|e| e.annotate("body"))?;
        let witness_set = (|| -> Result<_, DeserializeError> {
            Ok(TransactionWitnessSet::deserialize(raw)?)
        })().map_err(|e| e.annotate("witness_set"))?;
        let metadata = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(TransactionMetadata::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("metadata"))?;
        Ok(Transaction::new(body, witness_set, metadata))
    }
}